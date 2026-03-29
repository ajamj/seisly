# Auto-Tracking Engine Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Complete the auto-tracking engine by implementing the full tracking algorithm that uses the trained CNN model.

**Architecture:** Implement BFS-based horizon tracking that starts from a seed point, extracts seismic patches, uses CNN to predict horizon offsets, and expands to neighboring traces using 4-connectivity.

**Tech Stack:** Rust, Candle (ML framework), sf_core (domain types), sf_compute (seismic processing)

---

### Task 1: Implement track() method with BFS expansion

**Files:**
- Modify: `crates/sf_ml/src/tracker.rs`

- [ ] **Step 1: Update track() method implementation**

Replace the placeholder track() method with full BFS implementation:

```rust
/// Track horizon from seed point using CNN
pub fn track<P: TraceProvider>(
    &self,
    seismic: &P,
    seed_il: i32,
    seed_xl: i32,
    seed_sample: usize,
) -> Result<Surface, String> {
    use std::collections::{HashSet, VecDeque};
    
    let mut picks: Vec<(i32, i32, f32)> = Vec::new();
    let mut queue: VecDeque<(i32, i32, f32)> = VecDeque::new();
    
    // Add seed point (convert sample index to f32 for offset arithmetic)
    queue.push_back((seed_il, seed_xl, seed_sample as f32));
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    
    while let Some((il, xl, twt)) = queue.pop_front() {
        if visited.contains(&(il, xl)) {
            continue;
        }
        visited.insert((il, xl));
        
        // Extract patch and predict
        let patch = self.extract_patch(seismic, il, xl, twt as usize)?;
        let offset = self.predict_horizon_offset(&patch)?;
        
        let new_twt = twt + offset;
        picks.push((il, xl, new_twt));
        
        // Add neighbors to queue (4-connectivity)
        for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let ni = il.wrapping_add(di);
            let nj = xl.wrapping_add(dj);
            
            // Check bounds
            let (il_min, il_max) = seismic.inline_range();
            let (xl_min, xl_max) = seismic.crossline_range();
            
            if ni >= il_min && ni <= il_max && nj >= xl_min && nj <= xl_max {
                if !visited.contains(&(ni, nj)) {
                    queue.push_back((ni, nj, new_twt));
                }
            }
        }
    }
    
    // Convert picks to Surface
    Ok(self.picks_to_surface(&picks))
}
```

- [ ] **Step 2: Add predict_horizon_offset() helper method**

Add private method after extract_patch():

```rust
/// Predict horizon offset using CNN
fn predict_horizon_offset(&self, patch: &Tensor) -> Result<f32, String> {
    let output = self.model.forward(patch)
        .map_err(|e| e.to_string())?;
    
    // Extract scalar value from output tensor
    let offset = output
        .to_vec1::<f32>()
        .map_err(|e| e.to_string())?
        .first()
        .copied()
        .unwrap_or(0.0);
    
    Ok(offset)
}
```

- [ ] **Step 3: Add picks_to_surface() helper method**

Add private method to convert picks to Surface:

```rust
/// Convert horizon picks to Surface
fn picks_to_surface(&self, picks: &[(i32, i32, f32)]) -> Surface {
    use sf_core::domain::surface::Mesh;
    
    // Convert picks to mesh vertices
    let vertices: Vec<[f32; 3]> = picks
        .iter()
        .map(|(il, xl, twt)| [*il as f32, *xl as f32, *twt])
        .collect();
    
    // Create mesh from vertices (no indices for point cloud)
    let mesh = Mesh::new(vertices, vec![]);
    
    Surface::new(
        "AutoTracked Horizon".to_string(),
        sf_core::Crs::wgs84(),
        vec![mesh],
    )
}
```

- [ ] **Step 4: Run cargo check to verify compilation**

```bash
cd D:\GRC-Ajam\myfield
cargo check -p sf_ml
```

Expected: No errors

- [ ] **Step 5: Commit**

```bash
git add crates/sf_ml/src/tracker.rs
git commit -m "feat(ml): implement BFS-based auto-tracking algorithm

- Complete track() method with queue-based BFS expansion
- CNN-based horizon offset prediction from seismic patches
- 4-connectivity neighbor expansion with bounds checking
- Surface generation from horizon picks
"
```

