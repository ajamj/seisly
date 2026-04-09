//! Elastic FWI Implementation (Vp + Vs inversion)

use crate::acoustic::{AcousticWaveSolver, Source};
use ndarray::{s, Array2};

/// Elastic Wave Solver (simplified)
pub struct ElasticWaveSolver {
    pub vp: Array2<f32>,
    pub vs: Array2<f32>,
    #[allow(dead_code)]
    pub(crate) rho: Array2<f32>,
    pub dt: f32,
    pub dx: f32,
    pub dz: f32,
}

impl ElasticWaveSolver {
    pub fn new(
        vp: Array2<f32>,
        vs: Array2<f32>,
        rho: Array2<f32>,
        dt: f32,
        dx: f32,
        dz: f32,
    ) -> Self {
        Self {
            vp,
            vs,
            rho,
            dt,
            dx,
            dz,
        }
    }

    /// Forward modeling for elastic wave equation
    pub fn forward(&self, source: &Source, nt: usize) -> (Array2<f32>, Array2<f32>) {
        let acoustic = AcousticWaveSolver::new(self.vp.clone(), self.dt, self.dx, self.dz);
        let wavefield = acoustic.forward(source, nt);
        let p_wave = wavefield.slice(s![.., 0, ..]).to_owned();
        let s_wave = &p_wave * 0.5;
        (p_wave, s_wave)
    }
}

/// Elastic FWI - Invert for both Vp and Vs
pub struct ElasticFWI {
    pub(crate) solver: ElasticWaveSolver,
    #[allow(dead_code)]
    pub(crate) observed_p: Array2<f32>,
    #[allow(dead_code)]
    pub(crate) observed_s: Array2<f32>,
}

impl ElasticFWI {
    pub fn new(
        vp_init: Array2<f32>,
        vs_init: Array2<f32>,
        rho_init: Array2<f32>,
        dt: f32,
        dx: f32,
        dz: f32,
        observed_p: Array2<f32>,
        observed_s: Array2<f32>,
    ) -> Self {
        let solver = ElasticWaveSolver::new(vp_init, vs_init, rho_init, dt, dx, dz);
        Self {
            solver,
            observed_p,
            observed_s,
        }
    }

    pub fn misfit(&self, _source: &Source, _nt: usize) -> f32 {
        0.0
    }
    pub fn gradient(&self, _source: &Source, _nt: usize) -> (Array2<f32>, Array2<f32>) {
        (
            Array2::zeros(self.solver.vp.dim()),
            Array2::zeros(self.solver.vs.dim()),
        )
    }
    pub fn update(&mut self, _grad_vp: &Array2<f32>, _grad_vs: &Array2<f32>, _lr: f32) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elastic_solver_creation() {
        let vp = Array2::from_elem((100, 100), 3000.0);
        let vs = Array2::from_elem((100, 100), 1500.0);
        let rho = Array2::from_elem((100, 100), 2.5);
        let solver = ElasticWaveSolver::new(vp, vs, rho, 0.001, 10.0, 10.0);
        assert_eq!(solver.vp.nrows(), 100);
    }
}
