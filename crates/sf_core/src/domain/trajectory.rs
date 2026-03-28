//! Trajectory domain entity

use crate::types::EntityId;

/// Trajectory station
#[derive(Debug, Clone)]
pub struct Station {
    /// Measured depth (meters)
    pub md: f64,
    /// X coordinate (meters, in well CRS)
    pub x: f64,
    /// Y coordinate (meters, in well CRS)
    pub y: f64,
    /// Z coordinate (meters, in well CRS, positive down)
    pub z: f64,
}

/// Well trajectory
#[derive(Debug, Clone)]
pub struct Trajectory {
    pub id: EntityId,
    pub well_id: EntityId,
    pub stations: Vec<Station>,
}

impl Trajectory {
    pub fn new(well_id: EntityId) -> Self {
        Self {
            id: EntityId::new_v4(),
            well_id,
            stations: vec![],
        }
    }

    /// Add a station, maintaining MD order
    pub fn add_station(&mut self, md: f64, x: f64, y: f64, z: f64) {
        self.stations.push(Station { md, x, y, z });
        self.stations
            .sort_by(|a, b| a.md.partial_cmp(&b.md).unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trajectory_stations_ordered() {
        let well_id = EntityId::new_v4();
        let mut traj = Trajectory::new(well_id);
        traj.add_station(100.0, 0.0, 0.0, 100.0);
        traj.add_station(50.0, 0.0, 0.0, 50.0);
        traj.add_station(150.0, 0.0, 0.0, 150.0);

        assert_eq!(traj.stations.len(), 3);
        assert_eq!(traj.stations[0].md, 50.0);
        assert_eq!(traj.stations[1].md, 100.0);
        assert_eq!(traj.stations[2].md, 150.0);
    }
}
