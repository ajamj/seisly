# Phase 0 (v0.2.0) - Well-Seismic Workflow Design Specification

> **Design Status:** ✅ Approved  
> **Date:** 2026-03-29  
> **Author:** StrataForge Team  
> **Review Status:** Passed spec-document-reviewer

---

## 1. Overview

### 1.1 Goal

Complete Phase 0 foundation features to enable professional well-seismic workflow: full SEG-Y support, LAS 2.0/3.0, well-seismic tie with velocity modeling, and formation tops management.

### 1.2 Problem Statement

Current StrataForge v0.1.1 has:
- Partial SEG-Y support (incomplete header handling)
- LAS 2.0 only (no v3.0 support)
- No well-seismic tie capability
- No formation tops domain model

This limits the platform to basic visualization, preventing professional interpretation workflows that require well-to-seismic correlation.

### 1.3 Success Criteria

**Phase 0 is successful when:**
- ✅ All 7 tasks completed and merged
- ✅ All tests passing (unit + integration)
- ✅ User documentation complete (well-seismic tie guide)
- ✅ API documentation for new modules
- ✅ No breaking changes to existing APIs

---

## 2. Architecture

### 2.1 High-Level Design

```
┌─────────────────────────────────────────────────────────────┐
│                    Desktop Application                       │
│                    (sf_app - egui)                          │
├─────────────────────────────────────────────────────────────┤
│                    Well-Seismic Module                       │
│  ┌─────────────┐  ┌──────────────┐  ┌─────────────────┐    │
│  │ FormationTop│  │ WellTieEngine│  │ VelocityModel   │    │
│  │ (sf_core)   │  │ (sf_compute) │  │ (sf_compute)    │    │
│  └─────────────┘  └──────────────┘  └─────────────────┘    │
├─────────────────────────────────────────────────────────────┤
│                      I/O Layer                               │
│  ┌─────────────┐  ┌──────────────┐  ┌─────────────────┐    │
│  │ SEG-Y       │  │ LAS 2.0/3.0  │  │ Formation Tops  │    │
│  │ Reader/     │  │ Parser       │  │ CSV Import      │    │
│  │ Writer      │  │              │  │                 │    │
│  └─────────────┘  └──────────────┘  └─────────────────┘    │
└─────────────────────────────────────────────────────────────┘
```

### 2.2 Component Responsibilities

| Component | Crate | Responsibility |
|-----------|-------|----------------|
| `FormationTop` | sf_core | Domain model for stratigraphic markers |
| `WellTieEngine` | sf_compute | Time-depth conversion using velocity models |
| `SegyReader` | sf_io | SEG-Y file reading with hybrid approach |
| `SegyWriter` | sf_io | SEG-Y file writing |
| `LasV3Reader` | sf_io | LAS 3.0 format parsing |

### 2.3 Data Flow

```
User imports LAS file
         ↓
LasV3Reader parses → Well object
         ↓
User imports SEG-Y file
         ↓
SegyReader parses → SeismicVolume
         ↓
User creates Well-Seismic Tie
         ↓
WellTieEngine computes time-depth pairs
         ↓
Viewport displays well logs on seismic section
```

---

## 3. Detailed Design

### 3.1 FormationTop Domain Model

**File:** `crates/sf_core/src/domain/formation_top.rs`

```rust
pub struct FormationTop {
    pub id: FormationTopId,        // UUID
    pub well_id: Uuid,             // Parent well reference
    pub name: String,              // e.g., "Top Reservoir"
    pub depth_md: f64,             // Measured depth (meters)
    pub formation: Option<String>, // Optional formation name
    pub comments: Option<String>,  // Optional comments
}
```

**Key Methods:**
- `new(well_id, name, depth_md, formation)` - Constructor
- `with_comments(comments)` - Builder pattern for optional fields

**Serialization:** Full serde support (JSON, YAML, MessagePack)

---

### 3.2 SEG-Y Reader (Hybrid Approach)

**File:** `crates/sf_io/src/segy/reader.rs`

**Architecture:**
```
SegyReader
├── Mmap (memmap2)        ← Memory-mapped file access
├── SegyFile (segy-rs)    ← Header parsing
└── Custom trace access   ← Optimized data retrieval
```

**Key Methods:**
- `open(path)` - Open file with memory mapping
- `textual_header()` - Get EBCDIC header (decoded by segy-rs)
- `binary_header()` - Get binary header struct
- `trace_count()` - Number of traces in volume
- `read_trace(index)` - Read trace data + header

**Performance:**
- Zero-copy for trace data (via mmap)
- Lazy header parsing
- Thread-safe read access

---

### 3.3 SEG-Y Writer

**File:** `crates/sf_io/src/segy/writer.rs`

**Key Methods:**
- `new(path, sample_rate, trace_count, samples_per_trace)` - Create writer
- `write_trace(index, data)` - Write individual trace
- `finish()` - Finalize file (write headers, flush)

**Output Format:** SEG-Y Rev 1.0 (industry standard)

---

### 3.4 LAS 3.0 Parser

