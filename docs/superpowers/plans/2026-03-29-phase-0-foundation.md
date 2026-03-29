# Phase 0 (v0.2.0) - StrataForge Pro Foundation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Complete Phase 0 foundation features to enable professional well-seismic workflow: full SEG-Y support, LAS 2.0/3.0, well-seismic tie, and formation tops.

**Architecture:** Enhance existing `sf_io` crate with complete SEG-Y reader/writer, add LAS 3.0 support, create new `well_tie` module in `sf_compute`, and add `FormationTop` domain model to `sf_core`.

**Tech Stack:** Rust 1.70+, memmap2, segy-rs (optional), serde, thiserror, uuid

---

## File Structure Map

### Files to Create
- `crates/sf_core/src/domain/formation_top.rs` - FormationTop domain model
- `crates/sf_compute/src/well_tie.rs` - Well-seismic tie computation module
- `crates/sf_io/src/segy/reader.rs` - Complete SEG-Y reader implementation
- `crates/sf_io/src/segy/writer.rs` - Complete SEG-Y writer implementation
- `crates/sf_io/src/las/v3.rs` - LAS 3.0 parser
- `crates/sf_io/tests/segy_test.rs` - SEG-Y integration tests
- `crates/sf_io/tests/las_v3_test.rs` - LAS 3.0 integration tests
- `docs/well_seismic_tie.md` - User guide for well-seismic workflow

### Files to Modify
- `crates/sf_core/src/domain.rs` - Export FormationTop module
- `crates/sf_core/src/lib.rs` - Add well_tie re-export
- `crates/sf_io/src/segy/mod.rs` - Export reader/writer modules
- `crates/sf_io/src/las/mod.rs` - Export LAS v3 module
- `crates/sf_storage/src/schema.rs` - Add formation_tops table
- `Cargo.toml` (workspace root) - Add segy-rs dependency (optional)
- `crates/sf_io/Cargo.toml` - Add segy-rs dependency (optional)

---

## Task 1: FormationTop Domain Model

**Files:**
- Create: `crates/sf_core/src/domain/formation_top.rs`
- Modify: `crates/sf_core/src/domain.rs`
- Test: `crates/sf_core/src/domain/formation_top.rs` (inline tests)

- [ ] **Step 1: Write the failing test**

Add this test to the new file:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_formation_top_creation() {
        let well_id = Uuid::new_v4();
        let top = FormationTop::new(
            well_id,
            "Top Reservoir".to_string(),
            2500.0,
            Some("Formation A".to_string()),
        );

        assert_eq!(top.well_id, well_id);
        assert_eq!(top.name, "Top Reservoir");
        assert_eq!(top.depth_md, 2500.0);
        assert_eq!(top.formation, Some("Formation A".to_string()));
    }

    #[test]
    fn test_formation_top_serialization() {
        let well_id = Uuid::new_v4();
        let top = FormationTop::new(
            well_id,
            "Top Seal".to_string(),
            1800.5,
            None,
        );

        let json = serde_json::to_string(&top).unwrap();
        assert!(json.contains("Top Seal"));
        assert!(json.contains("1800.5"));
    }
}
```

- [ ] **Step 2: Run test to verify it fails**

```bash
cd crates/sf_core
cargo test domain::formation_top::tests::test_formation_top_creation
```

Expected: FAIL with "unresolved import `FormationTop`"

- [ ] **Step 3: Write minimal implementation**

Create `crates/sf_core/src/domain/formation_top.rs`:

```rust
//! Formation Top domain model
//!
//! Represents a stratigraphic horizon picked on a well log or seismic.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identifier for a formation top
pub type FormationTopId = Uuid;

/// A formation top (marker) picked on a well
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormationTop {
    /// Unique identifier
    pub id: FormationTopId,
    /// Reference to the well this top belongs to
    pub well_id: Uuid,
    /// Name of the formation top (e.g., "Top Reservoir", "Base Seal")
    pub name: String,
    /// Measured depth in meters
    pub depth_md: f64,
    /// Optional formation name
    pub formation: Option<String>,
    /// Optional comments
    pub comments: Option<String>,
}

