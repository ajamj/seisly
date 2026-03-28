//! Log domain entity

use crate::types::EntityId;

/// Log curve
#[derive(Debug, Clone)]
pub struct Curve {
    pub mnemonic: String,
    pub unit: String,
    pub values: Vec<f32>,
    pub null_value: f32,
}

/// Depth mnemonic for logs
#[derive(Debug, Clone)]
pub enum DepthMnemonic {
    MD,  // Measured Depth
    TVD, // True Vertical Depth
}

/// Well log
#[derive(Debug, Clone)]
pub struct Log {
    pub id: EntityId,
    pub well_id: EntityId,
    pub depth_mnemonic: DepthMnemonic,
    pub depth_unit: String,
    pub curves: Vec<Curve>,
}

impl Log {
    pub fn new(well_id: EntityId, depth_mnemonic: DepthMnemonic, depth_unit: String) -> Self {
        Self {
            id: EntityId::new_v4(),
            well_id,
            depth_mnemonic,
            depth_unit,
            curves: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_creation() {
        let well_id = EntityId::new_v4();
        let log = Log::new(well_id, DepthMnemonic::MD, "M".to_string());
        assert!(log.curves.is_empty());
        assert_eq!(log.depth_unit, "M");
    }
}
