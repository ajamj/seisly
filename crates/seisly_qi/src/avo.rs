//! AVO (Amplitude vs Offset) Analysis
//!
//! Implements AVO attributes for reservoir characterization.

/// AVO Analysis - computes gradient and intercept from amplitude vs angle
pub struct AvoAnalysis {
    angles: Vec<f32>,
    amplitudes: Vec<f32>,
}

impl AvoAnalysis {
    /// Create new AVO analysis from angle and amplitude vectors
    pub fn new(angles: Vec<f32>, amplitudes: Vec<f32>) -> Self {
        Self { angles, amplitudes }
    }
    
    /// AVO Gradient (slope of amplitude vs angle)
    /// Uses linear regression: gradient = (n*Σxy - Σx*Σy) / (n*Σx² - (Σx)²)
    pub fn gradient(&self) -> f32 {
        let n = self.angles.len() as f32;
        let sum_x = self.angles.iter().sum::<f32>();
        let sum_y = self.amplitudes.iter().sum::<f32>();
        let sum_xy = self.angles.iter()
            .zip(self.amplitudes.iter())
            .map(|(x, y)| x * y)
            .sum::<f32>();
        let sum_x2 = self.angles.iter().map(|x| x * x).sum::<f32>();
        
        let denominator = n * sum_x2 - sum_x * sum_x;
        if denominator.abs() < 1e-10 {
            0.0
        } else {
            (n * sum_xy - sum_x * sum_y) / denominator
        }
    }
    
    /// AVO Intercept (amplitude at zero offset)
    /// intercept = mean_y - gradient * mean_x
    pub fn intercept(&self) -> f32 {
        let gradient = self.gradient();
        let mean_x = self.angles.iter().sum::<f32>() / self.angles.len() as f32;
        let mean_y = self.amplitudes.iter().sum::<f32>() / self.amplitudes.len() as f32;
        
        mean_y - gradient * mean_x
    }
    
    /// AVO Product (Gradient × Intercept)
    /// Used for hydrocarbon detection
    pub fn avo_product(&self) -> f32 {
        self.gradient() * self.intercept()
    }
}

/// AVO Class - classification based on intercept and gradient
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AvoClass {
    /// Class 1: Positive intercept, negative gradient (gas sands)
    Class1,
    /// Class 2: Near-zero intercept, negative gradient
    Class2,
    /// Class 3: Negative intercept, negative gradient (bright spots)
    Class3,
    /// Class 4: Negative intercept, positive gradient
    Class4,
}

impl AvoAnalysis {
    /// Classify AVO response
    pub fn classify(&self) -> AvoClass {
        let intercept = self.intercept();
        let gradient = self.gradient();
        
        if intercept > 0.0 && gradient < 0.0 {
            AvoClass::Class1
        } else if intercept.abs() < 0.1 && gradient < 0.0 {
            AvoClass::Class2
        } else if intercept < 0.0 && gradient < 0.0 {
            AvoClass::Class3
        } else {
            AvoClass::Class4
        }
    }
}

/// Fluid Factor - discriminates fluid effects
pub struct FluidFactor;

impl FluidFactor {
    /// Compute fluid factor from AVO intercept and gradient
    /// FF = intercept - (expected_gradient × gradient)
    pub fn compute(intercept: f32, gradient: f32, expected_gradient: f32) -> f32 {
        intercept - (expected_gradient * gradient)
    }
    
    /// Compute fluid factor from Vp, Vs, and density
    pub fn from_elastic(vp: f32, vs: f32, rho: f32) -> f32 {
        // Simplified fluid factor
        let vp_vs = vp / vs;
        if vp_vs > 2.0 {
            (vp_vs - 2.0) * rho
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avo_gradient_positive() {
        let angles = vec![0.0, 10.0, 20.0, 30.0];
        let amplitudes = vec![1.0, 1.5, 2.0, 2.5];
        
        let avo = AvoAnalysis::new(angles, amplitudes);
        let gradient = avo.gradient();
        
        assert!(gradient > 0.0, "Gradient should be positive");
    }

    #[test]
    fn test_avo_gradient_negative() {
        let angles = vec![0.0, 10.0, 20.0, 30.0];
        let amplitudes = vec![2.0, 1.5, 1.0, 0.5];
        
        let avo = AvoAnalysis::new(angles, amplitudes);
        let gradient = avo.gradient();
        
        assert!(gradient < 0.0, "Gradient should be negative");
    }

    #[test]
    fn test_avo_intercept() {
        let angles = vec![0.0, 10.0, 20.0, 30.0];
        let amplitudes = vec![1.0, 1.0, 1.0, 1.0];
        
        let avo = AvoAnalysis::new(angles, amplitudes);
        let intercept = avo.intercept();
        
        assert!((intercept - 1.0).abs() < 0.1, "Intercept should be ~1.0");
    }

    #[test]
    fn test_avo_class_1() {
        let angles = vec![0.0, 10.0, 20.0, 30.0];
        let amplitudes = vec![2.0, 1.5, 1.0, 0.5];
        
        let avo = AvoAnalysis::new(angles, amplitudes);
        let class = avo.classify();
        
        assert_eq!(class, AvoClass::Class1);
    }

    #[test]
    fn test_fluid_factor() {
        let ff = FluidFactor::compute(0.5, -0.2, 0.3);
        assert!((ff - 0.56).abs() < 0.01);
    }

    #[test]
    fn test_fluid_factor_from_elastic() {
        let ff = FluidFactor::from_elastic(3000.0, 1500.0, 2.5);
        assert!(ff > 0.0);
    }
}
