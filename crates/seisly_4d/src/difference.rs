//! 4D Difference Attributes

use crate::timelapse::TimeLapseAnalysis;

/// 4D Difference Map
pub struct DifferenceMap {
    pub inline_start: usize,
    pub inline_end: usize,
    pub crossline_start: usize,
    pub crossline_end: usize,
    pub values: Vec<f32>,
}

impl DifferenceMap {
    /// Create difference map from time-lapse analysis
    pub fn from_timelapse(analysis: &TimeLapseAnalysis) -> Self {
        let diff = analysis.difference();
        
        Self {
            inline_start: 0,
            inline_end: 100,
            crossline_start: 0,
            crossline_end: 100,
            values: diff,
        }
    }
    
    /// Get difference at specific location
    pub fn get(&self, il: usize, xl: usize) -> Option<f32> {
        if il < self.inline_start || il > self.inline_end ||
           xl < self.crossline_start || xl > self.crossline_end {
            return None;
        }
        
        let idx = (il - self.inline_start) * (self.crossline_end - self.crossline_start + 1) +
                  (xl - self.crossline_start);
        
        self.values.get(idx).copied()
    }
    
    /// Compute map statistics
    pub fn statistics(&self) -> DifferenceStats {
        let n = self.values.len() as f32;
        let sum: f32 = self.values.iter().sum();
        let mean = sum / n;
        
        let variance = self.values.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f32>() / n;
        
        let std_dev = variance.sqrt();
        
        let min = self.values.iter().cloned().fold(f32::INFINITY, f32::min);
        let max = self.values.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        
        DifferenceStats {
            mean,
            std_dev,
            min,
            max,
            rms: (self.values.iter().map(|x| x * x).sum::<f32>() / n).sqrt(),
        }
    }
}

/// Difference Statistics
#[derive(Debug)]
pub struct DifferenceStats {
    pub mean: f32,
    pub std_dev: f32,
    pub min: f32,
    pub max: f32,
    pub rms: f32,
}

/// 4D Anomaly Detector
pub struct AnomalyDetector {
    threshold: f32,
}

impl AnomalyDetector {
    pub fn new(threshold: f32) -> Self {
        Self { threshold }
    }
    
    /// Detect anomalies (values above threshold)
    pub fn detect(&self, map: &DifferenceMap) -> Vec<(usize, usize, f32)> {
        let mut anomalies = Vec::new();
        
        for (idx, &value) in map.values.iter().enumerate() {
            if value.abs() > self.threshold {
                let il = map.inline_start + idx / (map.crossline_end - map.crossline_start + 1);
                let xl = map.crossline_start + idx % (map.crossline_end - map.crossline_start + 1);
                anomalies.push((il, xl, value));
            }
        }
        
        anomalies
    }
    
    /// Classify anomaly type
    pub fn classify_anomaly(&self, value: f32) -> &'static str {
        if value > 0.0 {
            "Hardening (e.g., water influx)"
        } else {
            "Softening (e.g., gas expansion)"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_difference_map_statistics() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let map = DifferenceMap {
            inline_start: 0,
            inline_end: 2,
            crossline_start: 0,
            crossline_end: 1,
            values,
        };
        
        let stats = map.statistics();
        
        assert!((stats.mean - 3.0).abs() < 0.01);
        assert!(stats.std_dev > 0.0);
        assert!((stats.min - 1.0).abs() < 0.01);
        assert!((stats.max - 5.0).abs() < 0.01);
    }

    #[test]
    fn test_anomaly_detection() {
        let values = vec![0.1, 0.2, 0.5, -0.6, 0.1];
        let map = DifferenceMap {
            inline_start: 0,
            inline_end: 4,
            crossline_start: 0,
            crossline_end: 0,
            values,
        };
        
        let detector = AnomalyDetector::new(0.4);
        let anomalies = detector.detect(&map);
        
        assert_eq!(anomalies.len(), 2); // 0.5 and -0.6
    }

    #[test]
    fn test_anomaly_classification() {
        let detector = AnomalyDetector::new(0.4);
        
        let pos_class = detector.classify_anomaly(0.5);
        assert!(pos_class.contains("Hardening"));
        
        let neg_class = detector.classify_anomaly(-0.5);
        assert!(neg_class.contains("Softening"));
    }
}