**File:** `crates/sf_io/src/las/v3.rs`

**LAS 3.0 Enhancements:**
- JSON-like metadata sections
- Enhanced curve definitions
- Better encoding support (UTF-8)

**Sections:**
1. `~VERSION INFORMATION` - File version
2. `~WELL INFORMATION` - Well metadata
3. `~CURVE INFORMATION` - Log curve definitions
4. `~PARAMETER INFORMATION` - Additional parameters
5. `~ASCII` - Log data section

**Backward Compatibility:** Full LAS 2.0 support maintained

---

### 3.5 Well-Seismic Tie Engine

**File:** `crates/sf_compute/src/well_tie.rs`

**Design Decision:** Use existing `LinearVelocity` model from `sf_compute::velocity`

```rust
// Reuse existing velocity model
use sf_compute::velocity::{VelocityModel, LinearVelocity};

pub struct WellTieEngine {
    velocity: Box<dyn VelocityModel>,
}

impl WellTieEngine {
    pub fn new(v0: f64, k: f64) -> Self {
        Self {
            velocity: Box::new(LinearVelocity::new(v0, k)),
        }
    }

    pub fn create_tie(&self, well: &Well) -> Result<WellTie> {
        // Generate time-depth pairs using V0 + kZ model
        // TWT = 2 * ∫(1/v(z)) dz from datum to depth
    }
}
```

**Time-Depth Formula:**
```
For linear velocity model v(z) = v0 + k*z:

TWT(depth) = (2/k) * ln((v0 + k*depth) / v0)

Where:
- v0 = surface velocity (m/s)
- k = velocity gradient (1/s)
- depth = depth below datum (m)
```

**Default Parameters:**
- `v0 = 2000 m/s` (typical sedimentary rock)
- `k = 0.5 1/s` (moderate compaction)

---

### 3.6 Testing Strategy

**Synthetic Test Data Generation:**

Use existing `sf_compute::synthetic` module:

```rust
// Generate synthetic SEG-Y
use sf_compute::synthetic::SyntheticSeismic;

let seismic = SyntheticSeismic::new(100, 100, 512);
let volume = seismic.generate();

// Write to temp file for testing
let mut writer = SegyWriter::new(temp_path, 4000, 100, 512);
// ... write traces
```

**Test Coverage:**
- Unit tests: All public functions (100%)
- Integration tests: Full roundtrip (SEG-Y, LAS)
- Property tests: Time-depth conversion accuracy

---

## 4. Error Handling

### 4.1 Error Types

**SEG-Y Errors:**
```rust
pub enum SegyError {
    IoError(std::io::Error),
    InvalidHeader(String),
    TraceOutOfBounds { index: usize, max: usize },
    ParseError(String),
}
```

**LAS Errors:**
```rust
pub enum LasError {
    IoError(std::io::Error),
    VersionNotSupported { version: String },
    MissingSection(String),
    InvalidCurveDefinition(String),
}
```

**Well Tie Errors:**
```rust
pub enum WellTieError {
    NoLogsAvailable,
    InvalidVelocityParameters,
    DepthRangeError { min: f64, max: f64 },
}
```

### 4.2 Error Recovery

| Error | Recovery |
|-------|----------|
| File not found | Return clear error message |
| Invalid header | Attempt partial read, warn user |
| Trace read failure | Skip trace, continue with next |
| Velocity model failure | Fallback to constant velocity |

---

## 5. Performance Considerations

### 5.1 Memory Management

**SEG-Y:**
- Memory-mapped files (no full load into RAM)
- Lazy trace loading (on-demand)
- Zero-copy for trace data access

**LAS:**
- Stream parsing (no full file in memory)
- Buffered I/O for large files

### 5.2 Benchmarks

**Target Performance:**
| Operation | Target | Notes |
|-----------|--------|-------|
| Open 1GB SEG-Y | <1s | mmap setup only |
| Read single trace | <10ms | Random access |
| Parse LAS file | <100ms | Typical 1000 curves |
| Create well tie | <50ms | 100 time-depth pairs |

---

## 6. API Design

### 6.1 Public Exports

**sf_core:**
```rust
pub use domain::formation_top::{FormationTop, FormationTopId};
```

**sf_io:**
```rust
pub use segy::reader::SegyReader;
pub use segy::writer::SegyWriter;
pub use las::v3::LasV3Reader;
```

**sf_compute:**
```rust
pub use well_tie::{WellTieEngine, WellTie, TimeDepthPair, TieParameters};
```

### 6.2 Usage Examples

**Formation Top:**
```rust
use sf_core::domain::FormationTop;

let top = FormationTop::new(
    well_id,
    "Top Reservoir".to_string(),
    2500.0,
    Some("Formation A".to_string()),
);
```

**SEG-Y Roundtrip:**
```rust
use sf_io::segy::{SegyReader, SegyWriter};

// Write
let mut writer = SegyWriter::new("output.segy", 4000, 100, 512);
writer.write_trace(0, &data)?;
writer.finish()?;

// Read
let reader = SegyReader::open("output.segy")?;
let trace = reader.read_trace(0)?;
```