impl FormationTop {
    /// Create a new formation top
    pub fn new(
        well_id: Uuid,
        name: String,
        depth_md: f64,
        formation: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            well_id,
            name,
            depth_md,
            formation,
            comments: None,
        }
    }

    /// Set optional comments
    pub fn with_comments(mut self, comments: String) -> Self {
        self.comments = Some(comments);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formation_top_creation() {
        let well_id = Uuid::new_v4();
        let top = FormationTop::new(
            well_id,
            "Top Reservoir".to_string(),
            2500.0,
            Some("Formation A".to_string()),
        );

        assert_eq!(top.well_id, well_id);
        assert_eq!(top.name, "Top Reservoir");
        assert_eq!(top.depth_md, 2500.0);
        assert_eq!(top.formation, Some("Formation A".to_string()));
    }

    #[test]
    fn test_formation_top_serialization() {
        let well_id = Uuid::new_v4();
        let top = FormationTop::new(
            well_id,
            "Top Seal".to_string(),
            1800.5,
            None,
        );

        let json = serde_json::to_string(&top).unwrap();
        assert!(json.contains("Top Seal"));
        assert!(json.contains("1800.5"));
    }
}
```

- [ ] **Step 4: Run test to verify it passes**

```bash
cd crates/sf_core
cargo test domain::formation_top::tests
```

Expected: PASS (2 tests)

- [ ] **Step 5: Update domain.rs to export module**

Modify `crates/sf_core/src/domain.rs`:

```rust
// Add after existing exports
pub mod formation_top;
pub use formation_top::*;
```

- [ ] **Step 6: Commit**

```bash
git add crates/sf_core/src/domain/formation_top.rs crates/sf_core/src/domain.rs
git commit -m "feat(core): add FormationTop domain model

- New FormationTop struct with id, well_id, name, depth_md
- Support for optional formation name and comments
- Serialization via serde
- Unit tests for creation and JSON serialization"
```

---

## Task 2: Complete SEG-Y Reader Implementation (Hybrid Approach)

**Files:**
- Create: `crates/sf_io/src/segy/reader.rs`
- Modify: `crates/sf_io/src/segy/mod.rs`
- Test: `crates/sf_io/tests/segy_test.rs`

**Design Decision:** Hybrid approach - `segy-rs` for header parsing + custom I/O with `memmap2`

- [ ] **Step 1: Add dependencies**

Modify `crates/sf_io/Cargo.toml`:

```toml
[dependencies]
# Add to existing dependencies
segy-rs = "0.3"     # Header parsing (EBCDIC, binary)
memmap2 = "0.9"     # Memory-mapped file access
```

- [ ] **Step 2: Write integration test first**

Create `crates/sf_io/tests/segy_test.rs`:

```rust
use sf_io::segy::SegyReader;
use tempfile::TempDir;
use std::path::PathBuf;

#[test]
fn test_segy_reader_open_and_read() {
    // Generate synthetic SEG-Y for testing
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.segy");
    
    // Create test file using SegyWriter (tested separately)
    // ... setup code ...

    let reader = SegyReader::open(&test_file).unwrap();
    
    // Check binary header
    assert_eq!(reader.binary_header().sample_rate, 4000); // 4ms
    assert!(reader.binary_header().trace_count > 0);
    
    // Read first trace (zero-copy via mmap)
    let trace = reader.read_trace(0).unwrap();
    assert!(!trace.data.is_empty());
}

#[test]
fn test_segy_reader_textual_header() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.segy");
    
    // Create test file...

    let reader = SegyReader::open(&test_file).unwrap();
    let textual = reader.textual_header();
    
    // Should contain client info
    assert!(textual.contains("CLIENT"));
}
```

- [ ] **Step 3: Run test to verify it fails**

```bash
cd crates/sf_io
cargo test --test segy_test
```

Expected: FAIL with "unresolved import `SegyReader`"

- [ ] **Step 4: Implement SegyReader (Hybrid)**

Create `crates/sf_io/src/segy/reader.rs`:

```rust
//! SEG-Y file reader with memory-mapped access
//!
//! Hybrid approach:
//! - segy-rs: Header parsing (EBCDIC decoding, binary header)
//! - memmap2: Zero-copy trace data access
//! - Custom: Optimized I/O layer

use memmap2::Mmap;
use segy_rs::{SegyFile, SegyTrace, BinaryHeader};
use std::fs::File;
use std::path::Path;

use crate::error::IoError;

/// SEG-Y volume reader
pub struct SegyReader {
    mmap: Mmap,
    segy: SegyFile,
}
```

```rust
//! SEG-Y file reader with memory-mapped access
//!
//! Supports:
//! - Textual (EBCDIC) and binary headers
//! - Trace-level metadata
//! - Memory-mapped file access for performance

use memmap2::Mmap;
use segy_rs::{SegyFile, SegyTrace, BinaryHeader};
use std::fs::File;
use std::path::Path;

use crate::error::IoError;

/// SEG-Y volume reader
pub struct SegyReader {
    mmap: Mmap,
    segy: SegyFile,
}

impl SegyReader {
    /// Open a SEG-Y file with memory mapping
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, IoError> {
        let file = File::open(path.as_ref())?;
        let mmap = unsafe { Mmap::map(&file)? };
        let segy = SegyFile::from_bytes(&mmap)
            .map_err(|e| IoError::ParseError(e.to_string()))?;
        
        Ok(Self { mmap, segy })
    }

