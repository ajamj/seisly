# Well Integration Specification

**Date:** 2026-03-28
**Phase:** v06 - Well Integration
**Status:** Approved

## Overview

Complete well integration system for StrataForge, enabling users to import, visualize, and interpret well data alongside seismic volumes. Supports LAS 2.0/3.0 formats, floating log viewer, and well-seismic tie in both 2D and 3D views.

## Goals

1. Import well data from LAS files (2.0 + 3.0)
2. Visualize well logs in floating viewer
3. Display well trajectory on seismic (2D section + 3D viewport)
4. Generate synthetic seismogram for well-seismic calibration
5. Enable well placement (manual + CSV import)
6. Edit well interpretation (tops, markers)

## Non-Goals

- Full petrophysical analysis (out of scope)
- Well completion design (out of scope)
- Production data management (out of scope)

## Architecture

### Component Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                    Well Integration Layer                    │
├─────────────────────────────────────────────────────────────┤
│  UI Layer                                                    │
│  ├─ Well Manager Panel (left panel)                         │
│  ├─ Floating Log Viewer (draggable window)                  │
│  └─ Well-Seismic Tie Display (2D + 3D)                      │
├─────────────────────────────────────────────────────────────┤
│  Data Layer                                                  │
│  ├─ LAS Parser (2.0 + 3.0)                                  │
│  ├─ Well Data Model (sf_core::domain::well)                 │
│  └─ Well-Seismic Tie Engine                                 │
├─────────────────────────────────────────────────────────────┤
│  Import/Export                                               │
│  ├─ LAS Import/Export                                       │
│  ├─ CSV Import (well locations)                             │
│  └─ Synthetic Generator (sf_compute::synthetic)             │
└─────────────────────────────────────────────────────────────┘
```

### Data Model Extensions

**Existing Well Model** (sf_core::domain::well):
```rust
pub struct Well {
    pub id: Uuid,
    pub name: String,
    pub symbol: String,
    pub location: WellLocation,  // X, Y, CRS
    pub datum: WellDatum,        // Elevation reference
    pub logs: Vec<WellLog>,      // GR, DT, RHOB, etc.
    pub tops: Vec<WellTop>,      // Formation markers
    pub is_visible: bool,
}
```

**New: Well Trajectory**:
```rust
pub struct WellTrajectory {
    pub md: Vec<f32>,      // Measured Depth
    pub tvd: Vec<f32>,     // True Vertical Depth
    pub x: Vec<f32>,       // Easting
    pub y: Vec<f32>,       // Northing
    pub inclination: Vec<f32>,
    pub azimuth: Vec<f32>,
}
```

**New: Synthetic Seismogram**:
```rust
pub struct SyntheticSeismogram {
    pub well_id: Uuid,
    pub depth: Vec<f32>,
    pub time: Vec<f32>,
    pub trace: Vec<f32>,
    pub wavelet: Vec<f32>,
}
```

## Implementation Details

### 1. LAS Parser

**LAS 2.0 Format:**
- ASCII-based
- Sections: ~Version, ~Well, ~Curve, ~Param, ~Ascii
- Curve data in columns

**LAS 3.0 Format:**
- XML-based
- More structured metadata
- Better unit handling

**Parser Interface:**
```rust
pub struct LasParser;

impl LasParser {
    pub fn read(path: &Path) -> Result<Well, LasError>;
    pub fn write(well: &Well, path: &Path) -> Result<(), LasError>;
    pub fn read_version(path: &Path) -> Result<LasVersion, LasError>;
}

pub enum LasVersion {
    Las20,
    Las30,
    Unknown,
}
```

**Error Handling:**
- Malformed sections
- Missing required curves
- Unit conversion errors
- Encoding issues

### 2. Well Manager Panel

**Location:** Left panel, replaces current "Wells" section

**Features:**
- Tree view with well list
- Context menu (right-click):
  - Import LAS
  - Export LAS
  - Delete Well
  - Properties
  - Rename
- Drag-and-drop reordering
- Multi-select (Ctrl+click)
- Visibility toggle (checkbox)
- Well symbol/color indicator

**UI Actions:**
```rust
enum WellManagerAction {
    ImportLas,
    ExportLas(Uuid),
    DeleteWell(Uuid),
    ToggleVisibility(Uuid),
    SelectWell(Uuid),
    OpenLogViewer(Uuid),
}
```

### 3. Floating Log Viewer

**Window Properties:**
- Draggable title bar
- Resizable corners
- Close button
- Stay-on-top option
- Docking (future enhancement)

**Log Display:**
- Depth track (leftmost)
- Multiple curve tracks (side-by-side)
- Configurable curves per track
- Curve styling:
  - Color
  - Line width
  - Fill (left/right)
  - Scale (min/max)

**Controls:**
- Zoom in/out (mouse wheel)
- Pan (drag)
- Fit to depth
- Track lock (sync depth across wells)

**Export:**
- PNG image
- PDF document
- SVG vector

**Structure:**
```rust
pub struct LogViewerWindow {
    pub well_id: Uuid,
    pub position: egui::Pos2,
    pub size: egui::Vec2,
    pub curves: Vec<CurveDisplay>,
    pub depth_range: (f32, f32),
    pub is_open: bool,
}