**Well-Seismic Tie:**
```rust
use sf_compute::well_tie::{WellTieEngine, TieParameters};

let engine = WellTieEngine::new(2000.0, 0.5); // v0=2000, k=0.5
let tie = engine.create_tie(&well)?;

// Convert depth to TWT
let twt = WellTieEngine::depth_to_twt(2500.0, &tie.parameters);
```

---

## 7. Documentation Plan

### 7.1 User Guide

**File:** `docs/well_seismic_tie.md`

**Sections:**
1. Overview (what is well-seismic tie)
2. Prerequisites (data requirements)
3. Step-by-step workflow
   - Import well data (LAS)
   - Import seismic (SEG-Y)
   - Create well tie
   - View synthetic seismogram
   - Pick horizons
4. Time-depth conversion methods
5. Export options
6. Troubleshooting

### 7.2 API Documentation

**Location:** Rust doc comments (cargo doc)

**Coverage:**
- All public structs
- All public methods
- Usage examples in doc tests

### 7.3 Migration Guide

**Not required** - No breaking changes to existing APIs.

---

## 8. Testing Plan

### 8.1 Unit Tests

**Location:** Inline in source files (`#[cfg(test)]`)

**Coverage:**
- FormationTop creation and serialization
- SEG-Y header parsing
- LAS curve definition parsing
- Time-depth conversion accuracy

### 8.2 Integration Tests

**Location:** `crates/sf_io/tests/`, `crates/sf_compute/tests/`

**Tests:**
1. `segy_roundtrip` - Write then read, verify data integrity
2. `las_v3_parse` - Parse synthetic LAS 3.0 file
3. `well_tie_creation` - Full well tie workflow
4. `velocity_conversion` - Verify TWT formulas

### 8.3 Synthetic Data Generation

**Module:** `sf_compute::synthetic` (existing, enhanced)

**Generators:**
- `SyntheticSeismic` - 3D volume with configurable noise
- `SyntheticWellLog` - GR, DT, NPHI curves
- `SyntheticFormationTops` - Random stratigraphic markers

---

## 9. Dependencies

### 9.1 New Dependencies

```toml
# crates/sf_io/Cargo.toml
[dependencies]
segy-rs = "0.3"        # SEG-Y parsing
memmap2 = "0.9"        # Memory-mapped I/O
```

### 9.2 Existing Dependencies (reused)

```toml
# Already in workspace
serde = "1.0"          # Serialization
uuid = "1.0"           # Unique IDs
thiserror = "1.0"      # Error types
tempfile = "3.0"       # Test temp files
```

---

## 10. Timeline

### 10.1 Task Breakdown

| Week | Tasks | Deliverables |
|------|-------|--------------|
| 1 | Task 1 (FormationTop) | Domain model + tests |
| 2 | Task 2 (SEG-Y Reader) | Reader implementation |
| 3 | Task 3 (SEG-Y Writer) | Writer + roundtrip test |
| 4 | Task 4 (LAS 3.0) | Parser + tests |
| 5 | Task 5 (Well Tie) | Engine + velocity integration |
| 6 | Task 6-7 (Docs + Tests) | User guide + integration tests |

### 10.2 Milestones

- **Week 2:** FormationTop + SEG-Y Reader complete
- **Week 4:** All I/O modules complete
- **Week 6:** Full integration + docs complete
- **Week 6:** v0.2.0 release candidate

---

## 11. Risks & Mitigations

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| `segy-rs` incompatible | High | Low | Fallback to custom parser |
| Performance issues | Medium | Low | Benchmark early, optimize mmap |
| LAS 3.0 edge cases | Medium | Medium | Test with real files from community |
| Velocity model accuracy | Low | Low | Validate with synthetic data |

---

## 12. Future Enhancements (Post-Phase 0)

### Phase 1 (v0.3.0)
- Checkshot/VSP data import
- Multi-well tie correlation
- Synthetic seismogram generation
- Wavelet extraction from well ties

### Phase 2 (v1.0)
- Auto-tracking guided by well tops
- Time-stratigraphic slicing
- Well log visualization in 3D viewport

---

## Appendix A: Glossary

| Term | Definition |
|------|------------|
| **SEG-Y** | Standard format for seismic data exchange |
| **LAS** | Log ASCII Standard for well log data |
| **TWT** | Two-Way Time (seismic travel time) |
| **V0 + kZ** | Linear velocity model |
| **Formation Top** | Stratigraphic horizon marker |
| **Well Tie** | Correlation between well and seismic |

---

## Appendix B: References

1. SEG-Y Rev 1.0 Specification: https://www.seg.org/
2. LAS 3.0 Specification: https://www.cwls.org/
3. segy-rs crate: https://crates.io/crates/segy-rs
4. memmap2 crate: https://crates.io/crates/memmap2

---

**Document Version:** 1.0  
**Status:** ✅ Approved  
**Next Step:** Invoke writing-plans skill for implementation plan