    /// Get textual header (EBCDIC)
    pub fn textual_header(&self) -> &str {
        self.segy.textual_header()
    }

    /// Get binary header
    pub fn binary_header(&self) -> &BinaryHeader {
        self.segy.binary_header()
    }

    /// Get number of traces
    pub fn trace_count(&self) -> usize {
        self.segy.trace_count()
    }

    /// Read a trace by index
    pub fn read_trace(&self, index: usize) -> Result<SegyTrace, IoError> {
        self.segy
            .trace(index)
            .map_err(|e| IoError::ParseError(e.to_string()))
    }

    /// Get trace header at index
    pub fn trace_header(&self, index: usize) -> Result<&segy_rs::TraceHeader, IoError> {
        let trace = self.read_trace(index)?;
        Ok(trace.header())
    }
}

/// Extended binary header with convenient accessors
pub struct ExtendedBinaryHeader {
    pub sample_rate: u32,
    pub trace_count: u32,
    pub samples_per_trace: u32,
    pub data_format: u16,
}

impl From<&BinaryHeader> for ExtendedBinaryHeader {
    fn from(header: &BinaryHeader) -> Self {
        Self {
            sample_rate: header.sample_rate(),
            trace_count: header.trace_count(),
            samples_per_trace: header.samples_per_trace(),
            data_format: header.data_format(),
        }
    }
}
```

- [ ] **Step 5: Update segy/mod.rs**

Modify `crates/sf_io/src/segy/mod.rs`:

```rust
mod reader;
mod writer;

pub use reader::*;
pub use writer::*;
```

- [ ] **Step 6: Run test to verify it passes**

```bash
cd crates/sf_io
cargo test --test segy_test
```

Expected: PASS (or skip if test data not available)

- [ ] **Step 7: Commit**

```bash
git add crates/sf_io/src/segy/reader.rs crates/sf_io/src/segy/mod.rs crates/sf_io/Cargo.toml
git commit -m "feat(io): implement complete SEG-Y reader

- SegyReader with memory-mapped file access
- Support for textual (EBCDIC) and binary headers
- Trace-level read access with metadata
- Integration tests with sample data"
```

---

## Task 3: Complete SEG-Y Writer Implementation

**Files:**
- Create: `crates/sf_io/src/segy/writer.rs`
- Test: `crates/sf_io/tests/segy_writer_test.rs`

- [ ] **Step 1: Write the failing test**

Create `crates/sf_io/tests/segy_writer_test.rs`:

```rust
use sf_io::segy::SegyWriter;
use tempfile::NamedTempFile;
use std::io::Write;

#[test]
fn test_segy_writer_create_and_write() {
    let temp_file = NamedTempFile::new().unwrap();
    
    // Create writer with standard parameters
    let mut writer = SegyWriter::new(
        temp_file.path(),
        4000, // 4ms sample rate
        100,  // 100 traces
        512,  // 512 samples per trace
    ).unwrap();

    // Write dummy traces
    for i in 0..100 {
        let data: Vec<f32> = (0..512).map(|j| (i as f32) * (j as f32) / 1000.0).collect();
        writer.write_trace(i, &data).unwrap();
    }

    writer.finish().unwrap();

    // Verify file can be read back
    let reader = SegyReader::open(temp_file.path()).unwrap();
    assert_eq!(reader.trace_count(), 100);
}
```

- [ ] **Step 2: Run test to verify it fails**

```bash
cd crates/sf_io
cargo test --test segy_writer_test
```

Expected: FAIL with "unresolved import `SegyWriter`"

- [ ] **Step 3: Implement SegyWriter**

Create `crates/sf_io/src/segy/writer.rs`:

```rust
//! SEG-Y file writer
//!
//! Supports:
//! - Custom binary header parameters
//! - Trace-by-trace writing
//! - Standard SEG-Y rev 1.0 format

use segy_rs::{SegyBuilder, SegyTraceBuilder};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use crate::error::IoError;

/// SEG-Y volume writer
pub struct SegyWriter {
    builder: SegyBuilder<BufWriter<File>>,
    trace_count: u32,
    current_trace: u32,
}

impl SegyWriter {
    /// Create a new SEG-Y writer
    pub fn new<P: AsRef<Path>>(
        path: P,
        sample_rate: u32,      // in microseconds (4000 = 4ms)
        trace_count: u32,
        samples_per_trace: u32,
    ) -> Result<Self, IoError> {
        let file = File::create(path.as_ref())?;
        let writer = BufWriter::new(file);

        let builder = SegyBuilder::new()
            .sample_rate(sample_rate)
            .traces(trace_count)
            .samples_per_trace(samples_per_trace)
            .build(writer)
            .map_err(|e| IoError::WriteError(e.to_string()))?;

        Ok(Self {
            builder,
            trace_count,
            current_trace: 0,
        })
    }

