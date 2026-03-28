//! Trajectory resampling utilities

use sf_core::domain::trajectory::{Station, Trajectory};

/// Resample trajectory to uniform MD intervals
///
/// Uses linear interpolation between stations.
pub fn resample_trajectory(traj: &Trajectory, interval: f64) -> Trajectory {
    if traj.stations.is_empty() {
        return traj.clone();
    }

    let mut resampled = Trajectory::new(traj.well_id);

    let md_min = traj.stations.first().unwrap().md;
    let md_max = traj.stations.last().unwrap().md;

    let mut current_md = md_min;
    while current_md <= md_max {
        let station = interpolate_station(traj, current_md);
        resampled.add_station(station.md, station.x, station.y, station.z);
        current_md += interval;
    }

    // Ensure last station is included
    if let Some(last) = traj.stations.last() {
        if resampled.stations.is_empty() || resampled.stations.last().unwrap().md < last.md {
            resampled.add_station(last.md, last.x, last.y, last.z);
        }
    }

    resampled
}

/// Interpolate a station at a specific MD
fn interpolate_station(traj: &Trajectory, md: f64) -> Station {
    // Find surrounding stations
    for i in 0..traj.stations.len().saturating_sub(1) {
        let s0 = &traj.stations[i];
        let s1 = &traj.stations[i + 1];

        if s0.md <= md && md <= s1.md {
            if (s1.md - s0.md).abs() < f64::EPSILON {
                return s0.clone();
            }

            let t = (md - s0.md) / (s1.md - s0.md);
            return Station {
                md,
                x: s0.x + t * (s1.x - s0.x),
                y: s0.y + t * (s1.y - s0.y),
                z: s0.z + t * (s1.z - s0.z),
            };
        }
    }

    // Return closest station if interpolation not possible
    traj.stations.last().unwrap().clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use sf_core::EntityId;

    #[test]
    fn test_resample_uniform() {
        let mut traj = Trajectory::new(EntityId::new_v4());
        traj.add_station(0.0, 0.0, 0.0, 0.0);
        traj.add_station(100.0, 10.0, 10.0, 100.0);
        traj.add_station(200.0, 20.0, 20.0, 200.0);

        let resampled = resample_trajectory(&traj, 50.0);

        assert_eq!(resampled.stations.len(), 5); // 0, 50, 100, 150, 200
        assert_eq!(resampled.stations[1].md, 50.0);
        assert_eq!(resampled.stations[2].md, 100.0);
        assert_eq!(resampled.stations[3].md, 150.0);
    }

    #[test]
    fn test_resample_empty() {
        let traj = Trajectory::new(EntityId::new_v4());
        let resampled = resample_trajectory(&traj, 10.0);
        assert!(resampled.stations.is_empty());
    }

    #[test]
    fn test_interpolation_values() {
        let mut traj = Trajectory::new(EntityId::new_v4());
        traj.add_station(0.0, 0.0, 0.0, 0.0);
        traj.add_station(100.0, 10.0, 10.0, 100.0);

        let resampled = resample_trajectory(&traj, 50.0);

        // At MD=50, should be halfway between stations
        assert_eq!(resampled.stations.len(), 3); // 0, 50, 100
        let mid = &resampled.stations[1];
        assert!((mid.x - 5.0).abs() < 0.001);
        assert!((mid.y - 5.0).abs() < 0.001);
        assert!((mid.z - 50.0).abs() < 0.001);
    }
}
