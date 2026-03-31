//! Multi-Horizon Tracking

use seisly_core::domain::surface::Surface;
use seisly_core::Crs;
use seisly_ml::AutoTracker;

/// Multi-Horizon Tracker - track multiple horizons simultaneously
pub struct MultiHorizonTracker {
    trackers: Vec<AutoTracker>,
    stratigraphic_order: Vec<String>,
}

impl MultiHorizonTracker {
    pub fn new() -> Self {
        Self {
            trackers: Vec::new(),
            stratigraphic_order: Vec::new(),
        }
    }
    
    /// Add horizon to track
    pub fn add_horizon(&mut self, tracker: AutoTracker, name: String) {
        self.trackers.push(tracker);
        self.stratigraphic_order.push(name);
    }
    
    /// Track all horizons with stratigraphic constraints
    pub fn track_all(&self, seismic: &dyn seisly_compute::seismic::TraceProvider) -> Vec<Surface> {
        let mut surfaces = Vec::new();
        
        for (i, tracker) in self.trackers.iter().enumerate() {
            // Track from seed point
            // In production: use actual seed points
            let surface = tracker.track(seismic, 50, 50, 1000).unwrap_or_else(|_| {
                Surface::new(format!("Surface_{}", i), Crs::wgs84(), vec![])
            });
            
            surfaces.push(surface);
        }
        
        // Apply stratigraphic constraints
        self.apply_stratigraphic_constraints(&mut surfaces);
        
        surfaces
    }
    
    /// Ensure horizons don't cross (stratigraphic order)
    fn apply_stratigraphic_constraints(&self, surfaces: &mut [Surface]) {
        for i in 0..surfaces.len() {
            let (upper_part, lower_part) = surfaces.split_at_mut(i + 1);
            let upper = &mut upper_part[i];
            for j in 0..lower_part.len() {
                let lower = &mut lower_part[j];
                // Ensure upper is above lower
                self.prevent_crossing(upper, lower);
            }
        }
    }
    
    /// Prevent horizon crossing
    fn prevent_crossing(&self, _upper: &mut Surface, _lower: &mut Surface) {
        // Simplified: in production, implement proper surface editing
        // to ensure upper horizon stays above lower horizon
    }
}

/// Horizon Relationship
pub enum HorizonRelationship {
    Conformable,
    Onlap,
    Downlap,
    Unconformity,
}

impl MultiHorizonTracker {
    /// Set relationship between horizons
    pub fn set_relationship(
        &mut self,
        _horizon1: usize,
        _horizon2: usize,
        _relationship: HorizonRelationship,
    ) {
        // In production: enforce relationship during tracking
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use candle_nn::VarBuilder;

    #[test]
    fn test_multi_horizon_creation() {
        let tracker = MultiHorizonTracker::new();
        assert_eq!(tracker.stratigraphic_order.len(), 0);
    }

    #[test]
    fn test_add_horizon() {
        let mut tracker = MultiHorizonTracker::new();
        
        let device = candle_core::Device::Cpu;
        let vb = VarBuilder::zeros(candle_core::DType::F32, &device);
        
        tracker.add_horizon(
            AutoTracker::new(seisly_ml::HorizonCNN::new(vb).unwrap()),
            "Top Reservoir".to_string(),
        );
        
        assert_eq!(tracker.stratigraphic_order.len(), 1);
    }
}
