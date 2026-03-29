//! QI & AVO Integration Tests

use sf_qi::{AvoAnalysis, AvoClass, FluidFactor, PoissonsRatio, VpVsRatio, Gassmann, BrightSpot, Dhi};

#[test]
fn test_full_avo_workflow() {
    // Simulate AVO analysis workflow
    let angles = vec![0.0, 10.0, 20.0, 30.0, 40.0];
    let amplitudes = vec![2.0, 1.5, 1.0, 0.5, 0.0];
    
    let avo = AvoAnalysis::new(angles, amplitudes);
    
    let gradient = avo.gradient();
    let intercept = avo.intercept();
    let class = avo.classify();
    
    assert!(gradient < 0.0, "Should have negative gradient");
    assert_eq!(class, AvoClass::Class1);
    
    let ff = FluidFactor::compute(intercept, gradient, 0.3);
    assert!(ff.is_finite());
}

#[test]
fn test_elastic_parameters() {
    // Typical gas sand: Vp=2500, Vs=1200, Rho=2.2
    let vp = 2500.0;
    let vs = 1200.0;
    let rho = 2.2;
    
    let vp_vs = VpVsRatio::compute(vp, vs);
    let poisson = PoissonsRatio::from_vp_vs(vp, vs);
    
    assert!(vp_vs > 2.0, "Gas sand should have high Vp/Vs");
    assert!(poisson < 0.2, "Gas sand should have low Poisson's ratio");
    
    let interpretation = VpVsRatio::interpret(vp_vs);
    assert!(interpretation.contains("Gas"));
}

#[test]
fn test_gassmann_fluid_substitution() {
    // Brine-saturated sand
    let vp_brine = 2800.0;
    let vs_brine = 1400.0;
    let rho_brine = 2.3;
    let k_brine = 15.0;
    let k_hc = 0.5; // Gas much more compressible
    let porosity = 0.25;
    
    let vp_hc = Gassmann::vp_after_substitution(
        vp_brine, vs_brine, rho_brine, k_brine, k_hc, porosity,
    );
    
    // Vp should decrease after gas substitution
    assert!(vp_hc < vp_brine, "Vp should decrease with gas");
}

#[test]
fn test_dhi_analysis() {
    // Strong DHI scenario
    let vp_vs = 2.3;
    let poisson = 0.12;
    let intercept = -0.4;
    let gradient = -0.4;
    
    let score = Dhi::score(vp_vs, poisson, intercept, gradient);
    let interpretation = Dhi::interpret(score);
    
    assert!(score > 0.7, "Strong DHI should have high score");
    assert!(interpretation.contains("Strong"));
    
    // Bright spot probability
    let prob = BrightSpot::probability(intercept, gradient, AvoClass::Class3);
    assert!(prob > 0.8, "Class 3 AVO should have high bright spot probability");
}

#[test]
fn test_qi_multi_well_analysis() {
    // Simulate multi-well QI analysis
    let wells = vec![
        ("Well-1", 2500.0, 1200.0, 2.2),
        ("Well-2", 2800.0, 1400.0, 2.3),
        ("Well-3", 3000.0, 1600.0, 2.4),
    ];
    
    for (name, vp, vs, rho) in wells {
        let vp_vs = VpVsRatio::compute(vp, vs);
        let poisson = PoissonsRatio::from_vp_vs(vp, vs);
        
        println!("{}: Vp/Vs={:.2}, Poisson={:.2}", name, vp_vs, poisson);
        
        // Verify calculations are finite
        assert!(vp_vs.is_finite());
        assert!(poisson.is_finite());
        assert!(poisson >= 0.0 && poisson <= 0.5);
    }
}