pub struct CurveDisplay {
    pub mnemonic: String,
    pub color: Color32,
    pub line_width: f32,
    pub fill: Option<FillStyle>,
    pub scale: (f32, f32),
}
```

### 4. Well-Seismic Tie

**2D Section Display:**
- Render well path on inline/crossline
- Display synthetic seismogram beside well
- Show formation tops as markers
- Time-depth conversion using velocity model

**3D Viewport Display:**
- Well trajectory as 3D line/tube
- Well head marker (sphere/cone)
- Formation tops as colored spheres
- Trajectory labeling

**Synthetic Seismogram Generation:**
```
1. Get well logs (DT, RHOB)
2. Calculate impedance: AI = RHOB * (1000000 / DT)
3. Calculate reflectivity: RC = (AI2 - AI1) / (AI2 + AI1)
4. Convolve with wavelet (Ricker, 35Hz)
5. Convert depth to time using velocity model
6. Display beside well path
```

**Time-Depth Calibration:**
- Use existing velocity model (V0, k)
- Formula: `TWT = 2 * depth / average_velocity`
- Interactive tie points (user adjusts)
- RMS error calculation

### 5. Well Placement

**Manual Placement:**
1. Click on map viewport
2. Dialog appears:
   - Well name (required)
   - Symbol (optional)
   - KB elevation (optional)
   - X, Y coordinates (auto-filled)
3. Create well with default trajectory (vertical)

**CSV Import:**
Format:
```csv
Name,X,Y,KB,TotalDepth
Well-1,500000,1000000,50,3000
Well-2,500100,1000100,55,2800
```

**Trajectory Generation:**
- Vertical well: straight line from surface
- Deviated well: minimum curvature method
- Import from survey data (MD, Inc, Azi)

## User Workflows

### Workflow 1: Import Well Data

1. Click "📂 Import LAS" in Well Manager
2. Select LAS file(s)
3. Preview curves and metadata
4. Confirm import
5. Well appears in tree
6. Double-click to open log viewer

### Workflow 2: Place New Well

1. Click "➕ Add Well"
2. Click on map location
3. Fill well info dialog
4. Well created with vertical trajectory
5. Edit properties as needed

### Workflow 3: Well-Seismic Tie

1. Select well in tree
2. Enable "Show Well Path" in viewport
3. Adjust velocity model if needed
4. Generate synthetic seismogram
5. Pick formation tops at well location
6. Export tie report

### Workflow 4: Log Analysis

1. Open floating log viewer
2. Add/remove curves
3. Adjust scales and colors
4. Add interpretation markers
5. Export log plot

## Error Handling

**Import Errors:**
- File not found → Show file picker
- Invalid LAS format → Show parse error details
- Missing curves → Warn but continue
- Unit mismatch → Auto-convert or ask user

**Runtime Errors:**
- Well not found → Graceful fallback
- Log data missing → Show placeholder
- Synthetic generation failed → Show error message

**Validation:**
- Well name must be unique
- Coordinates must be valid
- Depth must be positive
- Curves must have matching depth

## Testing Strategy

**Unit Tests:**
- LAS 2.0 parser
- LAS 3.0 parser
- Trajectory calculation
- Synthetic seismogram generation
- Time-depth conversion

**Integration Tests:**
- Import → Display → Export roundtrip
- Well placement → Visualization
- Log viewer → Curve styling

**Manual Testing:**
- Import real LAS files (multiple vendors)
- Test with deviated wells
- Test with missing data
- Performance with many wells (10+)

## Performance Considerations

**Log Rendering:**
- Downsample for display (1 sample per pixel)
- Lazy loading for deep wells
- Cache rendered curves

**Well-Seismic Tie:**
- Pre-calculate synthetic traces
- Update only on velocity change
- GPU rendering for 3D trajectories

**Memory:**
- Load logs on-demand
- Unload unused well data
- Compress trajectory data

## Future Enhancements (Out of Scope for v06)

- Multi-well correlation panel
- Cross-section view
- Well completion visualization
- Production data overlay
- Time-lapse (4D) well monitoring
- Automatic well-seismic tie (machine learning)

## Success Criteria

- [ ] Import LAS 2.0 files successfully
- [ ] Import LAS 3.0 files successfully
- [ ] Display well logs in floating viewer
- [ ] Render well trajectory in 2D section
- [ ] Render well trajectory in 3D viewport
- [ ] Generate synthetic seismogram
- [ ] Place wells manually
- [ ] Import wells from CSV
- [ ] Edit well tops/markers
- [ ] Export well data to LAS
- [ ] All unit tests passing
- [ ] No memory leaks with 10+ wells

## Timeline Estimate

| Phase | Component | Estimated Time |
|-------|-----------|----------------|
| 1 | LAS Parser | 2-3 hours |
| 2 | Well Manager | 1-2 hours |
| 3 | Well Placement | 1-2 hours |
| 4 | Log Viewer | 3-4 hours |
| 5 | Well-Seismic Tie | 4-5 hours |
| **Total** | | **12-18 hours** |

## Dependencies

**External:**
- `nom` crate for LAS parsing (optional, can use manual parsing)
- `serde_xml_rs` for LAS 3.0 (XML)

**Internal:**
- sf_core::domain::well (existing)
- sf_compute::synthetic (existing)
- sf_compute::velocity (existing)

## Risks & Mitigation

| Risk | Impact | Mitigation |
|------|--------|------------|
| LAS format variations | High | Test with multiple vendor files |
| Large well datasets | Medium | Implement downsampling |
| Complex trajectories | Medium | Use minimum curvature method |
| Performance with many wells | Low | Lazy loading, caching |

## Approval

**Design Author:** AI Assistant
**Approved By:** User
**Approval Date:** 2026-03-28
**Next Step:** Invoke writing-plans skill for implementation planning
