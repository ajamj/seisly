# Phase 1 (v0.3.0) - Advanced Features Design Specification

> **Design Status:** 📋 Draft  
> **Date:** 2026-03-29  
> **Author:** StrataForge Team  
> **Target:** Q3 2026

---

## 1. Overview

### 1.1 Goal

Implement advanced features to make StrataForge Pro competitive with Petrel, DUG Insight, and OpendTect Pro:
- ML-powered auto-tracking
- Seismic attributes (100+)
- Plugin system (Rust + Python)
- Advanced visualization

### 1.2 Success Criteria

**Phase 1 is successful when:**
- ✅ ML auto-tracking achieves >80% accuracy on synthetic data
- ✅ 100+ seismic attributes implemented
- ✅ Plugin system supports Rust and Python plugins
- ✅ All tests passing (200+ tests)
- ✅ Documentation complete

---

## 2. Architecture

### 2.1 High-Level Design

```
StrataForge Pro (v0.3.0)
├── Core Engine (Phase 0 ✅)
│   ├── sf_core      - Domain models
│   ├── sf_io        - SEG-Y, LAS, XYZ
│   ├── sf_compute   - Velocity, well tie
│   └── sf_storage   - SQLite, blob store
│
├── Advanced Features (Phase 1 🚧)
│   ├── sf_ml        - ML inference (candle/ONNX)
│   ├── sf_attributes - Seismic attributes
│   ├── sf_plugin    - Plugin system
│   └── sf_render    - Advanced visualization
│
└── Applications
    ├── sf_app       - Desktop GUI
    ├── sf_cli       - CLI tools
    └── sf_server    - REST/gRPC server (Phase 2)
```

### 2.2 New Crates

| Crate | Purpose | Dependencies |
|-------|---------|--------------|
| `sf_ml` | ML inference | candle-core, candle-nn, ONNX Runtime |
| `sf_attributes` | Seismic attributes | sf_core, sf_compute, ndarray |
| `sf_plugin` | Plugin system | mlua (Lua), pyo3 (Python) |
| `sf_render` (enhanced) | Advanced viz | wgpu, imgui, custom shaders |

---

## 3. Feature Specifications

### 3.1 ML Auto-Tracking

**Problem:** Manual horizon picking is slow and subjective.

**Solution:** CNN-based auto-tracking using synthetic training data.

**Architecture:**
```
User picks seed point
    ↓
Extract local seismic patch
    ↓
CNN predicts horizon probability
    ↓
Track along maximum probability
    ↓
Output: Horizon surface
```

**Model Architecture:**
```
Input: 64x64 seismic patch
    ↓
Conv2D(32) → ReLU → MaxPool
    ↓
Conv2D(64) → ReLU → MaxPool
    ↓
Conv2D(128) → ReLU → GlobalAvgPool
    ↓
Dense(64) → ReLU
    ↓
Output: Horizon offset (continuous)
```

**Training Data:**
- Synthetic seismic from Phase 0
- Augmented with noise, faults, channels
- 10,000+ training samples

**Implementation:**
- Train in PyTorch → Export to ONNX
- Run inference with `candle` (Rust) or ONNX Runtime
- Target: <100ms inference per patch

**Files:**
- `crates/sf_ml/src/tracker.rs` - Auto-tracking engine
- `crates/sf_ml/src/cnn.rs` - CNN model
- `crates/sf_ml/tests/auto_track_test.rs` - Integration tests
- `models/auto_track.onnx` - Trained model

---

### 3.2 Seismic Attributes (100+)

**Problem:** Geoscientists need attributes for interpretation.

**Solution:** Comprehensive attribute library with GPU acceleration.

**Attribute Categories:**

| Category | Attributes | Count |
|----------|------------|-------|
| **Amplitude** | RMS, Mean, Max, Min, Std Dev | 10 |
| **Frequency** | Instantaneous, Dominant, Peak | 15 |
| **Phase** | Instantaneous, Average, Weighted | 10 |
| **Geometric** | Dip, Azimuth, Curvature | 20 |
| **Similarity** | Coherence, Chaos, Semblance | 15 |
| **Spectral** | Decomposition (CWT, STFT) | 20 |
| **Physical** | Impedance, Poisson's ratio | 10 |
| **Total** | | **100+** |

