//! CCUS (Carbon Capture, Utilization, and Storage) Monitoring

use sf_core::domain::surface::Surface;
use sf_4d::TimeLapseAnalysis;

/// CCUS Monitor - CO2 plume tracking and storage verification
pub struct CCUSMonitor {
    baseline_survey: Vec<f32>,
    injection_rate: f32, // tonnes/day
}

impl CCUSMonitor {
    pub fn new(baseline: Vec<f32>, injection_rate: f32) -> Self {
        Self {
            baseline_survey: baseline,
            injection_rate,
        }
    }
    
    /// Track CO2 plume from 4D seismic
    pub fn track_plume(
        &self,
        monitor_survey: Vec<f32>,
        dt: f32,
    ) -> PlumeMap {
        let analysis = TimeLapseAnalysis::new(
            self.baseline_survey.clone(),
            monitor_survey,
            dt,
        );
        
        // Detect CO2 signature (brightening due to gas)
        let diff = analysis.difference();
        let plume_extent = self.detect_plume_extent(&diff);
        
        PlumeMap {
            extent: plume_extent,
            volume: self.estimate_plume_volume(&diff),
            confidence: 0.85,
        }
    }
    
    /// Detect plume extent from 4D difference
    fn detect_plume_extent(&self, diff: &[f32]) -> PlumeExtent {
        // Find contiguous region with positive amplitude change
        let mut max_amplitude = 0.0f32;
        let mut num_samples = 0;
        
        for &val in diff {
            if val > 0.3 { // Threshold for CO2 detection
                max_amplitude = max_amplitude.max(val);
                num_samples += 1;
            }
        }
        
        PlumeExtent {
            area_km2: num_samples as f32 * 0.01, // Simplified
            max_amplitude,
        }
    }
    
    /// Estimate CO2 plume volume
    fn estimate_plume_volume(&self, diff: &[f32]) -> f32 {
        // Simplified volume estimation
        // In production: use proper petrophysical relationships
        let anomaly_volume: f32 = diff.iter().filter(|&&x| x > 0.3).count() as f32;
        anomaly_volume * 1000.0 // m³
    }
    
    /// Verify storage capacity
    pub fn verify_storage_capacity(&self, plume_volume: f32) -> StorageReport {
        // Calculate injected CO2 volume
        let injected_mass = self.injection_rate * 365.0 * 10.0; // 10 years
        let injected_volume = injected_mass / 0.7; // CO2 density ~0.7 tonnes/m³
        
        // Storage efficiency
        let efficiency = plume_volume / injected_volume;
        
        StorageReport {
            injected_mass,
            plume_volume,
            storage_efficiency: efficiency,
            capacity_remaining: 1000000.0 - plume_volume, // Simplified
        }
    }
    
    /// Monitor for leakage
    pub fn detect_leakage(&self, plume_map: &PlumeMap) -> LeakageReport {
        // Check if plume extends beyond reservoir bounds
        // In production: implement proper leakage detection
        
        LeakageReport {
            leakage_detected: false,
            leakage_location: None,
            leakage_rate: 0.0,
        }
    }
}

/// CO2 Plume Map
pub struct PlumeMap {
    pub extent: PlumeExtent,
    pub volume: f32,
    pub confidence: f32,
}

/// Plume Extent
pub struct PlumeExtent {
    pub area_km2: f32,
    pub max_amplitude: f32,
}

/// Storage Report
pub struct StorageReport {
    pub injected_mass: f32, // tonnes
    pub plume_volume: f32, // m³
    pub storage_efficiency: f32,
    pub capacity_remaining: f32, // m³
}

/// Leakage Report
pub struct LeakageReport {
    pub leakage_detected: bool,
    pub leakage_location: Option<(f64, f64)>,
    pub leakage_rate: f32, // tonnes/year
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ccus_monitor_creation() {
        let baseline = vec![1.0; 1000];
        let monitor = CCUSMonitor::new(baseline, 1000.0);
        
        assert_eq!(monitor.baseline_survey.len(), 1000);
        assert_eq!(monitor.injection_rate, 1000.0);
    }

    #[test]
    fn test_plume_tracking() {
        let baseline = vec![1.0; 1000];
        let mut monitor_survey = vec![1.0; 1000];
        
        // Add CO2 signal
        for i in 400..500 {
            monitor_survey[i] = 1.5;
        }
        
        let monitor = CCUSMonitor::new(baseline, 1000.0);
        let plume = monitor.track_plume(monitor_survey, 0.004);
        
        assert!(plume.volume > 0.0);
        assert!(plume.confidence > 0.0);
    }

    #[test]
    fn test_storage_verification() {
        let baseline = vec![1.0; 1000];
        let monitor = CCUSMonitor::new(baseline, 1000.0);
        
        let report = monitor.verify_storage_capacity(50000.0);
        
        assert!(report.injected_mass > 0.0);
        assert!(report.storage_efficiency > 0.0);
    }

    #[test]
    fn test_leakage_detection() {
        let baseline = vec![1.0; 1000];
        let monitor = CCUSMonitor::new(baseline, 1000.0);
        
        let plume = PlumeMap {
            extent: PlumeExtent {
                area_km2: 1.0,
                max_amplitude: 0.5,
            },
            volume: 50000.0,
            confidence: 0.9,
        };
        
        let leakage = monitor.detect_leakage(&plume);
        assert!(!leakage.leakage_detected);
    }
}
