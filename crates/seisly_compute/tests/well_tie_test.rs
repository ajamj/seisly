//! Well-seismic tie tests

use sf_compute::well_tie::WellTieEngine;
use sf_core::domain::Well;

#[test]
fn test_well_tie_with_velocity_model() {
    // Create synthetic well
    let mut well = Well::new(
        "Test Well".to_string(),
        "TW".to_string(),
        0.0,
        0.0,
        0.0,
    );

    // Add GR log from 0 to 990m (100 samples at 10m intervals)
    let depths: Vec<f32> = (0..100).map(|i| i as f32 * 10.0).collect();
    let gr_values: Vec<f32> = (0..100).map(|i| 50.0 + (i as f32 * 0.5)).collect();

    well.add_log("GR".to_string(), "GAPI".to_string(), gr_values, depths);

    // Create tie engine with V0 + kZ model
    let engine = WellTieEngine::new(2000.0, 0.5); // v0=2000 m/s, k=0.5 1/s
    let tie = engine.create_tie(&well).unwrap();

    assert_eq!(tie.well_id, well.id);
    assert!(!tie.time_depth_pairs.is_empty());
    
    // Verify accuracy: for v0=2000, k=0.5, depth=500m:
    // TWT = (2/0.5) * ln((2000 + 0.5*500) / 2000) * 1000
    // TWT = 4 * ln(1.125) * 1000 ≈ 471ms
    let pair = tie.time_depth_pairs.iter()
        .find(|p| (p.depth_md - 500.0).abs() < 0.1)
        .expect("Could not find depth pair near 500m");
    let twt = pair.twt;
    assert!((twt - 471.0).abs() < 1.0, "Expected TWT ~471ms, got {}", twt);
}

#[test]
fn test_depth_time_conversion() {
    // For v0=2000, k=0.5, depth=1000m:
    // TWT = (2/0.5) * ln((2000 + 0.5*1000) / 2000) * 1000 = 892ms
    let twt = WellTieEngine::depth_to_twt(1000.0, 2000.0, 0.5);
    assert!((twt - 892.0).abs() < 1.0, "Expected TWT ~892ms, got {}", twt);

    // Back conversion
    let depth = WellTieEngine::twt_to_depth(twt, 2000.0, 0.5);
    assert!((depth - 1000.0).abs() < 1.0, "Expected depth ~1000m, got {}", depth);
}