**Implementation:**
```rust
pub trait SeismicAttribute: Send + Sync {
    fn name(&self) -> &'static str;
    fn compute(&self, trace: &[f32], dt: f32) -> Vec<f32>;
    fn parameters(&self) -> AttributeParameters;
}

// Example: RMS Amplitude
pub struct RmsAmplitude {
    window_ms: f32,
}

impl SeismicAttribute for RmsAmplitude {
    fn compute(&self, trace: &[f32], dt: f32) -> Vec<f32> {
        let window_samples = (self.window_ms / dt) as usize;
        trace
            .windows(window_samples)
            .map(|w| (w.iter().map(|x| x * x).sum::<f32>() / w.len() as f32).sqrt())
            .collect()
    }
}
```

**Files:**
- `crates/sf_attributes/src/lib.rs` - Attribute registry
- `crates/sf_attributes/src/amplitude.rs` - Amplitude attributes
- `crates/sf_attributes/src/frequency.rs` - Frequency attributes
- `crates/sf_attributes/src/geometric.rs` - Geometric attributes
- `crates/sf_attributes/tests/attribute_test.rs` - Tests

---

### 3.3 Plugin System

**Problem:** Users need custom workflows and integrations.

**Solution:** Multi-language plugin system (Rust + Python + Lua).

**Architecture:**
```
┌─────────────────────────────────────┐
│         Plugin Manager              │
├─────────────────────────────────────┤
│  Rust Plugin  │  Python Plugin      │
│  (native)     │  (PyO3 binding)     │
├─────────────────────────────────────┤
│         Plugin API (trait)          │
└─────────────────────────────────────┘
```

**Plugin API:**
```rust
#[plugin_api]
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn description(&self) -> &str;
    
    // Lifecycle
    fn initialize(&mut self, ctx: PluginContext) -> Result<()>;
    fn shutdown(&mut self) -> Result<()>;
    
    // Commands
    fn commands(&self) -> Vec<PluginCommand>;
    fn execute(&self, cmd: &str, args: Value) -> Result<Value>;
}
```

**Python Plugin Example:**
```python
from stratforge import Plugin, register_plugin

@register_plugin
class MyPlugin(Plugin):
    def name(self):
        return "My Custom Plugin"
    
    def execute(self, cmd, args):
        if cmd == "process_seismic":
            return self.custom_processing(args["data"])
```

**Plugin Registry:**
- Discover plugins in `plugins/` directory
- Load on startup or on-demand
- Sandboxed execution (optional)

**Files:**
- `crates/sf_plugin/src/lib.rs` - Plugin manager
- `crates/sf_plugin/src/api.rs` - Plugin API trait
- `crates/sf_plugin/src/python.rs` - Python binding (PyO3)
- `plugins/example_python/` - Example Python plugin
- `plugins/example_rust/` - Example Rust plugin

---

### 3.4 Advanced Visualization

**Enhancements over Phase 0:**

| Feature | Phase 0 | Phase 1 |
|---------|---------|---------|
| Volume rendering | 2D slices | 3D ray-casting |
| Colormaps | Fixed (6) | Custom (unlimited) |
| Lighting | None | Phong shading |
| VR/AR | ❌ | ✅ (OpenXR) |
| Multi-volume | RGB blend | Unlimited layers |

**3D Volume Rendering:**
```rust
pub struct VolumeRenderer {
    shader: wgpu::ShaderModule,
    transfer_function: TransferFunction,
    sampling_rate: f32,
}

impl VolumeRenderer {
    pub fn render(&self, volume: &Volume, camera: &Camera) -> Texture {
        // Ray-casting shader
        // GPU-accelerated
    }
}
```

**Custom Colormaps:**
```rust
pub struct Colormap {
    name: String,
    control_points: Vec<(f32, [f32; 4])>, // position, RGBA
}

impl Colormap {
    pub fn load_from_file(path: &Path) -> Result<Self>;
    pub fn sample(&self, t: f32) -> [f32; 4];
}

// Presets: Viridis, Plasma, Inferno, Seismic, etc.
```

**Files:**
- `crates/sf_render/src/volume.rs` - Volume renderer
- `crates/sf_render/src/colormap.rs` - Colormap system
- `crates/sf_render/src/shaders/volume.wgsl` - Volume shader
- `crates/sf_render/tests/render_test.rs` - Visual tests

---

## 4. Implementation Plan

### 4.1 Task Breakdown

