//! Time-Lapse Analysis

/// Time-Lapse Analysis - compares base and monitor surveys
pub struct TimeLapseAnalysis {
    base_survey: Vec<f32>,
    monitor_survey: Vec<f32>,
    dt: f32,
}

impl TimeLapseAnalysis {
    /// Create new time-lapse analysis from base and monitor surveys
    pub fn new(base: Vec<f32>, monitor: Vec<f32>, dt: f32) -> Self {
        Self { base_survey: base, monitor_survey: monitor, dt }
    }
    
    /// Simple difference (Monitor - Base)
    pub fn difference(&self) -> Vec<f32> {
        self.monitor_survey.iter()
            .zip(self.base_survey.iter())
            .map(|(m, b)| m - b)
            .collect()
    }
    
    /// RMS difference (energy change)
    pub fn rms_difference(&self) -> f32 {
        let diff = self.difference();
        let sum_squares: f32 = diff.iter().map(|x| x * x).sum();
        (sum_squares / diff.len() as f32).sqrt()
    }
    
    /// Mean absolute difference
    pub fn mean_absolute_difference(&self) -> f32 {
        let diff = self.difference();
        let sum_abs: f32 = diff.iter().map(|x| x.abs()).sum();
        sum_abs / diff.len() as f32
    }
    
    /// Normalized RMS difference (NRMS)
    /// NRMS = 2 * RMS(M-B) / (RMS(M) + RMS(B)) * 100%
    pub fn nrms(&self) -> f32 {
        let diff = self.difference();
        let rms_diff: f32 = (diff.iter().map(|x| x * x).sum::<f32>() / diff.len() as f32).sqrt();
        
        let rms_base: f32 = (self.base_survey.iter().map(|x| x * x).sum::<f32>() / self.base_survey.len() as f32).sqrt();
        let rms_monitor: f32 = (self.monitor_survey.iter().map(|x| x * x).sum::<f32>() / self.monitor_survey.len() as f32).sqrt();
        
        let denominator = rms_monitor + rms_base;
        if denominator.abs() < 1e-10 {
            0.0
        } else {
            2.0 * rms_diff / denominator * 100.0
        }
    }
    
    /// Time shift (cross-correlation based)
    pub fn time_shift(&self, window: usize, max_lag: usize) -> Vec<f32> {
        let mut shifts = Vec::with_capacity(self.base_survey.len());
        
        for i in 0..self.base_survey.len() {
            let start = i.saturating_sub(window / 2);
            let end = (i + window / 2).min(self.base_survey.len() - 1);
            
            let base_window = &self.base_survey[start..=end];
            let monitor_window = &self.monitor_survey[start..=end];
            
            // Find best lag via cross-correlation
            let mut best_lag = 0;
            let mut best_corr = 0.0;
            
            for lag in 0..max_lag {
                let corr = self.cross_correlation(base_window, monitor_window, lag);
                if corr > best_corr {
                    best_corr = corr;
                    best_lag = lag;
                }
            }
            
            shifts.push(best_lag as f32 * self.dt);
        }
        
        shifts
    }
    
    /// Cross-correlation at specific lag
    fn cross_correlation(&self, base: &[f32], monitor: &[f32], lag: usize) -> f32 {
        if lag >= base.len() {
            return 0.0;
        }
        
        let sum: f32 = base.iter()
            .zip(monitor.iter().skip(lag))
            .map(|(a, b)| a * b)
            .sum();
        
        sum
    }
}

/// 4D Attribute Map
pub struct AttributeMap {
    pub inline: usize,
    pub crossline: usize,
    pub twt: f32,
    pub value: f32,
}

impl TimeLapseAnalysis {
    /// Generate attribute map for horizon slice
    pub fn horizon_slice(&self, horizon_times: &[(usize, usize, f32)]) -> Vec<AttributeMap> {
        horizon_times.iter()
            .filter_map(|(il, xl, twt)| {
                let sample_idx = (twt / self.dt) as usize;
                if sample_idx < self.base_survey.len() && sample_idx < self.monitor_survey.len() {
                    Some(AttributeMap {
                        inline: *il,
                        crossline: *xl,
                        twt: *twt,
                        value: self.monitor_survey[sample_idx] - self.base_survey[sample_idx],
                    })
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_difference() {
        let base = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let monitor = vec![1.5, 2.5, 3.5, 4.5, 5.5];
        
        let analysis = TimeLapseAnalysis::new(base, monitor, 0.004);
        let diff = analysis.difference();
        
        assert_eq!(diff.len(), 5);
        assert!(diff.iter().all(|&x| (x - 0.5).abs() < 0.01));
    }

    #[test]
    fn test_rms_difference() {
        let base = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let monitor = vec![2.0, 3.0, 4.0, 5.0, 6.0];
        
        let analysis = TimeLapseAnalysis::new(base, monitor, 0.004);
        let rms = analysis.rms_difference();
        
        assert!((rms - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_nrms() {
        let base = vec![1.0, 1.0, 1.0, 1.0, 1.0];
        let monitor = vec![1.1, 1.1, 1.1, 1.1, 1.1];
        
        let analysis = TimeLapseAnalysis::new(base, monitor, 0.004);
        let nrms = analysis.nrms();
        
        // Should be around 10% change
        assert!(nrms > 5.0 && nrms < 15.0);
    }

    #[test]
    fn test_no_change() {
        let base = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let monitor = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        
        let analysis = TimeLapseAnalysis::new(base, monitor, 0.004);
        
        assert_eq!(analysis.rms_difference(), 0.0);
        assert_eq!(analysis.nrms(), 0.0);
    }
}
