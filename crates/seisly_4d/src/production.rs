//! Production Data Integration

/// Well Production Data
#[derive(Debug, Clone)]
pub struct ProductionData {
    pub well_name: String,
    pub date: String,
    pub oil_rate: f32,    // STB/day
    pub gas_rate: f32,    // MSCF/day
    pub water_rate: f32,  // STB/day
    pub pressure: f32,    // psi
}

impl ProductionData {
    /// Compute water cut
    pub fn water_cut(&self) -> f32 {
        let total_liquid = self.oil_rate + self.water_rate;
        if total_liquid.abs() < 1e-10 {
            0.0
        } else {
            self.water_rate / total_liquid
        }
    }
    
    /// Compute gas-oil ratio (GOR)
    pub fn gor(&self) -> f32 {
        if self.oil_rate.abs() < 1e-10 {
            0.0
        } else {
            self.gas_rate / self.oil_rate
        }
    }
}

/// Production Timeline
pub struct ProductionTimeline {
    pub well_name: String,
    pub data: Vec<ProductionData>,
}

impl ProductionTimeline {
    pub fn new(well_name: String) -> Self {
        Self { well_name, data: Vec::new() }
    }
    
    pub fn add(&mut self, data: ProductionData) {
        self.data.push(data);
    }
    
    /// Get cumulative production
    pub fn cumulative(&self) -> (f32, f32, f32) {
        let mut oil = 0.0;
        let mut gas = 0.0;
        let mut water = 0.0;
        
        for d in &self.data {
            oil += d.oil_rate;
            gas += d.gas_rate;
            water += d.water_rate;
        }
        
        (oil, gas, water)
    }
}

/// 4D Production Integration
pub struct ProductionIntegration {
    timelines: Vec<ProductionTimeline>,
}

impl ProductionIntegration {
    pub fn new() -> Self {
        Self { timelines: Vec::new() }
    }
    
    pub fn add_well(&mut self, timeline: ProductionTimeline) {
        self.timelines.push(timeline);
    }
    
    /// Correlate production changes with 4D signal
    pub fn correlate_with_4d(&self, _4d_signal: &[f32]) -> f32 {
        // Simplified correlation
        // In production: use actual time-lapse and production data
        if _4d_signal.is_empty() {
            return 0.0;
        }
        
        let signal_strength: f32 = _4d_signal.iter().map(|x| x.abs()).sum();
        signal_strength / _4d_signal.len() as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_water_cut() {
        let data = ProductionData {
            well_name: "Well-1".to_string(),
            date: "2024-01".to_string(),
            oil_rate: 800.0,
            gas_rate: 500.0,
            water_rate: 200.0,
            pressure: 3000.0,
        };
        
        let wc = data.water_cut();
        assert!((wc - 0.2).abs() < 0.01); // 20% water cut
    }

    #[test]
    fn test_gor() {
        let data = ProductionData {
            well_name: "Well-1".to_string(),
            date: "2024-01".to_string(),
            oil_rate: 1000.0,
            gas_rate: 2000.0,
            water_rate: 100.0,
            pressure: 3000.0,
        };
        
        let gor = data.gor();
        assert!((gor - 2.0).abs() < 0.01); // 2 MSCF/STB
    }

    #[test]
    fn test_cumulative_production() {
        let mut timeline = ProductionTimeline::new("Well-1".to_string());
        timeline.add(ProductionData {
            well_name: "Well-1".to_string(),
            date: "2024-01".to_string(),
            oil_rate: 1000.0,
            gas_rate: 500.0,
            water_rate: 100.0,
            pressure: 3000.0,
        });
        timeline.add(ProductionData {
            well_name: "Well-1".to_string(),
            date: "2024-02".to_string(),
            oil_rate: 900.0,
            gas_rate: 600.0,
            water_rate: 150.0,
            pressure: 2800.0,
        });
        
        let (oil, gas, water) = timeline.cumulative();
        assert!((oil - 1900.0).abs() < 0.01);
        assert!((gas - 1100.0).abs() < 0.01);
    }
}