---

### Task 2: Create integration tests

**Files:**
- Create: `crates/sf_ml/tests/tracker_integration_test.rs`

- [ ] **Step 1: Create test file with tracker initialization test**

```rust
use sf_ml::cnn::HorizonCNN;
use sf_ml::tracker::AutoTracker;
use candle_core::{Device, Tensor};
use candle_nn::{VarBuilder, VarBuilderArgs};
use sf_compute::seismic::{InMemoryProvider, TraceProvider};

#[test]
fn test_tracker_initialization() {
    let device = Device::Cpu;
    let vb = VarBuilder::zeros(VarBuilderArgs::default(), &device);
    let model = HorizonCNN::new(vb).unwrap();
    let tracker = AutoTracker::new(model);
    
    assert_eq!(tracker.patch_size(), 64);
}

#[test]
fn test_tracker_with_dummy_seismic() {
    let device = Device::Cpu;
    let vb = VarBuilder::zeros(VarBuilderArgs::default(), &device);
    let model = HorizonCNN::new(vb).unwrap();
    let tracker = AutoTracker::new(model);
    
    // Create small dummy seismic volume (10x10x20)
    let inline_range = (0, 9);
    let crossline_range = (0, 9);
    let sample_count = 20;
    let data = vec![0.0f32; 10 * 10 * 20];
    
    let provider = InMemoryProvider {
        data,
        inline_range,
        crossline_range,
        sample_count,
    };
    
    // Test tracking from center seed point
    let result = tracker.track(&provider, 5, 5, 10);
    
    assert!(result.is_ok());
    let surface = result.unwrap();
    
    // Should have tracked at least the seed point
    assert!(!surface.meshes.is_empty());
    assert!(!surface.meshes[0].vertices.is_empty());
}

#[test]
fn test_patch_extraction_bounds() {
    let device = Device::Cpu;
    let vb = VarBuilder::zeros(VarBuilderArgs::default(), &device);
    let model = HorizonCNN::new(vb).unwrap();
    let tracker = AutoTracker::new(model);
    
    // Create small seismic volume
    let inline_range = (0, 5);
    let crossline_range = (0, 5);
    let sample_count = 10;
    let data = vec![1.0f32; 6 * 6 * 10];
    
    let provider = InMemoryProvider {
        data,
        inline_range,
        crossline_range,
        sample_count,
    };
    
    // Test extraction at edge (should not panic, uses saturating arithmetic)
    let patch = tracker.extract_patch(&provider, 0, 0, 5);
    assert!(patch.is_ok());
    
    // Verify patch shape
    let patch = patch.unwrap();
    assert_eq!(patch.dims(), (1, 1, 64, 64));
}
```

- [ ] **Step 2: Run tests**

```bash
cd D:\GRC-Ajam\myfield
cargo test -p sf_ml tracker_integration
```

Expected: All 3 tests pass

- [ ] **Step 3: Commit**

```bash
git add crates/sf_ml/tests/tracker_integration_test.rs
git commit -m "test(ml): add integration tests for auto-tracker

- Test tracker initialization and patch_size accessor
- Test full tracking workflow with dummy seismic data
- Test patch extraction at volume boundaries
"
```

---

### Task 3: Final verification and self-review

**Files:**
- None (verification only)

- [ ] **Step 1: Run full test suite for sf_ml**

```bash
cd D:\GRC-Ajam\myfield
cargo test -p sf_ml
```

Expected: All tests pass

- [ ] **Step 2: Run cargo clippy for linting**

```bash
cd D:\GRC-Ajam\myfield
cargo clippy -p sf_ml -- -D warnings
```

Expected: No warnings

- [ ] **Step 3: Verify git status**

```bash
git status
```

Expected: Only tracker.rs and tracker_integration_test.rs modified/added

- [ ] **Step 4: Review changes**

```bash
git diff HEAD
```

Review for:
- Correct BFS implementation
- Proper error handling
- No unwrap() in production code (except tests)
- Bounds checking for seismic volume
