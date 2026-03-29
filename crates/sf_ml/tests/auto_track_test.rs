use sf_ml::cnn::HorizonCNN;
use sf_ml::tracker::AutoTracker;
use candle_core::{Device, DType};
use candle_nn::VarBuilder;

#[test]
fn test_cnn_creation() {
    let device = Device::Cpu;
    let vb = VarBuilder::zeros(DType::F32, &device);
    
    let model = HorizonCNN::new(vb);
    assert!(model.is_ok());
}

#[test]
fn test_cnn_forward_pass() {
    let device = Device::Cpu;
    let vb = VarBuilder::zeros(DType::F32, &device);
    
    let model = HorizonCNN::new(vb).unwrap();
    
    // Create dummy input: batch=1, channels=1, 64x64
    let input = candle_core::Tensor::zeros((1, 1, 64, 64), DType::F32, &device).unwrap();
    
    let output = model.forward(&input).unwrap();
    assert_eq!(output.dims()[0], 1); // batch size
    assert_eq!(output.dims()[1], 1); // single output
}

#[test]
fn test_tracker_creation() {
    let device = Device::Cpu;
    let vb = VarBuilder::zeros(DType::F32, &device);
    let model = HorizonCNN::new(vb).unwrap();
    
    let tracker = AutoTracker::new(model);
    assert_eq!(tracker.patch_size(), 64);
}
