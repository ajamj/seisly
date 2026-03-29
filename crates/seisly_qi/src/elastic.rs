//! Elastic Parameters for QI

/// Poisson's Ratio computation
pub struct PoissonsRatio;

impl PoissonsRatio {
    /// Compute from Vp and Vs
    /// σ = (Vp² - 2Vs²) / (2(Vp² - Vs²))
    pub fn from_vp_vs(vp: f32, vs: f32) -> f32 {
        let vp2 = vp * vp;
        let vs2 = vs * vs;
        let denominator = 2.0 * (vp2 - vs2);
        
        if denominator.abs() < 1e-10 {
            0.25 // Default value
        } else {
            (vp2 - 2.0 * vs2) / denominator
        }
    }
    
    /// Compute from lambda and mu (Lamé parameters)
    pub fn from_lame(lambda: f32, mu: f32) -> f32 {
        let denominator = 2.0 * (lambda + mu);
        if denominator.abs() < 1e-10 {
            0.25
        } else {
            lambda / denominator
        }
    }
}

/// Vp/Vs Ratio
pub struct VpVsRatio;

impl VpVsRatio {
    /// Compute Vp/Vs ratio
    pub fn compute(vp: f32, vs: f32) -> f32 {
        if vs.abs() < 1e-10 {
            f32::INFINITY
        } else {
            vp / vs
        }
    }
    
    /// Interpret Vp/Vs ratio
    pub fn interpret(vp_vs: f32) -> &'static str {
        if vp_vs < 1.5 {
            "Hard rock / carbonate"
        } else if vp_vs < 1.7 {
            "Sand / clastic"
        } else if vp_vs < 2.0 {
            "Shale / unconsolidated"
        } else if vp_vs < 2.5 {
            "Gas sand (anomaly)"
        } else {
            "Very soft / gas"
        }
    }
}

/// Lambda-Rho (λρ) - Incompressibility
pub struct LambdaRho;

impl LambdaRho {
    /// Compute from Vp, Vs, and density
    pub fn compute(vp: f32, vs: f32, rho: f32) -> f32 {
        let vp2 = vp * vp;
        let vs2 = vs * vs;
        rho * (vp2 - 2.0 * vs2)
    }
}

/// Mu-Rho (μρ) - Rigidity
pub struct MuRho;

impl MuRho {
    /// Compute from Vs and density
    pub fn compute(vs: f32, rho: f32) -> f32 {
        rho * vs * vs
    }
}

/// Young's Modulus
pub struct YoungsModulus;

impl YoungsModulus {
    /// Compute from Vp, Vs, and density
    pub fn compute(vp: f32, vs: f32, rho: f32) -> f32 {
        let vs2 = vs * vs;
        let vp2 = vp * vp;
        let term = (3.0 * vp2 - 4.0 * vs2) / (vp2 - vs2);
        rho * vs2 * term
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poissons_ratio_typical() {
        // Typical sandstone: Vp=3000, Vs=1800
        let sigma = PoissonsRatio::from_vp_vs(3000.0, 1800.0);
        assert!(sigma > 0.2 && sigma < 0.3, "Typical Poisson's ratio for sandstone");
    }

    #[test]
    fn test_vp_vs_ratio_sand() {
        let vp_vs = VpVsRatio::compute(3000.0, 1800.0);
        assert!((vp_vs - 1.67).abs() < 0.1);
        
        let interpretation = VpVsRatio::interpret(vp_vs);
        assert!(interpretation.contains("Sand"));
    }

    #[test]
    fn test_vp_vs_ratio_gas() {
        let vp_vs = VpVsRatio::compute(2500.0, 1200.0);
        assert!(vp_vs > 2.0);
        
        let interpretation = VpVsRatio::interpret(vp_vs);
        assert!(interpretation.contains("Gas"));
    }

    #[test]
    fn test_lambda_rho() {
        let lr = LambdaRho::compute(3000.0, 1800.0, 2.5);
        assert!(lr > 0.0);
    }

    #[test]
    fn test_mu_rho() {
        let mr = MuRho::compute(1800.0, 2.5);
        assert!(mr > 0.0);
    }
}