    /// Write a trace at specified index
    pub fn write_trace(&mut self, index: u32, data: &[f32]) -> Result<(), IoError> {
        if index >= self.trace_count {
            return Err(IoError::WriteError(format!(
                "Trace index {} out of bounds (max {})",
                index, self.trace_count - 1
            )));
        }

        let trace = SegyTraceBuilder::new()
            .data(data.to_vec())
            .build()
            .map_err(|e| IoError::WriteError(e.to_string()))?;

        self.builder
            .write_trace(index as usize, &trace)
            .map_err(|e| IoError::WriteError(e.to_string()))?;

        self.current_trace = index + 1;
        Ok(())
    }

    /// Finish writing and flush
    pub fn finish(mut self) -> Result<(), IoError> {
        self.builder
            .finish()
            .map_err(|e| IoError::WriteError(e.to_string()))?;
        Ok(())
    }
}
```

- [ ] **Step 4: Run test to verify it passes**

```bash
cd crates/sf_io
cargo test --test segy_writer_test
```

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/sf_io/src/segy/writer.rs crates/sf_io/tests/segy_writer_test.rs
git commit -m "feat(io): implement SEG-Y writer

- SegyWriter with configurable parameters
- Trace-by-trace writing API
- Automatic header generation
- Round-trip test with SegyReader"
```

---

## Task 4: LAS 3.0 Parser Implementation

**Files:**
- Create: `crates/sf_io/src/las/v3.rs`
- Modify: `crates/sf_io/src/las/mod.rs`
- Test: `crates/sf_io/tests/las_v3_test.rs`

- [ ] **Step 1: Write the failing test**

Create `crates/sf_io/tests/las_v3_test.rs`:

```rust
use sf_io::las::LasV3Reader;
use std::path::PathBuf;

#[test]
fn test_las_v3_reader_parse() {
    let test_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data/sample_v3.las");
    
    if !test_file.exists() {
        return;
    }

    let reader = LasV3Reader::open(&test_file).unwrap();
    let well = reader.parse().unwrap();

    assert_eq!(well.name, "TEST-WELL");
    assert!(!well.curves.is_empty());
    assert!(well.curves.iter().any(|c| c.mnemonic == "GR"));
}

#[test]
fn test_las_v3_with_metadata() {
    let test_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data/sample_v3.las");
    
    if !test_file.exists() {
        return;
    }

    let reader = LasV3Reader::open(&test_file).unwrap();
    let well = reader.parse().unwrap();

    // LAS 3.0 should have enhanced metadata
    assert!(well.metadata.contains_key("API"));
}
```

- [ ] **Step 2: Run test to verify it fails**

```bash
cd crates/sf_io
cargo test --test las_v3_test
```

Expected: FAIL

- [ ] **Step 3: Implement LasV3Reader**

Create `crates/sf_io/src/las/v3.rs`:

```rust
//! LAS 3.0 file parser
//!
//! LAS 3.0 enhancements:
//! - JSON-like metadata sections
//! - Enhanced curve definitions
//! - Better encoding support

use crate::domain::well::{Well, WellCurve, WellLog};
use crate::error::IoError;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// LAS 3.0 file reader
pub struct LasV3Reader {
    path: std::path::PathBuf,
}

impl LasV3Reader {
    /// Open a LAS 3.0 file
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, IoError> {
        Ok(Self {
            path: path.as_ref().to_path_buf(),
        })
    }

    /// Parse the LAS file and return Well structure
    pub fn parse(&self) -> Result<Well, IoError> {
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        
        let mut version_section = HashMap::new();
        let mut well_section = HashMap::new();
        let mut curve_section = Vec::new();
        let mut data_section = Vec::new();
        let mut current_section = String::new();

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();

            if line.starts_with('~') {
                current_section = line.to_string();
                continue;
            }

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            match current_section.as_str() {
                "~VERSION INFORMATION" => {
                    // Parse version info
                    let parts: Vec<&str> = line.splitn(2, ':').collect();
                    if parts.len() == 2 {
                        let key = parts[0].trim();
                        let value = parts[1].trim();
                        version_section.insert(key.to_string(), value.to_string());
                    }
                }
                "~WELL INFORMATION" => {
                    let parts: Vec<&str> = line.splitn(2, ':').collect();
                    if parts.len() == 2 {
                        let key = parts[0].trim();
                        let value = parts[1].trim();
                        well_section.insert(key.to_string(), value.to_string());
                    }
                }
                "~CURVE INFORMATION" => {
                    // Format: MNEM.UNIT API CODE : DESCR
                    let parts: Vec<&str> = line.split(':').collect();
                    if parts.len() >= 1 {
                        let curve_def = parts[0].trim();
                        let curve = self.parse_curve_definition(curve_def)?;
                        curve_section.push(curve);
                    }
                }
                "~ASCII" | "~PARAMETER INFORMATION" => {
                    // Data section
                    if current_section == "~ASCII" {
                        let values: Result<Vec<f64>, _> = line
                            .split_whitespace()
                            .map(|s| s.parse::<f64>())
                            .collect();
                        if let Ok(values) = values {
                            data_section.push(values);
                        }
                    }
                }
                _ => {}
            }
        }

        // Build Well structure
        let well_name = well_section
            .get("STRT")
            .map(|s| s.clone())
            .unwrap_or_else(|| "Unknown".to_string());

        let well = Well {
            id: uuid::Uuid::new_v4(),
            name: well_name,
            curves: curve_section,
            data: data_section,
            metadata: well_section,
        };

        Ok(well)
    }

    fn parse_curve_definition(&self, def: &str) -> Result<WellCurve, IoError> {
        let parts: Vec<&str> = def.split_whitespace().collect();
        
        if parts.is_empty() {
            return Err(IoError::ParseError("Empty curve definition".to_string()));
        }

        let mnemonic = parts[0].to_string();
        let unit = parts.get(1).map(|s| s.to_string()).unwrap_or_default();
        let description = parts.get(2..).map(|p| p.join(" ")).unwrap_or_default();

        Ok(WellCurve {
            mnemonic,
            unit,
            description,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curve_definition_parser() {
        let reader = LasV3Reader {
            path: std::path::PathBuf::new(),
        };

        let curve = reader.parse_curve_definition("GR GAPI 00 000 : Gamma Ray").unwrap();
        assert_eq!(curve.mnemonic, "GR");
        assert_eq!(curve.unit, "GAPI");
    }
}
```

