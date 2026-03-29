//! Acoustic FWI Implementation

use ndarray::{Array2, Array3};
use rayon::prelude::*;

/// Acoustic Wave Equation Solver
pub struct AcousticWaveSolver {
    velocity: Array2<f32>,
    dt: f32,
    dx: f32,
    dz: f32,
}

impl AcousticWaveSolver {
    pub fn new(velocity: Array2<f32>, dt: f32, dx: f32, dz: f32) -> Self {
        Self { velocity, dt, dx, dz }
    }
    
    /// Forward modeling: compute wavefield
    pub fn forward(&self, source: &Source, nt: usize) -> Array3<f32> {
        let nz = self.velocity.nrows();
        let nx = self.velocity.ncols();
        
        let mut wavefield = Array3::zeros((nt, nz, nx));
        let mut u_prev = Array2::zeros((nz, nx));
        let mut u_curr = Array2::zeros((nz, nx));
        
        // Source time function
        let stf = self.ricker_wavelet(nt, source.freq);
        
        for t in 0..nt {
            let mut u_next = Array2::zeros((nz, nx));
            
            // Finite difference update (2nd order in space, 2nd order in time)
            for iz in 1..nz-1 {
                for ix in 1..nx-1 {
                    let v = self.velocity[(iz, ix)];
                    let dt2 = self.dt * self.dt;
                    let dx2 = self.dx * self.dx;
                    let dz2 = self.dz * self.dz;
                    
                    let d2u_dx2 = (u_curr[(iz, ix+1)] - 2.0*u_curr[(iz, ix)] + u_curr[(iz, ix-1)]) / dx2;
                    let d2u_dz2 = (u_curr[(iz+1, ix)] - 2.0*u_curr[(iz, ix)] + u_curr[(iz-1, ix)]) / dz2;
                    
                    u_next[(iz, ix)] = 2.0*u_curr[(iz, ix)] - u_prev[(iz, ix)]
                        + dt2 * v * v * (d2u_dx2 + d2u_dz2);
                    
                    // Add source
                    if iz == source.iz && ix == source.ix {
                        u_next[(iz, ix)] += stf[t] * dt2 * v * v;
                    }
                }
            }
            
            // Absorbing boundary conditions (simple sponge)
            self.apply_absorbing_boundary(&mut u_next);
            
            wavefield.slice_mut(t..).assign(&u_next);
            u_prev = u_curr;
            u_curr = u_next;
        }
        
        wavefield
    }
    
    /// Ricker wavelet source time function
    fn ricker_wavelet(&self, nt: usize, freq: f32) -> Vec<f32> {
        let t0 = 1.0 / freq;
        let mut wavelet = Vec::with_capacity(nt);
        
        for i in 0..nt {
            let t = i as f32 * self.dt - t0;
            let pi_t = std::f32::consts::PI * t * freq;
            let val = (1.0 - 2.0 * pi_t * pi_t) * (-pi_t * pi_t).exp();
            wavelet.push(val);
        }
        
        wavelet
    }
    
    /// Apply absorbing boundary conditions
    fn apply_absorbing_boundary(&self, u: &mut Array2<f32>) {
        let nz = u.nrows();
        let nx = u.ncols();
        let sponge_width = 20;
        
        // Simple sponge absorption
        for iz in 0..nz {
            for ix in 0..sponge_width {
                let factor = ((sponge_width - ix) as f32 / sponge_width as f32).powi(2);
                u[(iz, ix)] *= factor;
                u[(iz, nx-1-ix)] *= factor;
            }
        }
        
        for ix in 0..nx {
            for iz in 0..sponge_width {
                let factor = ((sponge_width - iz) as f32 / sponge_width as f32).powi(2);
                u[(iz, ix)] *= factor;
                u[(nz-1-iz, ix)] *= factor;
            }
        }
    }
}

/// Acoustic FWI Main Class
pub struct AcousticFWI {
    solver: AcousticWaveSolver,
    observed_data: Array2<f32>,
}

impl AcousticFWI {
    pub fn new(velocity_init: Array2<f32>, dt: f32, dx: f32, dz: f32, observed: Array2<f32>) -> Self {
        let solver = AcousticWaveSolver::new(velocity_init, dt, dx, dz);
        Self {
            solver,
            observed_data: observed,
        }
    }
    
    /// Compute misfit function
    pub fn misfit(&self, velocity: &Array2<f32>, source: &Source, nt: usize) -> f32 {
        // Update solver velocity
        let mut new_solver = AcousticWaveSolver::new(velocity.clone(), self.solver.dt, self.solver.dx, self.solver.dz);
        
        // Forward modeling
        let wavefield = new_solver.forward(source, nt);
        
        // Extract data at receiver locations
        // For now, simple L2 misfit
        let mut misfit = 0.0f32;
        for i in 0..self.observed_data.len() {
            let diff = wavefield[[0, i % 100, i / 100]] - self.observed_data[[0, i]];
            misfit += diff * diff;
        }
        
        misfit / 2.0
    }
    
