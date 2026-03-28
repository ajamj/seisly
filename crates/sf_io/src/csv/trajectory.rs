//! Trajectory CSV parser

use sf_core::domain::trajectory::Trajectory;
use sf_core::EntityId;
use std::io::{BufRead, BufReader};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TrajError {
    #[error("Failed to read CSV: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Invalid CSV format: {0}")]
    ParseError(String),
    #[error("No data rows found")]
    NoData,
}

pub struct TrajectoryParser;

impl TrajectoryParser {
    pub fn parse(path: &Path, well_id: EntityId) -> Result<Trajectory, TrajError> {
        let file = std::fs::File::open(path)?;
        let reader = BufReader::new(file);

        let mut traj = Trajectory::new(well_id);
        let mut is_header = true;
        let mut data_count = 0;

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if is_header {
                // Check if first line looks like a header
                if line.to_lowercase().contains("md")
                    || line.to_lowercase().starts_with("md,")
                    || line.to_lowercase().starts_with("md,x,y,z")
                {
                    is_header = false;
                    continue;
                }
                is_header = false;
            }

            // Parse data row: md,x,y,z
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 4 {
                let md: f64 = parts[0]
                    .trim()
                    .parse()
                    .map_err(|_| TrajError::ParseError(format!("Invalid MD: {}", parts[0])))?;
                let x: f64 = parts[1]
                    .trim()
                    .parse()
                    .map_err(|_| TrajError::ParseError(format!("Invalid X: {}", parts[1])))?;
                let y: f64 = parts[2]
                    .trim()
                    .parse()
                    .map_err(|_| TrajError::ParseError(format!("Invalid Y: {}", parts[2])))?;
                let z: f64 = parts[3]
                    .trim()
                    .parse()
                    .map_err(|_| TrajError::ParseError(format!("Invalid Z: {}", parts[3])))?;

                traj.add_station(md, x, y, z);
                data_count += 1;
            }
        }

        if data_count == 0 {
            return Err(TrajError::NoData);
        }

        Ok(traj)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    fn create_test_csv(content: &str) -> TempDir {
        let temp_dir = TempDir::new().unwrap();
        let csv_path = temp_dir.path().join("test.csv");
        let mut file = std::fs::File::create(&csv_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        temp_dir
    }

    #[test]
    fn test_parse_trajectory_with_header() {
        let csv_content = "md,x,y,z
0,500000,1000000,0
50,500000,1000000,50
100,500001,1000001,100
150,500002,1000002,150
200,500003,1000003,200
";
        let temp_dir = create_test_csv(csv_content);
        let csv_path = temp_dir.path().join("test.csv");
        let well_id = EntityId::new_v4();

        let traj = TrajectoryParser::parse(&csv_path, well_id).unwrap();
        assert_eq!(traj.stations.len(), 5);
        assert_eq!(traj.stations[0].md, 0.0);
        assert_eq!(traj.stations[4].md, 200.0);
    }

    #[test]
    fn test_parse_trajectory_no_header() {
        let csv_content = "0,500000,1000000,0
100,500001,1000001,100
200,500002,1000002,200
";
        let temp_dir = create_test_csv(csv_content);
        let csv_path = temp_dir.path().join("test.csv");
        let well_id = EntityId::new_v4();

        let traj = TrajectoryParser::parse(&csv_path, well_id).unwrap();
        assert_eq!(traj.stations.len(), 3);
    }

    #[test]
    fn test_parse_trajectory_empty() {
        let csv_content = "md,x,y,z
";
        let temp_dir = create_test_csv(csv_content);
        let csv_path = temp_dir.path().join("test.csv");
        let well_id = EntityId::new_v4();

        let result = TrajectoryParser::parse(&csv_path, well_id);
        assert!(result.is_err());
    }
}
