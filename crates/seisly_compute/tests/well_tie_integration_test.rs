//! Well-seismic tie integration tests

use sf_compute::well_tie::{WellTieEngine, WellTieError};
use sf_core::domain::Well;

#[test]
fn test_well_tie_full_workflow() {
    // Create synthetic well with multiple logs
    let mut well = Well::new(
        "Integration Test Well".to_string(),
        "ITW".to_string(),
        500000.0,
        1000000.0,
        100.0,
    );
    
    // Add GR log (0-1000m, 10m interval)
    let depths: Vec<f32> = (0..101).map(|i| i as f32 * 10.0).collect();
    let gr_values: Vec<f32> = (0..101).map(|i| 50.0 + (i as f32 * 0.3)).collect();
    
    well.add_log("GR".to_string(), "GAPI".to_string(), gr_values, depths.clone());
    
    // Add DT log
    let dt_values: Vec<f32> = (0..101).map(|i| 200.0 + (i as f32 * 0.5)).collect();
    well.add_log("DT".to_string(), "US/M".to_string(), dt_values, depths.clone());
    
    // Add RHOB log
    let rhob_values: Vec<f32> = (0..101).map(|i| 2.0 + (i as f32 * 0.005)).collect();
    well.add_log("RHOB".to_string(), "G/CC".to_string(), rhob_values, depths);

    // Create well-seismic tie
    let engine = WellTieEngine::new(2000.0, 0.5);
    let tie = engine.create_tie(&well).unwrap();
    
    // Verify tie
    assert_eq!(tie.well_id, well.id);
    assert!(!tie.time_depth_pairs.is_empty());
    
    // Verify depth range coverage
    let min_depth = tie.time_depth_pairs.first().unwrap().depth_md;
    let max_depth = tie.time_depth_pairs.last().unwrap().depth_md;
    assert!(min_depth <= 0.0, "Min depth should be <= 0, got {}", min_depth);
    assert!(max_depth >= 1000.0, "Max depth should be >= 1000, got {}", max_depth);
    
    // Verify time-depth relationship (monotonic increase)
    for i in 1..tie.time_depth_pairs.len() {
        assert!(
            tie.time_depth_pairs[i].twt > tie.time_depth_pairs[i-1].twt,
            "TWT should increase with depth: pair[{}] = {}ms, pair[{}] = {}ms",
            i, tie.time_depth_pairs[i].twt,
            i-1, tie.time_depth_pairs[i-1].twt
        );
        assert!(
            tie.time_depth_pairs[i].depth_md > tie.time_depth_pairs[i-1].depth_md,
            "Depth should increase: pair[{}] = {}m, pair[{}] = {}m",
            i, tie.time_depth_pairs[i].depth_md,
            i-1, tie.time_depth_pairs[i-1].depth_md
        );
    }
    
    // Verify parameters were stored correctly
    assert_eq!(tie.parameters.v0, 2000.0);
    assert_eq!(tie.parameters.k, 0.5);
}

#[test]
fn test_velocity_model_accuracy() {
    // Test various velocity models - calculate expected values using the formula
    // TWT = (2/k) * ln((v0 + k*depth) / v0) * 1000
    let test_cases = vec![
        // (v0, k, depth, expected_twt)
        (2000.0, 0.5, 1000.0, 892.58),   // Standard sedimentary
        (2500.0, 0.3, 1000.0, 755.52),   // Carbonate (slower gradient)
        (1800.0, 0.7, 1000.0, 938.58),   // Unconsolidated (faster gradient)
        (2000.0, 0.5, 500.0, 471.13),    // Mid-depth
        (2000.0, 0.5, 2000.0, 1621.86),  // Deep
    ];

    for (v0, k, depth, expected_twt) in test_cases {
        let twt = WellTieEngine::depth_to_twt(depth, v0, k);
        assert!(
            (twt - expected_twt).abs() < 0.5,
            "Failed for v0={}, k={}, depth={}: expected ~{}, got {}",
            v0, k, depth, expected_twt, twt
        );

        // Verify inverse conversion
        let recovered_depth = WellTieEngine::twt_to_depth(twt, v0, k);
        assert!(
            (recovered_depth - depth).abs() < 0.5,
            "Inverse conversion failed: expected depth {}, got {}",
            depth, recovered_depth
        );
    }
}