- [ ] **Step 4: Update las/mod.rs**

Modify `crates/sf_io/src/las/mod.rs`:

```rust
mod v2;
mod v3;

pub use v2::*;
pub use v3::*;
```

- [ ] **Step 5: Commit**

```bash
git add crates/sf_io/src/las/v3.rs crates/sf_io/src/las/mod.rs
git commit -m "feat(io): add LAS 3.0 parser support

- LasV3Reader with enhanced metadata support
- JSON-like section parsing
- Backward compatible with LAS 2.0 format
- Unit tests for curve definition parsing"
```

---

## Task 5: Well-Seismic Tie Module (Using Existing LinearVelocity)

**Files:**
- Create: `crates/sf_compute/src/well_tie.rs`
- Test: `crates/sf_compute/tests/well_tie_test.rs`

**Design Decision:** Reuse existing `LinearVelocity` model from `sf_compute::velocity` instead of creating new implementation.

**Formula:** For linear velocity model v(z) = v0 + k*z:
```
TWT(depth) = (2/k) * ln((v0 + k*depth) / v0)
```

- [ ] **Step 1: Write the failing test**

Create `crates/sf_compute/tests/well_tie_test.rs`:

```rust
use sf_compute::well_tie::{WellTieEngine, TieParameters};
use sf_core::domain::{Well, WellLog};

#[test]
fn test_well_tie_with_velocity_model() {
    // Create synthetic well
    let mut well = Well::new("Test Well".to_string(), 0.0, 0.0);

    // Add GR log
    let depths: Vec<f64> = (0..100).map(|i| i as f64 * 10.0).collect();
    let gr_values: Vec<f64> = (0..100).map(|i| 50.0 + (i as f64 * 0.5)).collect();

    well.add_log(WellLog {
        mnemonic: "GR".to_string(),
        unit: "GAPI".to_string(),
        depths: depths.clone(),
        values: gr_values,
    });

    // Create tie engine with V0 + kZ model
    let engine = WellTieEngine::new(2000.0, 0.5); // v0=2000 m/s, k=0.5 1/s
    let tie = engine.create_tie(&well).unwrap();

    assert_eq!(tie.well_id, well.id);
    assert!(!tie.time_depth_pairs.is_empty());
    
    // Verify accuracy: for v0=2000, k=0.5, depth=1000m:
    // TWT = (2/0.5) * ln((2000 + 0.5*1000) / 2000) = 4 * ln(1.25) ≈ 892ms
    let twt = tie.time_depth_pairs.iter()
        .find(|p| p.depth_md == 1000.0).unwrap().twt;
    assert!((twt - 892.0).abs() < 1.0);
}
```

- [ ] **Step 2: Run test to verify it fails**

```bash
cd crates/sf_compute
cargo test --test well_tie_test
```

Expected: FAIL with "unresolved import `WellTieEngine`"

- [ ] **Step 3: Implement WellTieEngine (reusing LinearVelocity)**

Create `crates/sf_compute/src/well_tie.rs`:

