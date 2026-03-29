//! Seismic Attribute Trait

/// Trait for all seismic attributes
pub trait SeismicAttribute: Send + Sync {
    /// Attribute name
    fn name(&self) -> &'static str;
    
    /// Compute attribute on a trace
    fn compute(&self, trace: &[f32], window_size: usize) -> Vec<f32>;
}
