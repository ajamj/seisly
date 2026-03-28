//! Velocity State and Depth Conversion Management

use sf_compute::velocity::LinearVelocityModel;

#[derive(Debug, Clone, Copy)]
pub struct VelocityState {
    pub model: LinearVelocityModel,
    pub is_depth_mode: bool,
}

impl VelocityState {
    pub fn new() -> Self {
        Self {
            model: LinearVelocityModel::new(2000.0, 0.5, 4.0, 0.0),
            is_depth_mode: false,
        }
    }

    /// Project a 3D point (Inline, Crossline, Sample) to (Inline, Crossline, Depth)
    pub fn project_to_depth(&self, pos: [f32; 3]) -> [f32; 3] {
        if !self.is_depth_mode {
            return pos;
        }
        [pos[0], pos[1], self.model.sample_to_depth(pos[2])]
    }
}
