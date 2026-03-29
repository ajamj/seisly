//! Gradient Calculation for FWI

use ndarray::Array2;

/// Gradient Calculator using Adjoint State Method
pub struct GradientCalculator;

impl GradientCalculator {
    /// Compute gradient using adjoint state method
    pub fn compute_adjoint(
        forward_wavefield: &Array2<f32>,
        adjoint_wavefield: &Array2<f32>,
        velocity: &Array2<f32>,
    ) -> Array2<f32> {
        let mut gradient = Array2::zeros(velocity.dim());
        
        // Gradient = -2/v^3 * sum_t(u_fwd * u_adj)
        for iz in 0..velocity.nrows() {
            for ix in 0..velocity.ncols() {
                let v = velocity[(iz, ix)];
                let mut sum = 0.0f32;
                
                // Time summation (simplified - in production use full time loop)
                for t in 0..forward_wavefield.len() {
                    let u_fwd = forward_wavefield.as_slice().unwrap()[t];
                    let u_adj = adjoint_wavefield.as_slice().unwrap()[t];
                    sum += u_fwd * u_adj;
                }
                
                gradient[(iz, ix)] = -2.0 / (v * v * v) * sum;
            }
        }
        
        gradient
    }
    
    /// Apply preconditioning to gradient
    pub fn precondition(gradient: &Array2<f32>, method: Preconditioner) -> Array2<f32> {
        match method {
            Preconditioner::None => gradient.clone(),
            Preconditioner::Smooth(sigma) => Self::smooth(gradient, sigma),
            Preconditioner::Illumination => Self::illumination_precond(gradient),
        }
    }
    
    /// Gaussian smoothing
    fn smooth(gradient: &Array2<f32>, sigma: usize) -> Array2<f32> {
        let mut smoothed = Array2::zeros(gradient.dim());
        let nz = gradient.nrows();
        let nx = gradient.ncols();
        
        for iz in sigma..nz-sigma {
            for ix in sigma..nx-sigma {
                let mut sum = 0.0f32;
                let mut count = 0;
                for di in 0..sigma*2+1 {
                    for dj in 0..sigma*2+1 {
                        sum += gradient[(iz-sigma+di, ix-sigma+dj)];
                        count += 1;
                    }
                }
                smoothed[(iz, ix)] = sum / count as f32;
            }
        }
        
        smoothed
    }
    
    /// Illumination preconditioning
    fn illumination_precond(gradient: &Array2<f32>) -> Array2<f32> {
        // Simplified: normalize by max gradient
        let max_grad = gradient.iter().cloned().fold(0.0f32, f32::max);
        if max_grad > 0.0 {
            gradient / max_grad
        } else {
            gradient.clone()
        }
    }
}

/// Preconditioner types
pub enum Preconditioner {
    None,
    Smooth(usize), // sigma
    Illumination,
}

/// Line search for step length
pub struct LineSearch;

impl LineSearch {
    /// Backtracking line search
    pub fn backtrack<F>(
        initial_step: f32,
        gradient: &Array2<f32>,
        velocity: &Array2<f32>,
        mut misfit_fn: F,
    ) -> f32
    where
        F: FnMut(&Array2<f32>) -> f32,
    {
        let mut step = initial_step;
        let c = 1e-4; // Armijo condition parameter
        
        let current_misfit = misfit_fn(velocity);
        let grad_dot = Self::dot_product(gradient, velocity);
        
        for _ in 0..20 {
            // Trial velocity
            let mut trial_velocity = velocity.clone();
            for iz in 0..velocity.nrows() {
                for ix in 0..velocity.ncols() {
                    trial_velocity[(iz, ix)] -= step * gradient[(iz, ix)];
                }
            }
            
            let trial_misfit = misfit_fn(&trial_velocity);
            
            // Armijo condition
            if trial_misfit <= current_misfit - c * step * grad_dot {
                return step;
            }
            
            // Reduce step
            step *= 0.5;
        }
        
        step
    }
    
    fn dot_product(a: &Array2<f32>, b: &Array2<f32>) -> f32 {
        a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::Array2;

    #[test]
    fn test_gradient_computation() {
        let fwd = Array2::from_vec(vec![1.0, 2.0, 3.0, 4.0]);
        let adj = Array2::from_vec(vec![0.5, 1.0, 1.5, 2.0]);
        let vel = Array2::from_elem((2, 2), 2000.0);
        
        let grad = GradientCalculator::compute_adjoint(&fwd, &adj, &vel);
        
        assert_eq!(grad.dim(), (2, 2));
        assert!(grad.iter().all(|&x| x.is_finite()));
    }

    #[test]
    fn test_gradient_smoothing() {
        let gradient = Array2::from_vec(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
        let gradient = gradient.into_shape((3, 3)).unwrap();
        
        let smoothed = GradientCalculator::precondition(&gradient, Preconditioner::Smooth(1));
        
        assert_eq!(smoothed.dim(), (3, 3));
        // Center should be smoothed average
        assert!((smoothed[(1, 1)] - 5.0).abs() < 1.0);
    }

    #[test]
    fn test_illumination_preconditioning() {
        let gradient = Array2::from_vec(vec![1.0, 2.0, 3.0, 4.0]);
        
        let precond = GradientCalculator::precondition(&gradient, Preconditioner::Illumination);
        
        assert!(precond.iter().all(|&x| x >= 0.0 && x <= 1.0));
        assert_eq!(precond.iter().cloned().fold(0.0f32, f32::max), 1.0);
    }

    #[test]
    fn test_line_search() {
        let gradient = Array2::from_vec(vec![1.0, 2.0, 3.0, 4.0]);
        let velocity = Array2::from_elem((2, 2), 2000.0);
        
        let misfit_fn = |v: &Array2<f32>| v.iter().sum::<f32>();
        
        let step = LineSearch::backtrack(1.0, &gradient, &velocity, misfit_fn);
        
        assert!(step > 0.0);
        assert!(step <= 1.0);
    }
}
