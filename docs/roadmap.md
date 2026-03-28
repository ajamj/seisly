# StrataForge Roadmap

## v0.1 - Offline Wells + Surfaces (Current)

**Status:** In Progress

### Completed
- [x] Workspace skeleton with 8 crates
- [x] Core domain model (Well, Trajectory, Log, Surface)
- [x] CRS support with PROJ
- [x] SQLite schema and blob store
- [x] LAS parser
- [x] Trajectory CSV parser
- [x] XYZ surface parser
- [x] Delaunay triangulation
- [x] Trajectory resampling
- [x] CLI commands (init, import, list)

### Remaining
- [ ] Desktop app stub (egui + wgpu)
- [ ] SQLite write operations
- [ ] Integration tests
- [ ] Sample data files
- [ ] Build verification on Linux/macOS/Windows

**Target:** Q2 2026

---

## v0.2 - SEG-Y + Slice + Picks

**Status:** Planned

### Features
- [ ] SEG-Y import
  - [ ] Parse headers (text + binary)
  - [ ] Detect endianness
  - [ ] Build trace index
  - [ ] Handle various sample formats
- [ ] Seismic slice extraction
  - [ ] Inline slice
  - [ ] Xline slice
  - [ ] Time slice
  - [ ] Depth slice (if velocity model available)
- [ ] Horizon picking tools
  - [ ] Add pick on click
  - [ ] Edit pick position
  - [ ] Delete pick
  - [ ] Snap to peak/trough
  - [ ] Pick visualization
- [ ] Surface from picks
  - [ ] Build surface job
  - [ ] Store with provenance
  - [ ] Display in 3D view
- [ ] Tile caching
  - [ ] Generate tile pyramid
  - [ ] Cache derived products
  - [ ] LOD selection

### Desktop App
- [ ] Seismic slice viewer
- [ ] Picking mode toggle
- [ ] Well log viewer (2D)
- [ ] 3D scene with surfaces + wells

**Target:** Q3 2026

---

## v0.3 - Server Mode (REST)

**Status:** Planned

### Server Features
- [ ] axum REST server
  - [ ] Project management endpoints
  - [ ] Dataset CRUD
  - [ ] Pick operations
  - [ ] Job submission
- [ ] RemoteBackend implementation
  - [ ] REST client in sf_app
  - [ ] Offline/online mode toggle
- [ ] Job queue
  - [ ] In-memory queue
  - [ ] SQLite persistence
  - [ ] Progress tracking
- [ ] Basic auth
  - [ ] Static token (config file)
  - [ ] Header-based auth
  - [ ] HTTPS support

### API Endpoints
```
GET  /api/v1/projects
POST /api/v1/projects/open
GET  /api/v1/datasets
GET  /api/v1/datasets/{id}
POST /api/v1/picks
GET  /api/v1/picks?volume_id={id}
POST /api/v1/jobs
GET  /api/v1/jobs/{id}
```

**Target:** Q4 2026

---

## v0.4 - Advanced Features

**Status:** Future

### Seismic Attributes
- [ ] Amplitude extraction
- [ ] Instantaneous attributes
- [ ] Coherence
- [ ] Curvature

### Fault Modeling
- [ ] Fault stick picking
- [ ] Fault surface generation
- [ ] Fault seal analysis (basic)

### Grid Refinement
- [ ] Local grid refinement (LGR)
- [ ] Unstructured grid support

### Collaboration
- [ ] Multi-user editing
- [ ] Conflict resolution
- [ ] Change tracking

**Target:** 2027

---

## v0.5+ - Enterprise Features

**Status:** Future

### RESQML Support
- [ ] RESQML import/export
- [ ] Full object model support

### Simulator Integration
- [ ] Eclipse deck export
- [ ] CMG export
- [ ] Result import

### Cloud Integration
- [ ] AWS S3 backend
- [ ] Azure Blob backend
- [ ] Google Cloud Storage backend

### Web Viewer
- [ ] WebGL-based viewer
- [ ] Lightweight client

**Target:** TBD

---

## Technical Debt

### Known Issues
- [ ] Mesh serialization format (consider glTF)
- [ ] CRS transform not implemented (placeholder)
- [ ] No vertical datum handling
- [ ] Limited error messages

### Performance Improvements
- [ ] Async IO for large file reads
- [ ] Better memory management for seismic
- [ ] GPU instancing for well markers
- [ ] Frustum culling in 3D view

### Testing Gaps
- [ ] End-to-end workflow tests
- [ ] Cross-platform CI
- [ ] Performance benchmarks
- [ ] Fuzz testing for parsers

---

## Success Metrics

### v0.1
- [ ] Can import LAS, trajectory, surface
- [ ] Project opens on Linux, macOS, Windows
- [ ] All unit tests pass
- [ ] CLI usable for basic workflows

### v0.2
- [ ] Can pick horizons on seismic slices
- [ ] Surface generation from picks works
- [ ] 60 FPS rendering for 100k triangles

### v0.3
- [ ] Server handles 10 concurrent clients
- [ ] Job queue processes 100 jobs/hour
- [ ] API latency < 100ms for simple queries

---

## Contributing

Want to help? Areas needing contributors:
- SEG-Y parser implementation
- egui UI components
- Test coverage
- Documentation
- Sample data generation

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.
