use moka::sync::Cache;
use std::sync::Arc;

/// A thread-safe LRU cache for seismic data bricks (traces).
pub struct BrickCache {
    cache: Cache<usize, Arc<Vec<f32>>>,
}

impl BrickCache {
    /// Create a new cache with the specified maximum capacity (number of bricks).
    pub fn new(max_capacity: u64) -> Self {
        let cache = Cache::builder()
            .max_capacity(max_capacity)
            .build();
        Self { cache }
    }

    /// Retrieve a brick from the cache.
    pub fn get(&self, index: usize) -> Option<Arc<Vec<f32>>> {
        self.cache.get(&index)
    }

    /// Insert a brick into the cache.
    pub fn insert(&self, index: usize, data: Arc<Vec<f32>>) {
        self.cache.insert(index, data);
    }

    /// Invalidate all entries in the cache.
    pub fn clear(&self) {
        self.cache.invalidate_all();
    }
}

impl Default for BrickCache {
    fn default() -> Self {
        // Default capacity: 10,000 traces (roughly 40MB if each trace is 1000 samples)
        Self::new(10_000)
    }
}
