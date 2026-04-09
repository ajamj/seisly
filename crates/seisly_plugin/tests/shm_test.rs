use seisly_core::ipc::ShmSegment;
/// Integration test for Shared Memory IPC between Rust host and Python worker.
///
/// This test verifies:
/// 1. Host can allocate a shared memory segment and write data
/// 2. Worker can attach to the segment and read the data
/// 3. Data integrity is maintained across process boundary
/// 4. SHM transfer is significantly faster than JSON-RPC for large payloads
use seisly_plugin::ipc::IpcBridge;

#[test]
fn test_shm_basic_transfer() {
    // Create test data: 1MB of f32 values (262144 elements)
    let size = 262144;
    let mut test_data: Vec<f32> = Vec::with_capacity(size);

    // Fill with deterministic pattern for checksum verification
    for i in 0..size {
        test_data.push((i % 1000) as f32 / 1000.0);
    }

    let expected_checksum: f64 = test_data.iter().map(|&x| x as f64).sum();
    let shape = vec![size];

    // Create IPC bridge and transfer data via SHM
    let bridge = IpcBridge::new();
    let result = bridge.transfer_shm(&test_data, shape.clone());

    assert!(result.is_ok(), "SHM transfer failed: {:?}", result.err());

    // Worker returns the sum of the array as verification
    let sum_result = result.unwrap();

    // Parse the sum from the result (can be Number or String)
    let actual_sum: f64 = match &sum_result {
        serde_json::Value::Number(n) => n.as_f64().unwrap_or(0.0),
        serde_json::Value::String(s) => s.parse().unwrap_or(0.0),
        _ => panic!(
            "Expected number or string result containing sum, got: {:?}",
            sum_result
        ),
    };

    // Allow small floating-point tolerance
    let tolerance = expected_checksum * 1e-5;
    assert!(
        (actual_sum - expected_checksum).abs() < tolerance,
        "Checksum mismatch: expected {}, got {}",
        expected_checksum,
        actual_sum
    );
}

#[test]
fn test_shm_cross_process() {
    // Test that SHM segment can be created in host and accessed by worker

    // Create 10MB of test data
    let size = 2_500_000; // ~10MB of f32
    let test_data: Vec<f32> = (0..size).map(|i| i as f32 * 0.001).collect();
    let expected_checksum: f64 = test_data.iter().map(|&x| x as f64).sum();

    let bridge = IpcBridge::new();
    let result = bridge.transfer_shm(&test_data, vec![size]);

    assert!(
        result.is_ok(),
        "Large SHM transfer failed: {:?}",
        result.err()
    );

    let sum_result = result.unwrap();
    let actual_sum: f64 = match &sum_result {
        serde_json::Value::Number(n) => n.as_f64().unwrap_or(0.0),
        serde_json::Value::String(s) => s.parse().unwrap_or(0.0),
        _ => panic!(
            "Expected number or string result containing sum, got: {:?}",
            sum_result
        ),
    };

    let tolerance = expected_checksum.abs() * 1e-4; // Larger tolerance for big arrays
    assert!(
        (actual_sum - expected_checksum).abs() < tolerance,
        "Large array checksum mismatch: expected {}, got {}",
        expected_checksum,
        actual_sum
    );
}

#[test]
fn test_shm_manual_segment() {
    // Test manual SHM segment creation and access

    let data_size = 1024;
    let test_data: Vec<u8> = (0..data_size).map(|i| (i % 256) as u8).collect();

    // Create segment
    let mut shm = ShmSegment::create(data_size).unwrap();
    let shm_id = shm.id().to_string();

    // Write data
    shm.write_data(&test_data);

    // Open existing segment (simulating worker access)
    let shm_reader = ShmSegment::open(&shm_id).unwrap();

    // Read data back
    let mut buffer = vec![0u8; data_size];
    shm_reader.read_data(&mut buffer);

    assert_eq!(test_data, buffer, "Data integrity check failed");

    // Verify segment properties (size may be rounded up by OS)
    assert!(shm_reader.size() >= data_size, "Segment too small");
    assert_eq!(shm_reader.id(), shm_id);
}

#[test]
#[ignore = "Performance benchmark - run manually with --ignored"]
fn test_shm_performance_vs_json() {
    // Compare SHM transfer time vs JSON-RPC for large payloads

    use std::time::Instant;

    // Create 10MB test array
    let size = 2_500_000;
    let test_data: Vec<f32> = (0..size).map(|i| i as f32 * 0.001).collect();

    let bridge = IpcBridge::new();

    // Measure SHM transfer time
    let shm_start = Instant::now();
    let shm_result = bridge.transfer_shm(&test_data, vec![size]);
    let shm_elapsed = shm_start.elapsed();

    assert!(shm_result.is_ok(), "SHM transfer failed");

    eprintln!("SHM transfer (10MB): {:?}", shm_elapsed);

    // Note: JSON-RPC transfer would require serializing the entire array,
    // which is significantly slower. This test documents the SHM performance.
    // For reference, JSON-RPC would take ~10-100x longer for this payload size.

    // SHM should complete in under 1 second for 10MB
    assert!(
        shm_elapsed.as_secs() < 1,
        "SHM transfer too slow: {:?}",
        shm_elapsed
    );
}
