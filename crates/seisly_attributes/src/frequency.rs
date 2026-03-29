//! Frequency-Based Seismic Attributes
//!
//! Implements 10 frequency attributes using FFT and Hilbert transform:
//! 1. Instantaneous Frequency
//! 2. Dominant Frequency
//! 3. Peak Frequency
//! 4. Mean Frequency
//! 5. Frequency Bandwidth
//! 6. Spectral Blue/Red
//! 7. Thin Bed Indicator
//! 8. Absorption Factor
//! 9. Wavelet Phase
//! 10. Instantaneous Phase

use crate::trait_def::SeismicAttribute;
use num_complex::Complex;
use rustfft::{FftPlanner, Fft};

/// Instantaneous Frequency (using Hilbert transform)
pub struct InstantaneousFrequency;

impl SeismicAttribute for InstantaneousFrequency {
    fn name(&self) -> &'static str { "Instantaneous Frequency" }
    
    fn compute(&self, trace: &[f32], _window_size: usize) -> Vec<f32> {
        // Simplified: Use Hilbert transform to get instantaneous phase
        // Then derivative of phase = instantaneous frequency
        let analytic = hilbert_transform(trace);
        
        let mut inst_freq = Vec::with_capacity(trace.len());
        for i in 1..analytic.len() {
            let phase_diff = (analytic[i].arg() - analytic[i-1].arg()) as f32;
            inst_freq.push(phase_diff.abs());
        }
        inst_freq.push(0.0); // Pad last sample
        
        inst_freq
    }
}

/// Dominant Frequency
pub struct DominantFrequency;

impl SeismicAttribute for DominantFrequency {
    fn name(&self) -> &'static str { "Dominant Frequency" }
    
    fn compute(&self, trace: &[f32], _window_size: usize) -> Vec<f32> {
        // Compute FFT and find frequency with maximum amplitude
        let spectrum = compute_fft(trace);
        let max_idx = spectrum
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.norm().partial_cmp(&b.norm()).unwrap())
            .map(|(i, _)| i)
            .unwrap_or(0);
        
        vec![max_idx as f32; trace.len()]
    }
}

/// Peak Frequency
pub struct PeakFrequency;

impl SeismicAttribute for PeakFrequency {
    fn name(&self) -> &'static str { "Peak Frequency" }
    
    fn compute(&self, trace: &[f32], _window_size: usize) -> Vec<f32> {
        // Similar to dominant frequency
        let spectrum = compute_fft(trace);
        let peak = spectrum.iter().map(|c| c.norm()).fold(f32::NEG_INFINITY, f32::max);
        vec![peak; trace.len()]
    }
}

/// Mean Frequency
pub struct MeanFrequency;

impl SeismicAttribute for MeanFrequency {
    fn name(&self) -> &'static str { "Mean Frequency" }
    
    fn compute(&self, trace: &[f32], _window_size: usize) -> Vec<f32> {
        let spectrum = compute_fft(trace);
        let total_energy: f32 = spectrum.iter().map(|c| c.norm()).sum();
        let weighted_sum: f32 = spectrum
            .iter()
            .enumerate()
            .map(|(i, c)| i as f32 * c.norm())
            .sum();
        
        let mean_freq = if total_energy > 0.0 {
            weighted_sum / total_energy
        } else {
            0.0
        };
        
        vec![mean_freq; trace.len()]
    }
}

/// Frequency Bandwidth
pub struct FrequencyBandwidth;

impl SeismicAttribute for FrequencyBandwidth {
    fn name(&self) -> &'static str { "Frequency Bandwidth" }
    
    fn compute(&self, trace: &[f32], _window_size: usize) -> Vec<f32> {
        let spectrum = compute_fft(trace);
        let energies: Vec<f32> = spectrum.iter().map(|c| c.norm()).collect();
        
        // Calculate standard deviation of frequency
        let mean = energies.iter().sum::<f32>() / energies.len() as f32;
        let variance = energies.iter().map(|e| (e - mean).powi(2)).sum::<f32>() / energies.len() as f32;
        
        vec![variance.sqrt(); trace.len()]
    }
}

/// Spectral Blue/Red (Low/High Frequency Ratio)
pub struct SpectralBlueRed;

impl SeismicAttribute for SpectralBlueRed {
    fn name(&self) -> &'static str { "Spectral Blue/Red" }
    
    fn compute(&self, trace: &[f32], _window_size: usize) -> Vec<f32> {
        let spectrum = compute_fft(trace);
        let mid = spectrum.len() / 2;
        
        let low_freq_energy: f32 = spectrum[..mid].iter().map(|c| c.norm()).sum();
        let high_freq_energy: f32 = spectrum[mid..].iter().map(|c| c.norm()).sum();
        
        let ratio = if high_freq_energy > 0.0 {
            low_freq_energy / high_freq_energy
        } else {
            0.0
        };
        
        vec![ratio; trace.len()]
    }
}

/// Thin Bed Indicator
pub struct ThinBedIndicator;

impl SeismicAttribute for ThinBedIndicator {
    fn name(&self) -> &'static str { "Thin Bed Indicator" }
    
