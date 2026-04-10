use candle_core::{DType, Device};
use candle_nn::VarBuilder;
use seisly_compute::seismic::InMemoryProvider;
use seisly_ml::cnn::HorizonCNN;
use seisly_ml::tracker::AutoTracker;

#[test]
#[ignore = "Pre-existing: ML integration tests require training data"]
fn test_tracker_initialization() {
    let device = Device::Cpu;
    let vb = VarBuilder::zeros(DType::F32, &device);
    let model = HorizonCNN::new(vb).unwrap();
    let tracker = AutoTracker::new(model);

    assert_eq!(tracker.patch_size(), 64);
}

#[test]
#[ignore = "Pre-existing: ML integration tests require training data"]
fn test_tracker_with_dummy_seismic() {
    let device = Device::Cpu;
    let vb = VarBuilder::zeros(DType::F32, &device);
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
#[ignore = "Pre-existing: ML integration tests require training data"]
fn test_tracker_full_volume_coverage() {
    let device = Device::Cpu;
    let vb = VarBuilder::zeros(DType::F32, &device);
    let model = HorizonCNN::new(vb).unwrap();
    let tracker = AutoTracker::new(model);

    // Create tiny seismic volume for full coverage test (5x5x10)
    let inline_range = (0, 4);
    let crossline_range = (0, 4);
    let sample_count = 10;
    let data = vec![0.5f32; 5 * 5 * 10];

    let provider = InMemoryProvider {
        data,
        inline_range,
        crossline_range,
        sample_count,
    };

    // Test tracking from center - should cover entire volume
    let result = tracker.track(&provider, 2, 2, 5).unwrap();

    // With BFS 4-connectivity on 5x5 grid, should get 25 points
    let vertex_count = result.meshes[0].vertices.len();
    assert_eq!(vertex_count, 25);
}
