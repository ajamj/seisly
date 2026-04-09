//! Seismic data traits and core models.

/// Trait for providing seismic traces based on coordinate indexing.
pub trait TraceProvider: Send + Sync {
    /// Retrieve a trace at the specified inline and crossline.
    fn get_trace(&self, inline: i32, xline: i32) -> Option<Vec<f32>>;

    /// Returns the (min, max) inline range.
    fn inline_range(&self) -> (i32, i32);

    /// Returns the (min, max) crossline range.
    fn crossline_range(&self) -> (i32, i32);

    /// Returns the number of samples per trace.
    fn sample_count(&self) -> usize;
}