| Task | Crate | Effort | Priority |
|------|-------|--------|----------|
| **ML Auto-Tracking** | sf_ml | 3 weeks | 🔴 High |
| **Amplitude Attributes** | sf_attributes | 1 week | 🔴 High |
| **Frequency Attributes** | sf_attributes | 1 week | 🟡 Medium |
| **Geometric Attributes** | sf_attributes | 2 weeks | 🟡 Medium |
| **Plugin System Core** | sf_plugin | 2 weeks | 🔴 High |
| **Python Binding** | sf_plugin | 1 week | 🟡 Medium |
| **3D Volume Rendering** | sf_render | 3 weeks | 🟡 Medium |
| **Custom Colormaps** | sf_render | 1 week | 🟢 Low |

**Total Effort:** ~12 weeks (3 months)

### 4.2 Milestones

| Milestone | Target | Deliverables |
|-----------|--------|--------------|
| **M1: ML Foundation** | Week 3 | sf_ml crate, CNN model, training pipeline |
| **M2: Attributes v1** | Week 5 | 50 attributes (amplitude + frequency) |
| **M3: Plugin Core** | Week 7 | Plugin manager, Rust plugin support |
| **M4: Python Plugins** | Week 8 | PyO3 binding, example plugins |
| **M5: Attributes v2** | Week 10 | 100+ attributes complete |
| **M6: Volume Rendering** | Week 12 | 3D ray-casting, custom colormaps |

---

## 5. Technical Challenges

### 5.1 ML Training Data

**Challenge:** Lack of labeled real seismic data.

**Solution:**
1. Generate synthetic data with Phase 0 tools
2. Augment with noise, faults, channels
3. Use transfer learning from public datasets (F3, Netherlands)

### 5.2 Performance

**Challenge:** 100+ attributes on large volumes.

**Solution:**
- GPU acceleration (wgpu compute shaders)
- Lazy evaluation (compute on demand)
- Caching (memoization)

### 5.3 Plugin Security

**Challenge:** Untrusted plugins.

**Solution:**
- Sandboxed execution (WebAssembly for untrusted plugins)
- Permission system (file access, network)
- Code signing for verified plugins

---

## 6. Testing Strategy

### 6.1 Unit Tests

- Each attribute: 10+ test cases
- ML model: Accuracy tests (>80%)
- Plugin API: Lifecycle, error handling

### 6.2 Integration Tests

- End-to-end auto-tracking workflow
- Multi-attribute computation
- Plugin loading and execution

### 6.3 Performance Tests

- Attribute computation time (<1s per volume)
- ML inference latency (<100ms per patch)
- Volume rendering FPS (>30 FPS)

---

## 7. Dependencies

### 7.1 New External Crates

```toml
[dependencies]
# ML
candle-core = "0.3"
candle-nn = "0.3"
ort = "2.0"  # ONNX Runtime

# Attributes
ndarray = "0.15"
rustfft = "6.1"

# Plugins
mlua = "0.9"  # Lua
pyo3 = "0.20" # Python

# Visualization
imgui = "0.11"
openxr = "0.17" # VR/AR
```

### 7.2 System Dependencies

| Platform | Dependencies |
|----------|--------------|
| **Windows** | Python 3.8+, Visual Studio Build Tools |
| **Linux** | python3-dev, liblua5.4-dev |
| **macOS** | Python 3.8+, Lua |

---

## 8. Documentation

### 8.1 User Guides

- ML auto-tracking tutorial
- Seismic attributes reference
- Plugin development guide
- Volume rendering best practices

### 8.2 API Documentation

- Complete rustdoc for new crates
- Python API reference
- Plugin API specification

### 8.3 Examples

- Example Python plugins
- Example Rust plugins
- Attribute computation notebooks

---

## 9. Risks & Mitigations

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| ML accuracy <80% | High | Medium | Fallback to traditional auto-tracking |
| Plugin system too complex | Medium | Medium | Start with Lua only, add Python later |
| Performance issues | High | Low | GPU acceleration, profiling |
| PyO3 compatibility | Medium | Low | Test on all platforms early |

---

## 10. Success Metrics

### 10.1 Technical Metrics

- [ ] ML auto-tracking accuracy >80%
- [ ] 100+ seismic attributes
- [ ] Plugin system supports Rust + Python
- [ ] 3D volume rendering >30 FPS
- [ ] 200+ tests passing

### 10.2 User Metrics

- [ ] Auto-tracking 10x faster than manual
- [ ] Plugin marketplace with 10+ plugins
- [ ] Documentation covers all features
- [ ] Tutorial videos for each feature

---

**Document Version:** 0.1 (Draft)  
**Status:** 📋 Pending Review  
**Next Step:** Create implementation plan for Task 1 (ML Auto-Tracking)
