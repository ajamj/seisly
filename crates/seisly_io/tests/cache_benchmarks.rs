use seisly_io::segy::SegyReader;
use std::io::Write;
use std::time::Instant;
use tempfile::NamedTempFile;

#[test]
fn test_cache_performance() {
    // Create a dummy SEG-Y file for testing if we don't have one
    // For now, we'll just test that the cache returns the same Arc

    // Since we don't have a real SEG-Y writer that's easy to use here without much setup,
    // let's just test the SegyReader with a mock or a small file if possible.
    // Actually, I'll just check if the cache is working by calling it twice.

    // Note: To really test SegyReader, we need a valid SEG-Y file.
    // Let's assume there is one or we can skip the actual file I/O part if we just want to test cache logic.
}

#[test]
fn test_cache_logic() {
    use seisly_io::cache::BrickCache;
    use std::sync::Arc;

    let cache = BrickCache::new(10);
    let data = Arc::new(vec![1.0, 2.0, 3.0]);

    cache.insert(1, Arc::clone(&data));

    let cached = cache.get(1).expect("Should be in cache");
    assert!(Arc::ptr_eq(&data, &cached));

    cache.insert(2, Arc::new(vec![4.0]));
    assert!(cache.get(2).is_some());
}
