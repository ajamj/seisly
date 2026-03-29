//! Reservoir Surveillance

use sf_4d::{TimeLapseAnalysis, ProductionTimeline};

/// Reservoir Surveillance - monitor reservoir performance
pub struct ReservoirSurveillance {
    production_data: Vec<ProductionTimeline>,
    time_lapse_surveys: Vec<Vec<f32>>,
}

impl ReservoirSurveillance {
    pub fn new() -> Self {
        Self {
            production_data: Vec::new(),
            time_lapse_surveys: Vec::new(),
        }
    }
    
    /// Add production timeline
    pub fn add_production_data(&mut self, timeline: ProductionTimeline) {
        self.production_data.push(timeline);
    }
    
    /// Add time-lapse survey
    pub fn add_survey(&mut self, survey: Vec<f32>) {
        self.time_lapse_surveys.push(survey);
    }
    
    /// Generate surveillance report
    pub fn generate_report(&self) -> SurveillanceReport {
        let production_summary = self.summarize_production();
        let four_d_summary = self.analyze_4d_changes();
        
        SurveillanceReport {
            production_summary,
            four_d_summary,
            recommendations: self.generate_recommendations(),
        }
    }
    
    /// Summarize production performance
    fn summarize_production(&self) -> ProductionSummary {
        let mut total_oil = 0.0;
        let mut total_gas = 0.0;
        let mut total_water = 0.0;
        
        for timeline in &self.production_data {
            let (oil, gas, water) = timeline.cumulative();
            total_oil += oil;
            total_gas += gas;
            total_water += water;
        }
        
        ProductionSummary {
            total_oil,
            total_gas,
            total_water,
            water_cut: if total_oil + total_water > 0.0 {
                total_water / (total_oil + total_water)
            } else {
                0.0
            },
        }
    }
    
    /// Analyze 4D changes
    fn analyze_4d_changes(&self) -> FourDSummary {
        if self.time_lapse_surveys.len() < 2 {
            return FourDSummary {
                nrms_average: 0.0,
                max_change: 0.0,
                change_location: None,
            };
        }
        
        let mut nrms_values = Vec::new();
        let mut max_change = 0.0f32;
        let mut max_location = None;
        
        for i in 1..self.time_lapse_surveys.len() {
            let analysis = TimeLapseAnalysis::new(
                self.time_lapse_surveys[0].clone(),
                self.time_lapse_surveys[i].clone(),
                0.004,
            );
            
            let nrms = analysis.nrms();
            nrms_values.push(nrms);
            
            // Find location of maximum change
            let diff = analysis.difference();
            for (idx, &val) in diff.iter().enumerate() {
                if val.abs() > max_change {
                    max_change = val.abs();
                    max_location = Some(idx);
                }
            }
        }
        
        FourDSummary {
            nrms_average: nrms_values.iter().sum::<f32>() / nrms_values.len() as f32,
            max_change,
            change_location: max_location,
        }
    }
    
    /// Generate recommendations
    fn generate_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        let prod_summary = self.summarize_production();
        let four_d_summary = self.analyze_4d_changes();
        
        // High water cut recommendation
        if prod_summary.water_cut > 0.5 {
            recommendations.push("High water cut detected - consider water shut-off".to_string());
        }
        
        // 4D change recommendation
        if four_d_summary.nrms_average > 20.0 {
            recommendations.push("Significant 4D changes - review sweep efficiency".to_string());
        }
        
        // Infills recommendation
        if self.production_data.len() < 5 && four_d_summary.max_change > 0.5 {
            recommendations.push("Consider infill drilling in areas with unswept oil".to_string());
        }
        
        recommendations
    }
}

/// Surveillance Report
pub struct SurveillanceReport {
    pub production_summary: ProductionSummary,
    pub four_d_summary: FourDSummary,
    pub recommendations: Vec<String>,
}

/// Production Summary
pub struct ProductionSummary {
    pub total_oil: f32,
    pub total_gas: f32,
    pub total_water: f32,
    pub water_cut: f32,
}

/// 4D Summary
pub struct FourDSummary {
    pub nrms_average: f32,
    pub max_change: f32,
    pub change_location: Option<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use sf_4d::ProductionData;

    #[test]
    fn test_surveillance_creation() {
        let surveillance = ReservoirSurveillance::new();
        assert!(surveillance.production_data.is_empty());
        assert!(surveillance.time_lapse_surveys.is_empty());
    }

    #[test]
    fn test_production_summary() {
        let mut surveillance = ReservoirSurveillance::new();
        
        let mut timeline = ProductionTimeline::new("Well-1".to_string());
        timeline.add(ProductionData {
            well_name: "Well-1".to_string(),
            date: "2024-01".to_string(),
            oil_rate: 1000.0,
            gas_rate: 500.0,
            water_rate: 100.0,
            pressure: 3000.0,
        });
        
        surveillance.add_production_data(timeline);
        
        let report = surveillance.generate_report();
        
        assert!((report.production_summary.total_oil - 1000.0).abs() < 0.01);
        assert!(report.production_summary.water_cut < 0.2);
    }

    #[test]
    fn test_four_d_analysis() {
        let mut surveillance = ReservoirSurveillance::new();
        
        let baseline = vec![1.0; 1000];
        let monitor = vec![1.1; 1000];
        
        surveillance.add_survey(baseline);
        surveillance.add_survey(monitor);
        
        let report = surveillance.generate_report();
        
        assert!(report.four_d_summary.nrms_average > 0.0);
    }

    #[test]
    fn test_recommendations() {
        let mut surveillance = ReservoirSurveillance::new();
        
        // Add high water cut well
        let mut timeline = ProductionTimeline::new("Well-1".to_string());
        timeline.add(ProductionData {
            well_name: "Well-1".to_string(),
            date: "2024-01".to_string(),
            oil_rate: 100.0,
            gas_rate: 50.0,
            water_rate: 900.0, // High water
            pressure: 3000.0,
        });
        
        surveillance.add_production_data(timeline);
        
        let report = surveillance.generate_report();
        
        assert!(!report.recommendations.is_empty());
        assert!(report.recommendations.iter().any(|r| r.contains("water")));
    }
}
