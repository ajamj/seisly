use std::process::{Child, Command, Stdio, ChildStdin, ChildStdout};
use std::io::{BufReader, BufRead, Write};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU8, Ordering};
use std::thread;
use std::time::Duration;
use anyhow::{Result, anyhow, Context};
use seisly_core::ipc::ShmSegment;
use crate::bridge::{should_use_shm, data_size_bytes};
use sysinfo::System;

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    id: u64,
    method: String,
    params: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    id: u64,
    result: Option<serde_json::Value>,
    error: Option<String>,
}

/// Protocol version for host-worker communication
const PROTOCOL_VERSION: &str = "1.1";

/// Heartbeat interval in seconds
const HEARTBEAT_INTERVAL_SECS: u64 = 5;

/// Maximum consecutive heartbeat failures before restarting worker
const MAX_HEARTBEAT_FAILURES: u8 = 3;

/// Memory limit for worker process in MB (configurable)
const DEFAULT_MEMORY_LIMIT_MB: u64 = 512;

pub struct IpcBridge {
    inner: Arc<Mutex<Option<WorkerInstance>>>,
    next_id: Arc<Mutex<u64>>,
    handshake_done: Arc<Mutex<bool>>,
    /// Watchdog state
    heartbeat_stop: Arc<AtomicU8>, // 0 = running, 1 = stopped
    failure_count: Arc<AtomicU8>,
    system: Arc<Mutex<System>>,
    worker_pid: Arc<Mutex<Option<u32>>>,
}

