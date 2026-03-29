//! Amplitude-Based Seismic Attributes
//!
//! Implements 10 amplitude attributes:
//! 1. RMS Amplitude
//! 2. Mean Amplitude
//! 3. Max Amplitude
//! 4. Min Amplitude
//! 5. Standard Deviation
//! 6. Energy
//! 7. Average Energy
//! 8. Absolute Amplitude
//! 9. Max Absolute
//! 10. Skewness

use crate::trait_def::SeismicAttribute;

/// Root Mean Square (RMS) Amplitude
pub struct RmsAmplitude;

impl SeismicAttribute for RmsAmplitude {
    fn name(&self) -> &'static str { "RMS Amplitude" }
    
    fn compute(&self, trace: &[f32], window_size: usize) -> Vec<f32> {
        trace
            .windows(window_size)
            .map(|w| {
                let sum_squares: f32 = w.iter().map(|x| x * x).sum();
                (sum_squares / w.len() as f32).sqrt()
            })
            .collect()
    }
}

/// Mean Amplitude
pub struct MeanAmplitude;

impl SeismicAttribute for MeanAmplitude {
    fn name(&self) -> &'static str { "Mean Amplitude" }
    
    fn compute(&self, trace: &[f32], window_size: usize) -> Vec<f32> {
        trace
            .windows(window_size)
            .map(|w| w.iter().sum::<f32>() / w.len() as f32)
            .collect()
    }
}

/// Max Amplitude
pub struct MaxAmplitude;

impl SeismicAttribute for MaxAmplitude {
    fn name(&self) -> &'static str { "Max Amplitude" }
    
    fn compute(&self, trace: &[f32], window_size: usize) -> Vec<f32> {
        trace
            .windows(window_size)
            .map(|w| w.iter().cloned().fold(f32::NEG_INFINITY, f32::max))
            .collect()
    }
}

/// Min Amplitude
pub struct MinAmplitude;

impl SeismicAttribute for MinAmplitude {
    fn name(&self) -> &'static str { "Min Amplitude" }
    
    fn compute(&self, trace: &[f32], window_size: usize) -> Vec<f32> {
        trace
            .windows(window_size)
            .map(|w| w.iter().cloned().fold(f32::INFINITY, f32::min))
            .collect()
    }
}

/// Standard Deviation
pub struct StdDevAmplitude;

impl SeismicAttribute for StdDevAmplitude {
    fn name(&self) -> &'static str { "Standard Deviation" }
    
    fn compute(&self, trace: &[f32], window_size: usize) -> Vec<f32> {
        trace
            .windows(window_size)
            .map(|w| {
                let mean = w.iter().sum::<f32>() / w.len() as f32;
                let variance = w.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / w.len() as f32;
                variance.sqrt()
            })
            .collect()
    }
}

/// Energy (Sum of Squares)
pub struct EnergyAmplitude;

impl SeismicAttribute for EnergyAmplitude {
    fn name(&self) -> &'static str { "Energy" }
    
    fn compute(&self, trace: &[f32], window_size: usize) -> Vec<f32> {
        trace
            .windows(window_size)
            .map(|w| w.iter().map(|x| x * x).sum())
            .collect()
    }
}

/// Average Energy
pub struct AverageEnergy;

impl SeismicAttribute for AverageEnergy {
    fn name(&self) -> &'static str { "Average Energy" }
    
    fn compute(&self, trace: &[f32], window_size: usize) -> Vec<f32> {
        trace
            .windows(window_size)
            .map(|w| {
                let sum_squares: f32 = w.iter().map(|x| x * x).sum();
                sum_squares / w.len() as f32
            })
            .collect()
    }
}

/// Absolute Amplitude (Sum of Absolute Values)
pub struct AbsoluteAmplitude;

impl SeismicAttribute for AbsoluteAmplitude {
    fn name(&self) -> &'static str { "Absolute Amplitude" }
    
    fn compute(&self, trace: &[f32], window_size: usize) -> Vec<f32> {
        trace
            .windows(window_size)
            .map(|w| w.iter().map(|x| x.abs()).sum())
            .collect()
    }
}

/// Max Absolute Amplitude
pub struct MaxAbsoluteAmplitude;

impl SeismicAttribute for MaxAbsoluteAmplitude {
    fn name(&self) -> &'static str { "Max Absolute" }
    
    fn compute(&self, trace: &[f32], window_size: usize) -> Vec<f32> {
        trace
            .windows(window_size)
            .map(|w| w.iter().map(|x| x.abs()).fold(f32::NEG_INFINITY, f32::max))
            .collect()
    }
}

/// Skewness (Asymmetry Measure)
pub struct SkewnessAmplitude;

impl SeismicAttribute for SkewnessAmplitude {
    fn name(&self) -> &'static str { "Skewness" }
    
    fn compute(&self, trace: &[f32], window_size: usize) -> Vec<f32> {
        trace
            .windows(window_size)
            .map(|w| {
                let n = w.len() as f32;
                let mean = w.iter().sum::<f32>() / n;
                let variance = w.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / n;
                let std_dev = variance.sqrt();
                
                if std_dev < 1e-10 {
                    return 0.0;
                }
                
                let skew = w.iter()
                    .map(|x| ((x - mean) / std_dev).powi(3))
                    .sum::<f32>() / n;
                
                skew
            })
            .collect()
    }
}

/// Get all amplitude attributes
pub fn all_amplitude_attributes() -> Vec<Box<dyn SeismicAttribute>> {
    vec![
        Box::new(RmsAmplitude),
        Box::new(MeanAmplitude),
        Box::new(MaxAmplitude),
        Box::new(MinAmplitude),
        Box::new(StdDevAmplitude),
        Box::new(EnergyAmplitude),
        Box::new(AverageEnergy),
        Box::new(AbsoluteAmplitude),
        Box::new(MaxAbsoluteAmplitude),
        Box::new(SkewnessAmplitude),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rms_amplitude() {
        let attr = RmsAmplitude;
        let trace = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = attr.compute(&trace, 3);
        
        assert_eq!(result.len(), 3);
        // RMS of [1,2,3] = sqrt((1+4+9)/3) = sqrt(4.67) ≈ 2.16
        assert!((result[0] - 2.16).abs() < 0.1);
    }

    #[test]
    fn test_mean_amplitude() {
        let attr = MeanAmplitude;
        let trace = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = attr.compute(&trace, 3);
        
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], 2.0); // Mean of [1,2,3]
    }

    #[test]
    fn test_max_amplitude() {
        let attr = MaxAmplitude;
        let trace = vec![1.0, 5.0, 3.0, 2.0, 4.0];
        let result = attr.compute(&trace, 3);
        
        assert_eq!(result[0], 5.0); // Max of [1,5,3]
    }

    #[test]
    fn test_std_dev() {
        let attr = StdDevAmplitude;
        let trace = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let result = attr.compute(&trace, 4);
        
        assert!(!result.is_empty());
    }
}