    /// Compute gradient using adjoint state method
    pub fn gradient(&self, velocity: &Array2<f32>, source: &Source, nt: usize) -> Array2<f32> {
        // Forward wavefield
        let wavefield_fwd = self.solver.forward(source, nt);
        
        // Compute residual at receivers
        let mut residual = Array2::zeros((nt, 100)); // Simplified
        for t in 0..nt {
            for i in 0..100 {
                residual[[t, i]] = wavefield_fwd[[t, 0, i]] - self.observed_data[[t, i]];
            }
        }
        
        // Adjoint wavefield (backpropagation)
        // TODO: Implement adjoint
        
        // Gradient = -2/v^3 * sum(u_fwd * u_adj)
        let mut gradient = Array2::zeros(velocity.dim());
        for iz in 0..velocity.nrows() {
            for ix in 0..velocity.ncols() {
                let v = velocity[(iz, ix)];
                let mut sum = 0.0f32;
                for t in 0..nt {
                    sum += wavefield_fwd[[t, iz, ix]] * wavefield_fwd[[t, iz, ix]]; // Simplified
                }
                gradient[(iz, ix)] = -2.0 / (v * v * v) * sum;
            }
        }
        
        gradient
    }
    
    /// Update velocity model using gradient
    pub fn update_velocity(&mut self, gradient: &Array2<f32>, learning_rate: f32) {
        // Simple gradient descent
        // In production: use L-BFGS or conjugate gradient
        let mut new_velocity = self.solver.velocity.clone();
        
        for iz in 0..new_velocity.nrows() {
            for ix in 0..new_velocity.ncols() {
                new_velocity[(iz, ix)] -= learning_rate * gradient[(iz, ix)];
            }
        }
        
        // Apply smoothing to gradient
        new_velocity = self.smooth(&new_velocity, 5);
        
        self.solver.velocity = new_velocity;
    }
    
    /// Gaussian smoothing
    fn smooth(&self, velocity: &Array2<f32>, sigma: usize) -> Array2<f32> {
        // Simple moving average smoothing
        let mut smoothed = velocity.clone();
        let nz = velocity.nrows();
        let nx = velocity.ncols();
        
        for iz in sigma..nz-sigma {
            for ix in sigma..nx-sigma {
                let mut sum = 0.0f32;
                let mut count = 0;
                for di in 0..sigma*2+1 {
                    for dj in 0..sigma*2+1 {
                        sum += velocity[(iz-sigma+di, ix-sigma+dj)];
                        count += 1;
                    }
                }
                smoothed[(iz, ix)] = sum / count as f32;
            }
        }
        
        smoothed
    }
    
    /// Get current velocity model
    pub fn velocity(&self) -> &Array2<f32> {
        &self.solver.velocity
    }
}

/// Source definition
#[derive(Clone)]
pub struct Source {
    pub iz: usize,
    pub ix: usize,
    pub freq: f32,
}

impl Source {
    pub fn new(iz: usize, ix: usize, freq: f32) -> Self {
        Self { iz, ix, freq }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acoustic_solver_creation() {
        let velocity = Array2::from_elem((100, 100), 2000.0);
        let solver = AcousticWaveSolver::new(velocity, 0.001, 10.0, 10.0);
        
        assert_eq!(solver.velocity.nrows(), 100);
        assert_eq!(solver.velocity.ncols(), 100);
    }

    #[test]
    fn test_fwi_misfit() {
        let velocity = Array2::from_elem((50, 50), 2000.0);
        let observed = Array2::zeros((100, 50));
        
        let fwi = AcousticFWI::new(velocity, 0.001, 10.0, 10.0, observed);
        let source = Source::new(25, 25, 25.0);
        
        let misfit = fwi.misfit(fwi.velocity(), &source, 100);
        assert!(misfit >= 0.0);
    }

    #[test]
    fn test_gradient_computation() {
        let velocity = Array2::from_elem((50, 50), 2000.0);
        let observed = Array2::zeros((100, 50));
        
        let fwi = AcousticFWI::new(velocity, 0.001, 10.0, 10.0, observed);
        let source = Source::new(25, 25, 25.0);
        
        let gradient = fwi.gradient(fwi.velocity(), &source, 100);
        
        assert_eq!(gradient.dim(), (50, 50));
        assert!(gradient.iter().all(|&x| x.is_finite()));
    }

    #[test]
    fn test_velocity_update() {
        let velocity = Array2::from_elem((50, 50), 2000.0);
        let observed = Array2::zeros((100, 50));
        
        let mut fwi = AcousticFWI::new(velocity, 0.001, 10.0, 10.0, observed);
        let source = Source::new(25, 25, 25.0);
        
        let gradient = fwi.gradient(fwi.velocity(), &source, 100);
        fwi.update_velocity(&gradient, 0.01);
        
        // Velocity should change
        assert_ne!(fwi.velocity(), &Array2::from_elem((50, 50), 2000.0));
    }
}