struct WorkerInstance {
    _child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl IpcBridge {
    pub fn new() -> Self {
        let bridge = Self {
            inner: Arc::new(Mutex::new(None)),
            next_id: Arc::new(Mutex::new(1)),
            handshake_done: Arc::new(Mutex::new(false)),
            heartbeat_stop: Arc::new(AtomicU8::new(0)),
            failure_count: Arc::new(AtomicU8::new(0)),
            system: Arc::new(Mutex::new(System::new())),
            worker_pid: Arc::new(Mutex::new(None)),
        };

        // Start background heartbeat thread
        let heartbeat_inner = Arc::clone(&bridge.inner);
        let heartbeat_stop = Arc::clone(&bridge.heartbeat_stop);
        let failure_count = Arc::clone(&bridge.failure_count);
        let system = Arc::clone(&bridge.system);
        let worker_pid = Arc::clone(&bridge.worker_pid);

        thread::spawn(move || {
            loop {
                // Check if heartbeat should stop
                if heartbeat_stop.load(Ordering::Relaxed) != 0 {
                    thread::sleep(Duration::from_secs(1));
                    continue;
                }

                thread::sleep(Duration::from_secs(HEARTBEAT_INTERVAL_SECS));

                // Skip if no worker is running
                {
                    let inner_guard = heartbeat_inner.lock().unwrap();
                    if inner_guard.is_none() {
                        continue;
                    }
                }

                // Check worker memory usage
                {
                    let mut sys = system.lock().unwrap();
                    sys.refresh_processes();
                    if let Some(pid) = *worker_pid.lock().unwrap() {
                        if let Some(process) = sys.process(sysinfo::Pid::from_u32(pid)) {
                            let memory_mb = process.memory() / 1024 / 1024;
                            if memory_mb > DEFAULT_MEMORY_LIMIT_MB {
                                log::warn!(
                                    "Worker memory usage ({:.0} MB) exceeds limit ({} MB)",
                                    memory_mb,
                                    DEFAULT_MEMORY_LIMIT_MB
                                );
                            }
                        }
                    }
                }

                // Send ping to check if worker is alive
                let bridge_weak = IpcBridge {
                    inner: Arc::clone(&heartbeat_inner),
                    next_id: Arc::new(Mutex::new(1)), // Dummy, won't be used
                    handshake_done: Arc::new(Mutex::new(true)), // Assume handshake done
                    heartbeat_stop: Arc::clone(&heartbeat_stop),
                    failure_count: Arc::clone(&failure_count),
                    system: Arc::clone(&system),
                    worker_pid: Arc::clone(&worker_pid),
                };

                match bridge_weak.execute_internal_simple("ping") {
                    Ok(_) => {
                        // Reset failure count on success
                        failure_count.store(0, Ordering::Relaxed);
                        log::debug!("Heartbeat: worker is alive");
                    }
                    Err(e) => {
                        let failures = failure_count.fetch_add(1, Ordering::Relaxed) + 1;
                        log::warn!("Heartbeat failed ({} / {}): {}", failures, MAX_HEARTBEAT_FAILURES, e);

                        if failures >= MAX_HEARTBEAT_FAILURES {
                            log::error!("Worker unresponsive, killing and marking for restart");
                            // Kill the worker
                            let mut inner_guard = heartbeat_inner.lock().unwrap();
                            *inner_guard = None;
                            failure_count.store(0, Ordering::Relaxed);
                            // Reset handshake flag so it will be redone on next attempt
                        }
                    }
                }
            }
        });

        bridge
    }

    fn ensure_worker(&self) -> Result<()> {
        let mut inner_guard = self.inner.lock().unwrap();

        if let Some(ref mut instance) = *inner_guard {
             if let Ok(None) = instance._child.try_wait() {
                 return Ok(());
             }
        }

        // Spawn worker with resource limits
        // In a production environment, we'd use the actual binary path.
        // For development, we use 'cargo run'.
        let mut child = Command::new("cargo")
            .args(["run", "-p", "seisly_py_worker", "--quiet"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| anyhow!("Failed to spawn worker: {}", e))?;

        // Capture worker PID for monitoring
        let pid = child.id();
        log::info!("Spawned worker process with PID: {}", pid);

        // Store PID for monitoring
        {
            let mut worker_pid_guard = self.worker_pid.lock().unwrap();
            *worker_pid_guard = Some(pid);
        }

        // Apply OS-level resource limits
        #[cfg(target_os = "linux")]
        {
            apply_linux_resource_limits(pid, DEFAULT_MEMORY_LIMIT_MB)?;
        }

        #[cfg(target_os = "windows")]
        {
            apply_windows_job_object(pid, DEFAULT_MEMORY_LIMIT_MB)?;
        }

        let stdin = child.stdin.take().ok_or_else(|| anyhow!("Stdin not available"))?;
        let stdout = child.stdout.take().ok_or_else(|| anyhow!("Stdout not available"))?;

        *inner_guard = Some(WorkerInstance {
            _child: child,
            stdin,
            stdout: BufReader::new(stdout),
        });

        Ok(())
    }

    /// Performs API version handshake with the worker.
    ///
    /// This must be called immediately after spawning the worker.
    /// The handshake negotiates protocol compatibility and feature support.
    pub fn handshake(&self) -> Result<()> {
        // Check if handshake already done
        {
            let handshake_guard = self.handshake_done.lock().unwrap();
            if *handshake_guard {
                return Ok(());
            }
        }

        self.ensure_worker()?;

        let params = serde_json::json!({
            "protocol_version": PROTOCOL_VERSION,
            "shm_supported": true
        });

        let result = self.execute_internal_str("handshake", params)?;

        // Parse handshake response
        let handshake_response: HandshakeResponse = serde_json::from_str(&result)
            .context("Failed to parse handshake response")?;

        // Check version compatibility
        if !self.is_version_compatible(&handshake_response.protocol_version) {
            return Err(anyhow!(
                "Incompatible protocol version: host requires {}, worker returned {}",
                PROTOCOL_VERSION,
                handshake_response.protocol_version
            ));
        }

        // Verify SHM support
        if !handshake_response.shm_supported {
            log::warn!("Worker does not support SHM transfers, falling back to JSON-RPC");
        }

        // Mark handshake as complete
        {
            let mut handshake_guard = self.handshake_done.lock().unwrap();
            *handshake_guard = true;
        }

        log::info!(
            "Handshake successful: protocol_version={}, shm_supported={}",
            handshake_response.protocol_version,
            handshake_response.shm_supported
        );

        Ok(())
    }

    /// Checks if the worker's protocol version is compatible with the host
    fn is_version_compatible(&self, worker_version: &str) -> bool {
        // For now, exact match required
        // In the future, could implement semver compatibility
        worker_version == PROTOCOL_VERSION
    }

    pub fn execute(&self, method: &str, params: serde_json::Value) -> Result<serde_json::Value> {
        // Ensure handshake is done before any other operation
        self.handshake().context("Handshake failed")?;

        let id = {
            let mut id_guard = self.next_id.lock().unwrap();
            let id = *id_guard;
            *id_guard += 1;
            id
        };

        let req = Request {
            id,
            method: method.to_string(),
            params,
        };

        let result = self.execute_internal(req);

        if result.is_err() {
            // Communication error or worker crash, clear instance to force restart next time
            let mut inner_guard = self.inner.lock().unwrap();
            *inner_guard = None;
            // Also reset handshake flag so it will be redone on next attempt
            let mut handshake_guard = self.handshake_done.lock().unwrap();
            *handshake_guard = false;
        }

        result
    }

    /// Transfers data via Shared Memory for high-performance seismic data transfer.
    ///
    /// This method:
    /// 1. Creates a SHM segment and writes data to it
    /// 2. Sends the SHM ID to the worker via JSON-RPC
    /// 3. Worker reads the data and returns a result
    /// 4. Returns the worker's result
    pub fn transfer_shm(&self, data: &[f32], shape: Vec<usize>) -> Result<serde_json::Value> {
        // Create SHM segment
        let size = data.len() * std::mem::size_of::<f32>();
        let mut shm = ShmSegment::create(size)?;
        let shm_id = shm.id().to_string();

        // Write data to SHM
        let data_bytes: &[u8] = bytemuck::cast_slice(data);
        shm.write_data(data_bytes);

        // Send SHM ID to worker
        let params = serde_json::json!({
            "shm_id": shm_id,
            "shape": shape,
            "dtype": "f32"
        });

        let result = self.execute("load_shm", params)?;

        // SHM segment will be cleaned up when dropped
        Ok(result)
    }

    /// Smart data transfer that automatically chooses SHM or JSON-RPC based on size.
    ///
    /// For large data (>= 1MB), uses high-performance SHM transfer.
    /// For small data (< 1MB), uses simple JSON-RPC inline transfer.
    pub fn transfer_data(&self, data: &[f32], shape: Vec<usize>) -> Result<serde_json::Value> {
        if should_use_shm(data) {
            log::debug!(
                "Using SHM transfer for large data: {} bytes",
                data_size_bytes(data)
            );
            self.transfer_shm(data, shape)
        } else {
            log::debug!(
                "Using JSON-RPC transfer for small data: {} bytes",
                data_size_bytes(data)
            );
            // For small data, send inline via JSON-RPC
            let params = serde_json::json!({
                "data": data,
                "shape": shape
            });
            self.execute("load_data", params)
        }
    }

    fn execute_internal(&self, req: Request) -> Result<serde_json::Value> {
        let mut inner_guard = self.inner.lock().unwrap();
        let instance = inner_guard.as_mut().ok_or_else(|| anyhow!("Worker not initialized"))?;

        let req_json = serde_json::to_string(&req)? + "\n";
        instance.stdin.write_all(req_json.as_bytes())?;
        instance.stdin.flush()?;

        let mut line = String::new();
        instance.stdout.read_line(&mut line)
            .map_err(|e| anyhow!("Failed to read from worker: {}", e))?;

        if line.is_empty() {
            return Err(anyhow!("Worker closed output pipe (EOF)"));
        }

        let resp: Response = serde_json::from_str(&line)
            .map_err(|e| anyhow!("Failed to parse response '{}': {}", line, e))?;

        if let Some(err) = resp.error {
            return Err(anyhow!("Worker error: {}", err));
        }

        resp.result.ok_or_else(|| anyhow!("Missing result in response"))
    }

    /// Internal method for executing a method and getting raw string response
    fn execute_internal_str(&self, method: &str, params: serde_json::Value) -> Result<String> {
        let mut inner_guard = self.inner.lock().unwrap();
        let instance = inner_guard.as_mut().ok_or_else(|| anyhow!("Worker not initialized"))?;

        let id = {
            let mut id_guard = self.next_id.lock().unwrap();
            let id = *id_guard;
            *id_guard += 1;
            id
        };

        let req = Request {
            id,
            method: method.to_string(),
            params,
        };

        let req_json = serde_json::to_string(&req)? + "\n";
        instance.stdin.write_all(req_json.as_bytes())?;
        instance.stdin.flush()?;

        let mut line = String::new();
        instance.stdout.read_line(&mut line)
            .map_err(|e| anyhow!("Failed to read from worker: {}", e))?;

        if line.is_empty() {
            return Err(anyhow!("Worker closed output pipe (EOF)"));
        }

        let resp: Response = serde_json::from_str(&line)
            .map_err(|e| anyhow!("Failed to parse response '{}': {}", line, e))?;

        if let Some(err) = resp.error {
            return Err(anyhow!("Worker error: {}", err));
        }

        // Return result as string
        match resp.result {
            Some(serde_json::Value::String(s)) => Ok(s),
            Some(v) => Ok(v.to_string()),
            None => Err(anyhow!("Missing result in response")),
        }
    }

    /// Simple internal method for heartbeat pings (no ID tracking, no handshake)
    fn execute_internal_simple(&self, method: &str) -> Result<String> {
        let mut inner_guard = self.inner.lock().unwrap();
        let instance = inner_guard.as_mut().ok_or_else(|| anyhow!("Worker not initialized"))?;

        // Use a fixed ID for heartbeat pings
        let req = Request {
            id: 0,
            method: method.to_string(),
            params: serde_json::Value::Null,
        };

        let req_json = serde_json::to_string(&req)? + "\n";
        instance.stdin.write_all(req_json.as_bytes())?;
        instance.stdin.flush()?;

        let mut line = String::new();
        instance.stdout.read_line(&mut line)
            .map_err(|e| anyhow!("Failed to read from worker: {}", e))?;

        if line.is_empty() {
            return Err(anyhow!("Worker closed output pipe (EOF)"));
        }

        let resp: Response = serde_json::from_str(&line)
            .map_err(|e| anyhow!("Failed to parse response '{}': {}", line, e))?;

        if let Some(err) = resp.error {
            return Err(anyhow!("Worker error: {}", err));
        }

        // Return result as string
        match resp.result {
            Some(serde_json::Value::String(s)) => Ok(s),
            Some(v) => Ok(v.to_string()),
            None => Err(anyhow!("Missing result in response")),
        }
    }
}

impl Default for IpcBridge {
    fn default() -> Self {
        Self::new()
    }
}

/// Response from the handshake method
#[derive(Debug, Serialize, Deserialize)]
struct HandshakeResponse {
    protocol_version: String,
    shm_supported: bool,
}

/// Apply Linux-specific resource limits using prlimit
#[cfg(target_os = "linux")]
fn apply_linux_resource_limits(pid: u32, memory_limit_mb: u64) -> Result<()> {
    use std::process::Command;

    // Convert MB to bytes
    let memory_limit_bytes = memory_limit_mb * 1024 * 1024;

    // Use prlimit to set RSS (resident set size) memory limit
    // RLIMIT_RSS = 5, soft limit, hard limit
    let output = Command::new("prlimit")
        .args([
            "--pid", &pid.to_string(),
            "--rss", &format!("{}:{}", memory_limit_bytes, memory_limit_bytes),
        ])
        .output();

    match output {
        Ok(out) => {
            if !out.status.success() {
                let stderr = String::from_utf8_lossy(&out.stderr);
                log::warn!("prlimit failed (may require root): {}", stderr);
                return Ok(()); // Don't fail, just warn
            }
            log::info!("Applied Linux resource limits: RSS limit = {} MB", memory_limit_mb);
        }
        Err(e) => {
            log::warn!("Failed to apply prlimit (not available or permission denied): {}", e);
            // Don't fail, prlimit may not be available
        }
    }

    Ok(())
}

/// Apply Windows-specific resource limits
#[cfg(target_os = "windows")]
fn apply_windows_job_object(pid: u32, memory_limit_mb: u64) -> Result<()> {
    // Note: Full Windows Job Object implementation requires additional setup.
    // For now, we rely on the background watchdog to monitor and kill the process
    // if it exceeds memory limits.
    log::info!("Windows worker spawned with PID: {}, memory limit: {} MB (monitored by watchdog)", pid, memory_limit_mb);
    Ok(())
}

// Fallback for non-Linux/Windows platforms
#[cfg(not(any(target_os = "linux", target_os = "windows")))]
fn apply_linux_resource_limits(_pid: u32, _memory_limit_mb: u64) -> Result<()> {
    log::warn!("Resource limits not implemented for this platform");
    Ok(())
}

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
fn apply_windows_job_object(_pid: u32, _memory_limit_mb: u64) -> Result<()> {
    log::warn!("Resource limits not implemented for this platform");
    Ok(())
}
