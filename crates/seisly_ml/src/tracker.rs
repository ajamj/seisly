//! Auto-Tracking Engine
//!
//! Uses CNN to predict horizon position from seismic patches.

use std::collections::{HashSet, VecDeque};

use crate::cnn::HorizonCNN;
use candle_core::{Device, Tensor};
use seisly_core::domain::surface::{Mesh, Surface};
use seisly_core::Crs;
use seisly_compute::seismic::TraceProvider;

pub struct AutoTracker {
    #[allow(dead_code)]
    model: HorizonCNN,
    patch_size: usize,
}

impl AutoTracker {
    pub fn new(model: HorizonCNN) -> Self {
        Self {
            model,
            patch_size: 64,
        }
    }

    /// Get patch size
    pub fn patch_size(&self) -> usize {
        self.patch_size
    }

    /// Track horizon from seed point using CNN
    pub fn track(
        &self,
        seismic: &dyn TraceProvider,
        seed_il: i32,
        seed_xl: i32,
        seed_sample: i32,
    ) -> Result<Surface, String> {
        let mut picks: Vec<(i32, i32, f32)> = Vec::new();
        let mut queue: VecDeque<(i32, i32, f32)> = VecDeque::new();

        // Add seed point
        queue.push_back((seed_il, seed_xl, seed_sample as f32));
        let mut visited: HashSet<(i32, i32)> = HashSet::new();

        let (il_min, il_max) = seismic.inline_range();
        let (xl_min, xl_max) = seismic.crossline_range();

        while let Some((il, xl, twt)) = queue.pop_front() {
            if visited.contains(&(il, xl)) {
                continue;
            }
            visited.insert((il, xl));

            // Extract patch and predict
            let patch = self
                .extract_patch(seismic, il, xl, twt as i32)
                .unwrap_or_else(|_| self.create_zero_patch());
            let offset = self.predict_horizon_offset(&patch).unwrap_or(0.0);

            let new_twt = twt + offset;
            picks.push((il, xl, new_twt));

            // Add neighbors to queue (4-connectivity)
            for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let ni = il.wrapping_add(di);
                let nj = xl.wrapping_add(dj);

                // Check bounds
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

    /// Predict horizon offset using CNN
    fn predict_horizon_offset(&self, patch: &Tensor) -> Result<f32, String> {
        let output = self.model.forward(patch).map_err(|e| e.to_string())?;

        // Extract scalar value from output tensor
        let offset = output
            .flatten_all()
            .map_err(|e| e.to_string())?
            .to_vec1::<f32>()
            .map_err(|e| e.to_string())?
            .first()
            .copied()
            .unwrap_or(0.0);

        Ok(offset)
    }

    /// Convert horizon picks to Surface
    fn picks_to_surface(&self, picks: &[(i32, i32, f32)]) -> Surface {
        // Convert picks to mesh vertices
        let vertices: Vec<[f32; 3]> = picks
            .iter()
            .map(|(il, xl, twt)| [*il as f32, *xl as f32, *twt])
            .collect();

        // Create mesh from vertices (no indices for point cloud)
        let mesh = Mesh::new(vertices, vec![]);

        Surface::new(
            "AutoTracked Horizon".to_string(),
            Crs::wgs84(),
            vec![mesh],
        )
    }

    /// Create a zero-filled patch for error cases
    fn create_zero_patch(&self) -> Tensor {
        Tensor::zeros(
            (1, 1, self.patch_size, self.patch_size),
            candle_core::DType::F32,
            &Device::Cpu,
        )
        .unwrap()
    }

    /// Extract seismic patch around point
    fn extract_patch(
        &self,
        seismic: &dyn TraceProvider,
        il: i32,
        xl: i32,
        sample_idx: i32,
    ) -> Result<Tensor, String> {
        let half_patch = self.patch_size / 2;
        let (il_min, il_max) = seismic.inline_range();
        let (xl_min, xl_max) = seismic.crossline_range();
        let sample_count = seismic.sample_count();

        let mut patch_data = Vec::with_capacity(self.patch_size * self.patch_size);

        for di in 0..self.patch_size {
            let i = il.saturating_sub(half_patch as i32).saturating_add(di as i32);
            let i = i.clamp(il_min, il_max);
            
            for dj in 0..self.patch_size {
                let j = xl.saturating_sub(half_patch as i32).saturating_add(dj as i32);
                let j = j.clamp(xl_min, xl_max);
                
                let trace = seismic.get_trace(i, j)
                    .unwrap_or_else(|| vec![0.0; sample_count]);
                
                let value = trace.get(sample_idx as usize).copied().unwrap_or(0.0);
                patch_data.push(value);
            }
        }

        Tensor::from_vec(
            patch_data,
            (1, 1, self.patch_size, self.patch_size),
            &Device::Cpu,
        )
        .map_err(|e| e.to_string())
    }
}