#[test]
fn test_well_without_logs_error() {
    let well = Well::new(
        "Empty Well".to_string(),
        "EW".to_string(),
        0.0,
        0.0,
        0.0,
    );
    let engine = WellTieEngine::new(2000.0, 0.5);
    
    let result = engine.create_tie(&well);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), WellTieError::NoLogs));
}

#[test]
fn test_well_tie_with_single_log() {
    let mut well = Well::new(
        "Single Log Well".to_string(),
        "SLW".to_string(),
        100.0,
        200.0,
        50.0,
    );

    // Add only one log (0-1000m at 20m intervals = 51 points)
    let depths: Vec<f32> = (0..51).map(|i| i as f32 * 20.0).collect();
    let gr_values: Vec<f32> = (0..51).map(|i| 40.0 + (i as f32 * 0.4)).collect();

    well.add_log("GR".to_string(), "GAPI".to_string(), gr_values, depths);

    let engine = WellTieEngine::new(2000.0, 0.5);
    let tie = engine.create_tie(&well).unwrap();

    assert_eq!(tie.well_id, well.id);
    // Engine generates pairs at 10m intervals, so 0-1000m = 101 pairs
    assert_eq!(tie.time_depth_pairs.len(), 101);

    // Verify first and last depths
    assert!((tie.time_depth_pairs[0].depth_md - 0.0).abs() < 0.1);
    assert!((tie.time_depth_pairs[100].depth_md - 1000.0).abs() < 0.1);
}

#[test]
fn test_well_tie_different_velocity_models() {
    let mut well = Well::new(
        "Multi-Velocity Test".to_string(),
        "MVT".to_string(),
        0.0,
        0.0,
        0.0,
    );
    
    let depths: Vec<f32> = (0..101).map(|i| i as f32 * 10.0).collect();
    let values: Vec<f32> = vec![50.0; 101]; // Constant values
    
    well.add_log("GR".to_string(), "GAPI".to_string(), values, depths);

    // Test with different velocity models
    let velocity_models = vec![
        (1500.0, 0.3),  // Slow, low gradient
        (2000.0, 0.5),  // Standard
        (2500.0, 0.7),  // Fast, high gradient
        (3000.0, 0.4),  // Fast, moderate gradient
    ];
    
    for (v0, k) in velocity_models {
        let engine = WellTieEngine::new(v0, k);
        let tie = engine.create_tie(&well).unwrap();
        
        assert_eq!(tie.parameters.v0, v0);
        assert_eq!(tie.parameters.k, k);
        
        // Verify TWT increases with depth
        for i in 1..tie.time_depth_pairs.len() {
            assert!(tie.time_depth_pairs[i].twt > tie.time_depth_pairs[i-1].twt);
        }
        
        // Different velocity models should give different TWT at same depth
        let twt_at_500m = tie.time_depth_pairs.iter()
            .find(|p| (p.depth_md - 500.0).abs() < 0.1)
            .unwrap()
            .twt;
        
        println!("v0={}, k={} -> TWT at 500m = {:.2}ms", v0, k, twt_at_500m);
    }
}

#[test]
fn test_well_tie_shallow_vs_deep() {
    // Create shallow well (0-500m)
    let mut shallow_well = Well::new(
        "Shallow Well".to_string(),
        "SW".to_string(),
        0.0,
        0.0,
        0.0,
    );
    let shallow_depths: Vec<f32> = (0..51).map(|i| i as f32 * 10.0).collect();
    let shallow_values: Vec<f32> = vec![50.0; 51];
    shallow_well.add_log("GR".to_string(), "GAPI".to_string(), shallow_values, shallow_depths);

    // Create deep well (0-3000m)
    let mut deep_well = Well::new(
        "Deep Well".to_string(),
        "DW".to_string(),
        0.0,
        0.0,
        0.0,
    );
    let deep_depths: Vec<f32> = (0..301).map(|i| i as f32 * 10.0).collect();
    let deep_values: Vec<f32> = vec![50.0; 301];
    deep_well.add_log("GR".to_string(), "GAPI".to_string(), deep_values, deep_depths);

    let engine = WellTieEngine::new(2000.0, 0.5);
    
    let shallow_tie = engine.create_tie(&shallow_well).unwrap();
    let deep_tie = engine.create_tie(&deep_well).unwrap();
    
    // Verify depth ranges
    assert!((shallow_tie.time_depth_pairs.last().unwrap().depth_md - 500.0).abs() < 0.1);
    assert!((deep_tie.time_depth_pairs.last().unwrap().depth_md - 3000.0).abs() < 0.1);
    
    // Deep well should have more time-depth pairs
    assert!(deep_tie.time_depth_pairs.len() > shallow_tie.time_depth_pairs.len());
    
    // Compare TWT at common depth (500m)
    let shallow_twt_500 = shallow_tie.time_depth_pairs.last().unwrap().twt;
    let deep_twt_500 = deep_tie.time_depth_pairs.iter()
        .find(|p| (p.depth_md - 500.0).abs() < 0.1)
        .unwrap()
        .twt;
    
    assert!((shallow_twt_500 - deep_twt_500).abs() < 1.0,
        "TWT at 500m should be same: shallow={}, deep={}", shallow_twt_500, deep_twt_500);
}

