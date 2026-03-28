//! Well State Management

use sf_core::domain::well::Well;

/// Well interpretation state
#[derive(Debug, Clone)]
pub struct WellState {
    pub wells: Vec<Well>,
    pub active_well_id: Option<uuid::Uuid>,
}

impl WellState {
    pub fn new() -> Self {
        Self {
            wells: Vec::new(),
            active_well_id: None,
        }
    }

    pub fn add_well(&mut self, well: Well) {
        self.active_well_id = Some(well.id);
        self.wells.push(well);
    }

    #[allow(dead_code)] // Reserved for future well management UI
    pub fn remove_well(&mut self, well_id: uuid::Uuid) {
        self.wells.retain(|w| w.id != well_id);
        if self.active_well_id == Some(well_id) {
            self.active_well_id = self.wells.first().map(|w| w.id);
        }
    }

    pub fn active_well(&self) -> Option<&Well> {
        self.active_well_id
            .and_then(|id| self.wells.iter().find(|w| w.id == id))
    }

    #[allow(dead_code)] // Reserved for future well editing
    pub fn active_well_mut(&mut self) -> Option<&mut Well> {
        self.active_well_id
            .and_then(|id| self.wells.iter_mut().find(|w| w.id == id))
    }

    #[allow(dead_code)] // Reserved for future well lookup
    pub fn get_well(&self, well_id: uuid::Uuid) -> Option<&Well> {
        self.wells.iter().find(|w| w.id == well_id)
    }
}

impl Default for WellState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sf_core::domain::well::Well;

    #[test]
    fn test_well_state_creation() {
        let state = WellState::new();
        assert!(state.wells.is_empty());
        assert!(state.active_well_id.is_none());
    }

    #[test]
    fn test_add_well() {
        let mut state = WellState::new();
        let well = Well::new(
            "Well-1".to_string(),
            "W1".to_string(),
            500000.0,
            1000000.0,
            100.0,
        );
        let well_id = well.id;

        state.add_well(well);

        assert_eq!(state.wells.len(), 1);
        assert_eq!(state.active_well_id, Some(well_id));
        assert!(state.active_well().is_some());
    }

    #[test]
    fn test_remove_well() {
        let mut state = WellState::new();
        let well1 = Well::new(
            "Well-1".to_string(),
            "W1".to_string(),
            500000.0,
            1000000.0,
            100.0,
        );
        let well2 = Well::new(
            "Well-2".to_string(),
            "W2".to_string(),
            500100.0,
            1000100.0,
            105.0,
        );
        let well1_id = well1.id;

        state.add_well(well1);
        state.add_well(well2);

        assert_eq!(state.wells.len(), 2);

        state.remove_well(well1_id);

        assert_eq!(state.wells.len(), 1);
        assert!(state.get_well(well1_id).is_none());
        assert!(state.get_well(state.wells[0].id).is_some());
    }
}