```rust
//! Well-seismic tie computation
//!
//! Uses existing LinearVelocity model (V0 + kZ) from sf_compute::velocity
//! for accurate time-depth conversion.
//!
//! Formula: TWT = (2/k) * ln((v0 + k*depth) / v0)

use crate::error::ComputeError;
use crate::velocity::LinearVelocity;
use sf_core::domain::{Well, WellLog, FormationTop};
use uuid::Uuid;

/// Parameters for well-seismic tie
#[derive(Debug, Clone)]
pub struct TieParameters {
    /// Datum elevation (meters)
    pub datum_elevation: f64,
    /// Surface velocity v0 (m/s)
    pub v0: f64,
    /// Velocity gradient k (1/s)
    pub k: f64,
}

impl Default for TieParameters {
    fn default() -> Self {
        Self {
            datum_elevation: 0.0,
            v0: 2000.0, // Typical sedimentary rock
            k: 0.5,     // Moderate compaction
        }
    }
}

/// Time-depth pair for well tie
#[derive(Debug, Clone)]
pub struct TimeDepthPair {
    pub depth_md: f64,    // Measured depth (m)
    pub twt: f64,         // Two-way time (ms)
}

/// Well-seismic tie result
#[derive(Debug, Clone)]
pub struct WellTie {
    pub id: Uuid,
    pub well_id: Uuid,
    pub time_depth_pairs: Vec<TimeDepthPair>,
    pub parameters: TieParameters,
}

/// Well tie computation engine using LinearVelocity model
pub struct WellTieEngine {
    velocity: LinearVelocity,
}

impl WellTieEngine {
    /// Create a new well tie engine with V0 + kZ velocity model
    pub fn new(v0: f64, k: f64) -> Self {
        Self {
            velocity: LinearVelocity::new(v0, k),
        }
    }

    /// Create well-seismic tie using V0 + kZ model
    pub fn create_tie(&self, well: &Well) -> Result<WellTie, ComputeError> {
        // Get first log for depth range
        let first_log = well
            .logs
            .first()
            .ok_or_else(|| ComputeError::InvalidInput("Well has no logs".to_string()))?;

        let min_depth = *first_log.depths.first().unwrap_or(&0.0);
        let max_depth = *first_log.depths.last().unwrap_or(&1000.0);

        // Generate time-depth pairs using LinearVelocity
        let mut pairs = Vec::new();
        let step = 10.0; // 10m intervals

        let mut depth = min_depth;
        while depth <= max_depth {
            // TWT = (2/k) * ln((v0 + k*depth) / v0) * 1000 (convert to ms)
            let v0 = self.velocity.v0();
            let k = self.velocity.k();
            let twt = (2.0 / k) * ((v0 + k * depth) / v0).ln() * 1000.0;
            
            pairs.push(TimeDepthPair {
                depth_md: depth,
                twt,
            });

            depth += step;
        }

        Ok(WellTie {
            id: Uuid::new_v4(),
            well_id: well.id,
            time_depth_pairs: pairs,
            parameters: TieParameters {
                datum_elevation: 0.0,
                v0,
                k,
            },
        })
    }

    /// Convert depth to TWT using V0 + kZ model
    pub fn depth_to_twt(depth: f64, v0: f64, k: f64) -> f64 {
        (2.0 / k) * ((v0 + k * depth) / v0).ln() * 1000.0
    }

    /// Convert TWT to depth (inverse formula)
    pub fn twt_to_depth(twt: f64, v0: f64, k: f64) -> f64 {
        let twt_sec = twt / 1000.0; // Convert ms to seconds
        (v0 * ((k * twt_sec / 2.0).exp() - 1.0)) / k
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_depth_time_conversion() {
        // For v0=2000, k=0.5, depth=1000m:
        // TWT = (2/0.5) * ln((2000 + 0.5*1000) / 2000) * 1000 = 892ms
        let twt = WellTieEngine::depth_to_twt(1000.0, 2000.0, 0.5);
        assert!((twt - 892.0).abs() < 1.0);

        // Back conversion
        let depth = WellTieEngine::twt_to_depth(twt, 2000.0, 0.5);
        assert!((depth - 1000.0).abs() < 1.0);
    }
}
```

- [ ] **Step 4: Commit**

```bash
git add crates/sf_compute/src/well_tie.rs
git commit -m "feat(compute): add well-seismic tie engine with V0 + kZ model

- WellTieEngine reusing existing LinearVelocity
- Accurate time-depth conversion formula
- Bidirectional depth<->TWT conversion
- Unit tests for formula accuracy"
```

---

## Task 6: Update README and Documentation

**Files:**
- Modify: `README.md`
- Create: `docs/well_seismic_tie.md`

- [ ] **Step 1: Update README.md**

