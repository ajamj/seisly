//! Seismic Attributes Integration Tests

use sf_attributes::{
    SeismicAttribute,
    amplitude::{RmsAmplitude, MeanAmplitude, all_amplitude_attributes},
    frequency::{InstantaneousFrequency, DominantFrequency, all_frequency_attributes},
};

#[test]
fn test_all_amplitude_attributes_count() {
    let attrs = all_amplitude_attributes();
    assert_eq!(attrs.len(), 10);
}

#[test]
fn test_all_frequency_attributes_count() {
    let attrs = all_frequency_attributes();
    assert_eq!(attrs.len(), 10);
}

#[test]
fn test_amplitude_attribute_names() {
    let attrs = all_amplitude_attributes();
    let names: Vec<&str> = attrs.iter().map(|a| a.name()).collect();
    
    assert!(names.contains(&"RMS Amplitude"));
    assert!(names.contains(&"Mean Amplitude"));
    assert!(names.contains(&"Max Amplitude"));
    assert!(names.contains(&"Min Amplitude"));
    assert!(names.contains(&"Standard Deviation"));
    assert!(names.contains(&"Energy"));
    assert!(names.contains(&"Average Energy"));
    assert!(names.contains(&"Absolute Amplitude"));
    assert!(names.contains(&"Max Absolute"));
    assert!(names.contains(&"Skewness"));
}

#[test]
fn test_frequency_attribute_names() {
    let attrs = all_frequency_attributes();
    let names: Vec<&str> = attrs.iter().map(|a| a.name()).collect();
    
    assert!(names.contains(&"Instantaneous Frequency"));
    assert!(names.contains(&"Dominant Frequency"));
    assert!(names.contains(&"Peak Frequency"));
    assert!(names.contains(&"Mean Frequency"));
    assert!(names.contains(&"Frequency Bandwidth"));
    assert!(names.contains(&"Spectral Blue/Red"));
    assert!(names.contains(&"Thin Bed Indicator"));
    assert!(names.contains(&"Absorption Factor"));
    assert!(names.contains(&"Wavelet Phase"));
    assert!(names.contains(&"Instantaneous Phase"));
}

#[test]
fn test_rms_vs_mean() {
    // RMS should always be >= mean for real signals
    let rms = RmsAmplitude;
    let mean = MeanAmplitude;
    let trace = vec![1.0, -2.0, 3.0, -4.0, 5.0];
    
    let rms_result = rms.compute(&trace, 5);
    let mean_result = mean.compute(&trace, 5);
    
    assert!(rms_result[0] >= mean_result[0].abs());
}

#[test]
fn test_instantaneous_phase_range() {
    let attr = InstantaneousFrequency;
    let trace = vec![1.0, 0.0, -1.0, 0.0, 1.0, 0.0, -1.0, 0.0];
    let result = attr.compute(&trace, 8);
    
    // Phase should be in reasonable range
    assert!(result.iter().all(|&x| x.is_finite()));
}
