//! Multi-Horizon Tracking

use sf_core::domain::surface::Surface;
use sf_ml::AutoTracker;

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
    pub fn track_all(&self, seismic: &dyn sf_compute::seismic::TraceProvider) -> Vec<Surface> {
        let mut surfaces = Vec::new();
        
        for (i, tracker) in self.trackers.iter().enumerate() {
            // Track from seed point
            // In production: use actual seed points
            let surface = tracker.track(seismic, 50, 50, 1000.0).unwrap_or_else(|_| {
                Surface::new(format!("Surface_{}", i))
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
            for j in (i+1)..surfaces.len() {
                // Ensure surface[i] is above surface[j]
                // In production: implement proper constraint enforcement
                self.prevent_crossing(&mut surfaces[i], &mut surfaces[j]);
            }
        }
    }
    
    /// Prevent horizon crossing
    fn prevent_crossing(&self, upper: &mut Surface, lower: &mut Surface) {
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
        horizon1: usize,
        horizon2: usize,
        relationship: HorizonRelationship,
    ) {
        // In production: enforce relationship during tracking
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multi_horizon_creation() {
        let tracker = MultiHorizonTracker::new();
        assert_eq!(tracker.stratigraphic_order.len(), 0);
    }

    #[test]
    fn test_add_horizon() {
        let mut tracker = MultiHorizonTracker::new();
        
        // Create dummy tracker
        // In production: add actual AutoTracker
        
        tracker.add_horizon(
            AutoTracker::new(sf_ml::HorizonCNN::new(
                candle_nn::VarBuilder::zeros(
                    candle_nn::VarBuilderArgs::default(),
                    &candle_core::Device::Cpu,
                ).unwrap()
            ).unwrap()),
            "Top Reservoir".to_string(),
        );
        
        assert_eq!(tracker.stratigraphic_order.len(), 1);
    }
}
