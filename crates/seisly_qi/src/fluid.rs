//! Fluid Discrimination & Substitution

use crate::elastic::{PoissonsRatio, VpVsRatio};

/// Gassmann Fluid Substitution
pub struct Gassmann;

impl Gassmann {
    /// Compute bulk modulus of fluid-saturated rock
    /// K_sat = K_dry + (1 - K_dry/K_matrix)² / (φ/K_fluid + (1-φ)/K_matrix - K_dry/K_matrix²)
    pub fn bulk_modulus(
        k_dry: f32,
        k_matrix: f32,
        k_fluid: f32,
        porosity: f32,
    ) -> f32 {
        if k_matrix.abs() < 1e-10 {
            return k_dry;
        }
        
        let phi = porosity;
        let term1 = 1.0 - k_dry / k_matrix;
        let denominator = phi / k_fluid + (1.0 - phi) / k_matrix - k_dry / (k_matrix * k_matrix);
        
        if denominator.abs() < 1e-10 {
            k_dry
        } else {
            k_dry + (term1 * term1) / denominator
        }
    }
    
    /// Compute Vp after fluid substitution
    pub fn vp_after_substitution(
        vp_brine: f32,
        vs_brine: f32,
        rho_brine: f32,
        k_brine: f32,
        k_hc: f32,
        porosity: f32,
    ) -> f32 {
        // Simplified fluid substitution
        let k_matrix = 36.0; // Typical quartz bulk modulus (GPa)
        let k_dry = k_brine - porosity * k_brine; // Approximate
        
        let k_sat_hc = Self::bulk_modulus(k_dry, k_matrix, k_hc, porosity);
        
        // Vp = sqrt((K + 4/3*μ) / ρ)
        let mu = rho_brine * vs_brine * vs_brine;
        let rho_hc = rho_brine * 0.9; // HC slightly lighter
        
        ((k_sat_hc + 4.0/3.0*mu) / rho_hc).sqrt() * 1000.0 // Convert to m/s
    }
}

/// Bright Spot Indicator
pub struct BrightSpot;

impl BrightSpot {
    /// Compute bright spot probability from AVO attributes
    pub fn probability(intercept: f32, gradient: f32, avo_class: super::AvoClass) -> f32 {
        let mut prob = 0.0;
        
        // Class 3 AVO = high probability
        if avo_class == super::AvoClass::Class3 {
            prob += 0.5;
        }
        
        // Strong negative intercept = high probability
        if intercept < -0.2 {
            prob += 0.3;
        }
        
        // Strong negative gradient = high probability
        if gradient < -0.3 {
            prob += 0.2;
        }
        
        prob.min(1.0)
    }
}

/// DHI (Direct Hydrocarbon Indicator)
pub struct Dhi;

impl Dhi {
    /// Compute DHI score from multiple attributes
    pub fn score(
        vp_vs: f32,
        poisson: f32,
        intercept: f32,
        gradient: f32,
    ) -> f32 {
        let mut score = 0.0;
        
        // High Vp/Vs = possible gas
        if vp_vs > 2.0 {
            score += 0.3;
        }
        
        // Low Poisson's ratio = possible gas
        if poisson < 0.15 {
            score += 0.3;
        }
        
        // Class 3 AVO
        if intercept < 0.0 && gradient < 0.0 {
            score += 0.4;
        }
        
        score.min(1.0)
    }
    
    /// Interpret DHI score
    pub fn interpret(score: f32) -> &'static str {
        if score > 0.7 {
            "Strong DHI - High confidence hydrocarbon"
        } else if score > 0.4 {
            "Moderate DHI - Possible hydrocarbon"
        } else {
            "Weak DHI - Unlikely hydrocarbon"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gassmann_bulk_modulus() {
        let k_sat = Gassmann::bulk_modulus(10.0, 36.0, 2.5, 0.25);
        assert!(k_sat > 10.0, "Saturated bulk modulus should increase");
    }

    #[test]
    fn test_bright_spot_probability() {
        use crate::AvoClass;
        
        // Class 3 with strong negative attributes
        let prob = BrightSpot::probability(-0.5, -0.5, AvoClass::Class3);
        assert!(prob > 0.8, "Strong bright spot should have high probability");
    }

    #[test]
    fn test_dhi_score_strong() {
        let score = Dhi::score(2.5, 0.1, -0.5, -0.5);
        assert!(score > 0.7, "Strong DHI indicators should give high score");
        
        let interpretation = Dhi::interpret(score);
        assert!(interpretation.contains("Strong"));
    }

    #[test]
    fn test_dhi_score_weak() {
        let score = Dhi::score(1.7, 0.25, 0.1, 0.1);
        assert!(score < 0.4, "Weak indicators should give low score");
    }
}
