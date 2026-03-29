//! Misfit Functions for FWI

use ndarray::Array2;

/// Misfit Function Trait
pub trait MisfitFunction: Send + Sync {
    fn compute(&self, predicted: &Array2<f32>, observed: &Array2<f32>) -> f32;
    fn gradient(&self, predicted: &Array2<f32>, observed: &Array2<f32>) -> Array2<f32>;
}

/// L2 Norm Misfit (Least Squares)
pub struct L2Misfit;

impl MisfitFunction for L2Misfit {
    fn compute(&self, predicted: &Array2<f32>, observed: &Array2<f32>) -> f32 {
        let mut misfit = 0.0f32;
        for i in 0..predicted.len() {
            let diff = predicted.as_slice().unwrap()[i] - observed.as_slice().unwrap()[i];
            misfit += diff * diff;
        }
        misfit / 2.0
    }
    
    fn gradient(&self, predicted: &Array2<f32>, observed: &Array2<f32>) -> Array2<f32> {
        // Gradient of L2: d/dm = (predicted - observed)
        let mut grad = Array2::zeros(predicted.dim());
        for i in 0..predicted.len() {
            grad.as_slice_mut().unwrap()[i] = 
                predicted.as_slice().unwrap()[i] - observed.as_slice().unwrap()[i];
        }
        grad
    }
}

/// Travel-time Misfit (Phase-only)
pub struct TravelTimeMisfit;

impl MisfitFunction for TravelTimeMisfit {
    fn compute(&self, predicted: &Array2<f32>, observed: &Array2<f32>) -> f32 {
        // Simplified: pick first arrival times
        let t_pred = self.pick_first_arrival(predicted);
        let t_obs = self.pick_first_arrival(observed);
        
        0.5 * (t_pred - t_obs).powi(2)
    }
    
    fn gradient(&self, _predicted: &Array2<f32>, _observed: &Array2<f32>) -> Array2<f32> {
        // TODO: Implement travel-time gradient
        Array2::zeros((1, 1))
    }
}

impl TravelTimeMisfit {
    fn pick_first_arrival(&self, data: &Array2<f32>) -> f32 {
        // Simple threshold picking
        let threshold = 0.1;
        for (t, &val) in data.iter().enumerate() {
            if val.abs() > threshold {
                return t as f32;
            }
        }
        data.len() as f32
    }
}

/// Waveform-Phase Misfit (instantaneous phase)
pub struct PhaseMisfit;

impl MisfitFunction for PhaseMisfit {
    fn compute(&self, predicted: &Array2<f32>, observed: &Array2<f32>) -> f32 {
        // Compute instantaneous phase using Hilbert transform
        // Simplified for now
        0.0
    }
    
    fn gradient(&self, _predicted: &Array2<f32>, _observed: &Array2<f32>) -> Array2<f32> {
        Array2::zeros((1, 1))
    }
}

/// Envelope Misfit (amplitude-only)
pub struct EnvelopeMisfit;

impl MisfitFunction for EnvelopeMisfit {
    fn compute(&self, predicted: &Array2<f32>, observed: &Array2<f32>) -> f32 {
        // Compute envelope using Hilbert transform
        let mut misfit = 0.0f32;
        for i in 0..predicted.len() {
            let env_pred = predicted.as_slice().unwrap()[i].abs();
            let env_obs = observed.as_slice().unwrap()[i].abs();
            misfit += (env_pred - env_obs).powi(2);
        }
        misfit / 2.0
    }
    
    fn gradient(&self, predicted: &Array2<f32>, observed: &Array2<f32>) -> Array2<f32> {
        let mut grad = Array2::zeros(predicted.dim());
        for i in 0..predicted.len() {
            let env_pred = predicted.as_slice().unwrap()[i].abs();
            let env_obs = observed.as_slice().unwrap()[i].abs();
            grad.as_slice_mut().unwrap()[i] = env_pred - env_obs;
        }
        grad
    }
}

/// Multi-objective Misfit (combines multiple misfits)
pub struct MultiMisfit {
    l2_weight: f32,
    travel_time_weight: f32,
    phase_weight: f32,
}

impl MultiMisfit {
    pub fn new(l2_weight: f32, travel_time_weight: f32, phase_weight: f32) -> Self {
        Self {
            l2_weight,
            travel_time_weight,
            phase_weight,
        }
    }
}

impl MisfitFunction for MultiMisfit {
    fn compute(&self, predicted: &Array2<f32>, observed: &Array2<f32>) -> f32 {
        let l2 = L2Misfit.compute(predicted, observed);
        let tt = TravelTimeMisfit.compute(predicted, observed);
        let phase = PhaseMisfit.compute(predicted, observed);
        
        self.l2_weight * l2 + self.travel_time_weight * tt + self.phase_weight * phase
    }
    
    fn gradient(&self, predicted: &Array2<f32>, observed: &Array2<f32>) -> Array2<f32> {
        let grad_l2 = L2Misfit.gradient(predicted, observed);
        let grad_tt = TravelTimeMisfit.gradient(predicted, observed);
        let grad_phase = PhaseMisfit.gradient(predicted, observed);
        
        (&grad_l2 * self.l2_weight) + (&grad_tt * self.travel_time_weight) + (&grad_phase * self.phase_weight)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::Array2;

    #[test]
    fn test_l2_misfit() {
        let predicted = Array2::from_vec(vec![1.0, 2.0, 3.0, 4.0]);
        let observed = Array2::from_vec(vec![1.1, 2.1, 3.1, 4.1]);
        
        let misfit = L2Misfit;
        let value = misfit.compute(&predicted, &observed);
        
        assert!(value > 0.0);
        assert!(value < 1.0);
    }

    #[test]
    fn test_l2_gradient() {
        let predicted = Array2::from_vec(vec![1.0, 2.0, 3.0]);
        let observed = Array2::from_vec(vec![0.0, 0.0, 0.0]);
        
        let misfit = L2Misfit;
        let grad = misfit.gradient(&predicted, &observed);
        
        assert_eq!(grad.len(), 3);
        assert!(grad.iter().all(|&x| x > 0.0));
    }

    #[test]
    fn test_envelope_misfit() {
        let predicted = Array2::from_vec(vec![1.0, -2.0, 3.0]);
        let observed = Array2::from_vec(vec![1.0, -2.0, 3.0]);
        
        let misfit = EnvelopeMisfit;
        let value = misfit.compute(&predicted, &observed);
        
        assert!((value - 0.0).abs() < 1e-6); // Perfect match
    }

    #[test]
    fn test_multi_misfit() {
        let predicted = Array2::from_vec(vec![1.0, 2.0, 3.0]);
        let observed = Array2::from_vec(vec![1.1, 2.1, 3.1]);
        
        let misfit = MultiMisfit::new(1.0, 0.1, 0.1);
        let value = misfit.compute(&predicted, &observed);
        
        assert!(value > 0.0);
    }
}