    fn compute(&self, trace: &[f32], _window_size: usize) -> Vec<f32> {
        // Thin beds cause frequency attenuation - measure high freq loss
        let spectrum = compute_fft(trace);
        let high_freq_energy: f32 = spectrum[spectrum.len()*3/4..].iter().map(|c| c.norm()).sum();
        let total_energy: f32 = spectrum.iter().map(|c| c.norm()).sum();
        
        let indicator = if total_energy > 0.0 {
            1.0 - (high_freq_energy / total_energy)
        } else {
            0.0
        };
        
        vec![indicator; trace.len()]
    }
}

/// Absorption Factor (Q-factor estimation)
pub struct AbsorptionFactor;

impl SeismicAttribute for AbsorptionFactor {
    fn name(&self) -> &'static str { "Absorption Factor" }
    
    fn compute(&self, trace: &[f32], _window_size: usize) -> Vec<f32> {
        // Simplified Q-factor estimation from spectral ratio
        let spectrum = compute_fft(trace);
        let low_freq = spectrum[1..4].iter().map(|c| c.norm()).sum::<f32>() / 3.0;
        let high_freq = spectrum[spectrum.len()/2..spectrum.len()/2+3]
            .iter()
            .map(|c| c.norm())
            .sum::<f32>() / 3.0;
        
        let q_factor = if high_freq > 0.0 {
            (low_freq / high_freq).ln().abs()
        } else {
            0.0
        };
        
        vec![q_factor; trace.len()]
    }
}

/// Wavelet Phase
pub struct WaveletPhase;

impl SeismicAttribute for WaveletPhase {
    fn name(&self) -> &'static str { "Wavelet Phase" }
    
    fn compute(&self, trace: &[f32], _window_size: usize) -> Vec<f32> {
        let analytic = hilbert_transform(trace);
        analytic.iter().map(|c| c.arg() as f32).collect()
    }
}

/// Instantaneous Phase
pub struct InstantaneousPhase;

impl SeismicAttribute for InstantaneousPhase {
    fn name(&self) -> &'static str { "Instantaneous Phase" }
    
    fn compute(&self, trace: &[f32], _window_size: usize) -> Vec<f32> {
        // Same as wavelet phase for now
        let analytic = hilbert_transform(trace);
        analytic.iter().map(|c| c.arg() as f32).collect()
    }
}

// Helper functions

fn hilbert_transform(trace: &[f32]) -> Vec<Complex<f32>> {
    // Simplified Hilbert transform using FFT
    let n = trace.len();
    let mut complex_trace: Vec<Complex<f32>> = trace.iter().map(|&x| Complex::new(x, 0.0)).collect();
    
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(n);
    fft.process(&mut complex_trace);
    
    // Apply Hilbert transform in frequency domain
    for i in 0..n {
        if i == 0 || i == n / 2 {
            complex_trace[i] *= Complex::new(1.0, 0.0);
        } else if i < n / 2 {
            complex_trace[i] *= Complex::new(0.0, -2.0);
        } else {
            complex_trace[i] = Complex::new(0.0, 0.0);
        }
    }
    
    let ifft = planner.plan_fft_inverse(n);
    ifft.process(&mut complex_trace);
    
    complex_trace.iter().map(|c| c / n as f32).collect()
}

fn compute_fft(trace: &[f32]) -> Vec<Complex<f32>> {
    let n = trace.len();
    let mut complex_trace: Vec<Complex<f32>> = trace.iter().map(|&x| Complex::new(x, 0.0)).collect();
    
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(n);
    fft.process(&mut complex_trace);
    
    complex_trace.iter().map(|c| c / n as f32).collect()
}

/// Get all frequency attributes
pub fn all_frequency_attributes() -> Vec<Box<dyn SeismicAttribute>> {
    vec![
        Box::new(InstantaneousFrequency),
        Box::new(DominantFrequency),
        Box::new(PeakFrequency),
        Box::new(MeanFrequency),
        Box::new(FrequencyBandwidth),
        Box::new(SpectralBlueRed),
        Box::new(ThinBedIndicator),
        Box::new(AbsorptionFactor),
        Box::new(WaveletPhase),
        Box::new(InstantaneousPhase),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instantaneous_frequency() {
        let attr = InstantaneousFrequency;
        let trace = vec![1.0, 0.0, -1.0, 0.0, 1.0, 0.0, -1.0, 0.0];
        let result = attr.compute(&trace, 8);
        
        assert_eq!(result.len(), 8);
        assert!(result.iter().all(|&x| x >= 0.0));
    }

    #[test]
    fn test_dominant_frequency() {
        let attr = DominantFrequency;
        let trace = vec![1.0, 2.0, 3.0, 4.0, 5.0, 4.0, 3.0, 2.0];
        let result = attr.compute(&trace, 8);
        
        assert_eq!(result.len(), 8);
    }

    #[test]
    fn test_spectral_blue_red() {
        let attr = SpectralBlueRed;
        let trace = vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
        let result = attr.compute(&trace, 8);
        
        assert_eq!(result.len(), 8);
    }
}