Add to `README.md` features section:

```markdown
### ✨ Features

#### Core Features
- ✅ **Seismic Visualization** - 3D volume rendering with inline/crossline slicing
- ✅ **Horizon Interpretation** - Manual picking, auto-tracking, seed picking
- ✅ **Fault Modeling** - Interactive sketch mode, RBF surface modeling
- ✅ **Velocity Modeling** - Linear velocity model (V0 + kZ)
- ✅ **Time-Depth Conversion** - Real-time depth domain visualization
- ✅ **Well-Seismic Tie** - Integration of well logs with seismic data (NEW v0.2)
- ✅ **Formation Tops** - Stratigraphic marker management (NEW v0.2)
- ✅ **Full SEG-Y Support** - Reader/writer with textual/binary headers (NEW v0.2)
- ✅ **LAS 2.0/3.0** - Complete well log import (NEW v0.2)
```

- [ ] **Step 2: Create user guide**

Create `docs/well_seismic_tie.md`:

```markdown
# Well-Seismic Tie Workflow

## Overview

Well-seismic tie integrates well log data with seismic volume, enabling:
- Correlation of geological formations with seismic reflections
- Synthetic seismogram generation
- Horizon picking guidance from well markers

## Prerequisites

- Seismic volume (SEG-Y format)
- Well log data (LAS 2.0 or 3.0 format)
- Checkshot or VSP data (optional, for accurate time-depth relationship)

## Step-by-Step Workflow

### 1. Import Well Data

```bash
sf import --project MyField.sf las --well "Well-1" well1.las
```

### 2. Import Formation Tops (Optional)

```bash
sf import --project MyField.sf tops --well "Well-1" tops.csv
```

CSV format:
```csv
well_id,name,depth_md,formation
Well-1,Top Reservoir,2500.0,Formation A
Well-1,Base Seal,2700.0,Formation B
```

### 3. Create Well-Seismic Tie

In the desktop application:
1. Select well from explorer
2. Right-click → "Create Well-Seismic Tie"
3. Choose replacement velocity or checkshot file
4. Click "Generate"

### 4. View Synthetic Seismogram

The synthetic seismogram will be displayed alongside the seismic volume at the well location.

### 5. Pick Horizons

Use formation tops as guides for horizon picking:
1. Enable "Show Formation Tops" in viewport
2. Select auto-track mode
3. Click near formation top to seed horizon

## Time-Depth Conversion

### Replacement Velocity Method

Simple constant velocity approach:

```
TWT (ms) = 2 × (Depth - Datum) / Velocity × 1000
```

Default velocity: 2000 m/s (typical sedimentary rock)

### Checkshot Method (Coming Soon)

Use actual checkshot measurements for accurate time-depth relationship.

## Export

Export well tie results:

```bash
sf export --project MyField.sf well-tie --well "Well-1" --output tie.json
```

## Troubleshooting

### Poor Tie Quality

- Check replacement velocity (try 1800-2500 m/s range)
- Verify well deviation (use deviated well correction)
- Check seismic polarity

### Missing Data

- Ensure LAS file has GR or DT log
- Verify SEG-Y has valid trace headers
```

- [ ] **Step 3: Commit**

```bash
git add README.md docs/well_seismic_tie.md
git commit -m "docs: add well-seismic tie user guide

- Update README with v0.2 features
- Complete workflow documentation
- Step-by-step tutorial with examples"
```

---

## Task 7: Integration Tests and Final Polish

**Files:**
- Create: `crates/sf_io/tests/integration_test.rs`
- Modify: Various (fix any issues found)

- [ ] **Step 1: Create comprehensive integration test**

Create `crates/sf_io/tests/integration_test.rs`:

```rust
//! Integration tests for complete Phase 0 workflow

use sf_io::segy::{SegyReader, SegyWriter};
use sf_io::las::LasV3Reader;
use tempfile::{NamedTempFile, TempDir};
use std::io::Write;

#[test]
fn test_full_segy_roundtrip() {
    let temp_dir = TempDir::new().unwrap();
    let segy_path = temp_dir.path().join("test.segy");

    // Write SEG-Y
    {
        let mut writer = SegyWriter::new(&segy_path, 4000, 10, 100).unwrap();
        for i in 0..10 {
            let data: Vec<f32> = (0..100).map(|j| (i * j) as f32).collect();
            writer.write_trace(i, &data).unwrap();
        }
        writer.finish().unwrap();
    }

    // Read back
    let reader = SegyReader::open(&segy_path).unwrap();
    assert_eq!(reader.trace_count(), 10);
    
    let trace = reader.read_trace(0).unwrap();
    assert_eq!(trace.data.len(), 100);
}

#[test]
fn test_las_v3_roundtrip() {
    // Create temporary LAS file
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "~VERSION INFORMATION").unwrap();
    writeln!(temp_file, "  VERS. 3.0 : CWLS LOG ASCII STANDARD -VERSION 3.0").unwrap();
    writeln!(temp_file, "~WELL INFORMATION").unwrap();
    writeln!(temp_file, "  STRT.M 0.0 : START DEPTH").unwrap();
    writeln!(temp_file, "~CURVE INFORMATION").unwrap();
    writeln!(temp_file, "  DEPT.M : Depth").unwrap();
    writeln!(temp_file, "  GR.GAPI : Gamma Ray").unwrap();
    writeln!(temp_file, "~ASCII").unwrap();
    writeln!(temp_file, "0 50").unwrap();
    writeln!(temp_file, "10 55").unwrap();
    writeln!(temp_file, "20 60").unwrap();

    // Read LAS file
    let reader = LasV3Reader::open(temp_file.path()).unwrap();
    let well = reader.parse().unwrap();

    assert!(!well.curves.is_empty());
    assert_eq!(well.curves[0].mnemonic, "DEPT");
}
```

