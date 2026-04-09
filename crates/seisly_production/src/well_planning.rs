//! Well Planning Tools

use seisly_core::domain::trajectory::Trajectory;
use seisly_core::types::EntityId;

/// Well Planner - design well trajectories
pub struct WellPlanner {
    surface_location: (f64, f64),           // (easting, northing)
    target_locations: Vec<(f64, f64, f64)>, // (easting, northing, tvd)
}

impl WellPlanner {
    pub fn new(surface_location: (f64, f64)) -> Self {
        Self {
            surface_location,
            target_locations: Vec::new(),
        }
    }

    /// Add target for well planning
    pub fn add_target(&mut self, easting: f64, northing: f64, tvd: f64) {
        self.target_locations.push((easting, northing, tvd));
    }

    /// Design well trajectory
    pub fn design_trajectory(&self, _well_name: &str) -> Result<Trajectory, String> {
        if self.target_locations.is_empty() {
            return Err("No targets defined".to_string());
        }

        // Simple straight-line trajectory (simplified)
        // In production: implement proper well planning

        let well_id = EntityId::new_v4();
        let mut trajectory = Trajectory::new(well_id);

        // Surface location
        trajectory.add_station(0.0, self.surface_location.0, self.surface_location.1, 0.0);

        // Target points
        for (i, (easting, northing, tvd)) in self.target_locations.iter().enumerate() {
            let md = self.calculate_measured_depth(i, *tvd);
            trajectory.add_station(md, *easting, *northing, *tvd);
        }

        Ok(trajectory)
    }

    /// Calculate measured depth from TVD (simplified)
    fn calculate_measured_depth(&self, _target_idx: usize, tvd: f64) -> f64 {
        // Simplified: assume 10% longer than TVD due to deviation
        tvd * 1.1
    }

    /// Optimize well placement for reservoir drainage
    pub fn optimize_well_placement(
        &mut self,
        reservoir_bounds: ((f64, f64), (f64, f64)),
        num_wells: usize,
    ) -> Vec<(f64, f64)> {
        // Simple grid-based placement
        // In production: use reservoir simulation for optimization

        let mut well_locations = Vec::new();
        let ((min_e, min_n), (max_e, max_n)) = reservoir_bounds;

        let spacing_e = (max_e - min_e) / (num_wells as f64).sqrt();
        let spacing_n = (max_n - min_n) / (num_wells as f64).sqrt();

        for i in 0..(num_wells as f64).sqrt() as usize {
            for j in 0..(num_wells as f64).sqrt() as usize {
                let easting = min_e + (i as f64 + 0.5) * spacing_e;
                let northing = min_n + (j as f64 + 0.5) * spacing_n;
                well_locations.push((easting, northing));
            }
        }

        well_locations
    }
}

/// Wellbore Stability Analyzer
pub struct WellboreStability;

impl WellboreStability {
    /// Analyze wellbore stability
    pub fn analyze(_trajectory: &Trajectory, _formation_pressures: &[f64]) -> StabilityReport {
        // In production: implement proper wellbore stability analysis
        StabilityReport {
            stable: true,
            mud_weight_min: 9.0,
            mud_weight_max: 15.0,
        }
    }
}

/// Stability Report
pub struct StabilityReport {
    pub stable: bool,
    pub mud_weight_min: f64, // ppg
    pub mud_weight_max: f64, // ppg
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_well_planner_creation() {
        let planner = WellPlanner::new((500000.0, 1000000.0));
        assert_eq!(planner.target_locations.len(), 0);
    }

    #[test]
    fn test_add_target() {
        let mut planner = WellPlanner::new((500000.0, 1000000.0));
        planner.add_target(500100.0, 1000100.0, 2500.0);

        assert_eq!(planner.target_locations.len(), 1);
    }

    #[test]
    fn test_trajectory_design() {
        let mut planner = WellPlanner::new((500000.0, 1000000.0));
        planner.add_target(500100.0, 1000100.0, 2500.0);

        let trajectory = planner.design_trajectory("Well-1");
        assert!(trajectory.is_ok());

        let traj = trajectory.unwrap();
        assert!(traj.stations.len() >= 2); // Surface + target
    }

    #[test]
    fn test_well_placement_optimization() {
        let mut planner = WellPlanner::new((500000.0, 1000000.0));
        let bounds = ((500000.0, 1000000.0), (502000.0, 1002000.0));

        let locations = planner.optimize_well_placement(bounds, 4);
        assert_eq!(locations.len(), 4);
    }
}