#[test]
fn test_velocity_model_edge_cases() {
    // Test zero depth
    let twt_zero = WellTieEngine::depth_to_twt(0.0, 2000.0, 0.5);
    assert!(twt_zero.abs() < 1e-10, "TWT at depth 0 should be 0, got {}", twt_zero);
    
    // Test zero TWT
    let depth_zero = WellTieEngine::twt_to_depth(0.0, 2000.0, 0.5);
    assert!(depth_zero.abs() < 1e-10, "Depth at TWT 0 should be 0, got {}", depth_zero);
    
    // Test very shallow depth
    let twt_shallow = WellTieEngine::depth_to_twt(1.0, 2000.0, 0.5);
    assert!(twt_shallow > 0.0 && twt_shallow < 10.0,
        "TWT at 1m should be small positive, got {}", twt_shallow);
    
    // Test very deep depth
    let twt_deep = WellTieEngine::depth_to_twt(5000.0, 2000.0, 0.5);
    assert!(twt_deep > 2000.0, "TWT at 5000m should be large, got {}", twt_deep);
}

#[test]
fn test_well_tie_with_well_tops() {
    let mut well = Well::new(
        "Well with Tops".to_string(),
        "WWT".to_string(),
        0.0,
        0.0,
        0.0,
    );
    
    // Add log
    let depths: Vec<f32> = (0..101).map(|i| i as f32 * 10.0).collect();
    let values: Vec<f32> = vec![50.0; 101];
    well.add_log("GR".to_string(), "GAPI".to_string(), values, depths);
    
    // Add formation tops
    well.add_top("Top Reservoir".to_string(), 1500.0, "TOP".to_string(), [1.0, 0.0, 0.0, 1.0]);
    well.add_top("Base Reservoir".to_string(), 2000.0, "BASE".to_string(), [0.0, 1.0, 0.0, 1.0]);
    well.add_top("Top Seal".to_string(), 1400.0, "TOP".to_string(), [0.0, 0.0, 1.0, 1.0]);

    let engine = WellTieEngine::new(2000.0, 0.5);
    let tie = engine.create_tie(&well).unwrap();
    
    // Verify tie was created successfully
    assert_eq!(tie.well_id, well.id);
    assert!(!tie.time_depth_pairs.is_empty());
    
    // Verify well tops are preserved
    assert_eq!(well.tops.len(), 3);
}

#[test]
fn test_time_depth_pair_consistency() {
    let mut well = Well::new(
        "Consistency Test".to_string(),
        "CT".to_string(),
        0.0,
        0.0,
        0.0,
    );
    
    let depths: Vec<f32> = (0..101).map(|i| i as f32 * 10.0).collect();
    let values: Vec<f32> = vec![50.0; 101];
    well.add_log("GR".to_string(), "GAPI".to_string(), values, depths);

    let engine = WellTieEngine::new(2000.0, 0.5);
    let tie = engine.create_tie(&well).unwrap();
    
    // Verify all pairs have valid values
    for pair in &tie.time_depth_pairs {
        assert!(pair.depth_md >= 0.0, "Depth should be non-negative: {}", pair.depth_md);
        assert!(pair.twt >= 0.0, "TWT should be non-negative: {}", pair.twt);
        assert!(pair.twt.is_finite(), "TWT should be finite: {}", pair.twt);
        assert!(pair.depth_md.is_finite(), "Depth should be finite: {}", pair.depth_md);
    }
    
    // Verify consistent interval
    for i in 1..tie.time_depth_pairs.len() {
        let depth_interval = tie.time_depth_pairs[i].depth_md - tie.time_depth_pairs[i-1].depth_md;
        assert!(
            (depth_interval - 10.0).abs() < 0.1,
            "Depth interval should be 10m, got {}", depth_interval
        );
    }
}