- [ ] **Step 2: Run all Phase 0 tests**

```bash
cd crates/sf_io
cargo test

cd crates/sf_compute
cargo test well_tie

cd crates/sf_core
cargo test formation_top
```

Expected: All tests pass

- [ ] **Step 3: Run workspace-wide tests**

```bash
cargo test --workspace
```

Expected: All tests pass

- [ ] **Step 4: Run clippy**

```bash
cargo clippy --workspace --all-targets -- -D warnings
```

Fix any warnings.

- [ ] **Step 5: Final commit**

```bash
git add crates/sf_io/tests/integration_test.rs
git commit -m "test: add comprehensive integration tests for Phase 0

- Full SEG-Y roundtrip test
- LAS 3.0 parsing test
- Well tie computation test
- All tests passing"
```

---

## Task 8: Create GitHub Project Board

**Files:**
- N/A (GitHub UI)

- [ ] **Step 1: Go to GitHub Project**

Navigate to: https://github.com/ajamj/StrataForge/projects

- [ ] **Step 2: Create new project**

- Click "New project"
- Choose "Project board" template
- Name: "Phase 0 - v0.2.0 Foundation"

- [ ] **Step 3: Add columns**

Create columns:
- 📋 Backlog
- 🔄 In Progress
- 👀 In Review
- ✅ Done

- [ ] **Step 4: Add issues**

Create issues for each task in this plan:
1. "FormationTop Domain Model"
2. "Complete SEG-Y Reader"
3. "Complete SEG-Y Writer"
4. "LAS 3.0 Parser"
5. "Well-Seismic Tie Engine"
6. "Documentation Update"
7. "Integration Tests"

- [ ] **Step 5: Add milestone**

Create milestone "v0.2.0 - Phase 0 Foundation"
Due date: 6 weeks from now

---

## Summary

### Files Created (11)
1. `crates/sf_core/src/domain/formation_top.rs`
2. `crates/sf_io/src/segy/reader.rs`
3. `crates/sf_io/src/segy/writer.rs`
4. `crates/sf_io/src/las/v3.rs`
5. `crates/sf_compute/src/well_tie.rs`
6. `crates/sf_io/tests/segy_test.rs`
7. `crates/sf_io/tests/segy_writer_test.rs`
8. `crates/sf_io/tests/las_v3_test.rs`
9. `crates/sf_compute/tests/well_tie_test.rs`
10. `crates/sf_io/tests/integration_test.rs`
11. `docs/well_seismic_tie.md`

### Files Modified (7)
1. `crates/sf_core/src/domain.rs`
2. `crates/sf_io/src/segy/mod.rs`
3. `crates/sf_io/src/las/mod.rs`
4. `crates/sf_storage/src/schema.rs` (optional, for formation tops persistence)
5. `crates/sf_io/Cargo.toml`
6. `README.md`
7. `.gitignore` (already done)

### Total Commits: 8
1. FormationTop domain model
2. SEG-Y reader implementation
3. SEG-Y writer implementation
4. LAS 3.0 parser
5. Well-seismic tie engine
6. Documentation update
7. Integration tests
8. Final polish

### Estimated Time: 4-6 weeks
- Week 1-2: Core implementation (Tasks 1-5)
- Week 3: Documentation & testing (Tasks 6-7)
- Week 4: Bug fixes & polish
- Week 5-6: Buffer for unexpected issues

---

**Plan complete and saved to `docs/superpowers/plans/2026-03-29-phase-0-foundation.md`. Two execution options:**

**1. Subagent-Driven (recommended)** - I dispatch a fresh subagent per task, review between tasks, fast iteration

**2. Inline Execution** - Execute tasks in this session using executing-plans, batch execution with checkpoints

**Which approach?**
