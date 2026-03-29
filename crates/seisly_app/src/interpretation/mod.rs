use serde::{Deserialize, Serialize};
use sf_core::domain::surface::Mesh;
use uuid::Uuid;

pub mod history;
pub mod velocity;
pub mod wells;

pub use history::HistoryManager;
pub use velocity::VelocityState;
pub use wells::WellState;

// InterpretationCommand is reserved for future history system
#[allow(unused_imports)]
pub use history::InterpretationCommand;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PickSource {
    Manual,
    AutoTracked,
    Seed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pick {
    pub id: Uuid,
    pub position: [f32; 3],
    pub confidence: f32,
    pub source: PickSource,
}

impl Pick {
    pub fn new(position: [f32; 3], source: PickSource) -> Self {
        Self {
            id: Uuid::new_v4(),
            position,
            confidence: 1.0,
            source,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Horizon {
    pub id: Uuid,
    pub name: String,
    pub picks: Vec<Pick>,
    pub color: [f32; 4], // RGBA with alpha for transparency
    pub is_visible: bool,
    #[serde(skip)]
    pub meshes: Vec<Mesh>,
    #[serde(skip)]
    pub intersection_lines: Vec<Vec<[f32; 3]>>,
}

impl Horizon {
    pub fn new(name: String, color: [f32; 4]) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            picks: Vec::new(),
            color,
            is_visible: true,
            meshes: Vec::new(),
            intersection_lines: Vec::new(),
        }
    }

    pub fn add_pick(&mut self, pick: Pick) {
        self.picks.push(pick);
    }

    pub fn update_mesh(&mut self) {
        if self.picks.len() < 3 {
            return;
        }

        use sf_compute::interpolation::{RbfInterpolator, RbfType};

        // Decimate points if there are too many for RBF (O(N^3))
        let max_rbf_points = 500;
        let points: Vec<[f32; 3]> = if self.picks.len() > max_rbf_points {
            let step = self.picks.len() / max_rbf_points;
            self.picks
                .iter()
                .step_by(step)
                .take(max_rbf_points)
                .map(|p| p.position)
                .collect()
        } else {
            self.picks.iter().map(|p| p.position).collect()
        };

        if let Ok(interp) = RbfInterpolator::new(&points, RbfType::ThinPlateSpline) {
            // Find bounds
            let mut min_x = f32::MAX;
            let mut max_x = f32::MIN;
            let mut min_y = f32::MAX;
            let mut max_y = f32::MIN;

            for p in &points {
                min_x = min_x.min(p[0]);
                max_x = max_x.max(p[0]);
                min_y = min_y.min(p[1]);
                max_y = max_y.max(p[1]);
            }

            // Expand bounds slightly
            let dx = ((max_x - min_x) * 0.1).max(10.0);
            let dy = ((max_y - min_y) * 0.1).max(10.0);

            self.meshes =
                vec![interp.generate_mesh(min_x - dx, max_x + dx, min_y - dy, max_y + dy, 20, 20)];
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PickingMode {
    None,
    Seed,
    AutoTrack,
    Manual,
    SketchFault,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaultStick {
    pub id: Uuid,
    pub picks: Vec<[f32; 3]>,
}

impl FaultStick {
    pub fn new(picks: Vec<[f32; 3]>) -> Self {
        Self {
            id: Uuid::new_v4(),
            picks,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fault {
    pub id: Uuid,
    pub name: String,
    pub color: [f32; 4], // RGBA with alpha for transparency
    pub sticks: Vec<FaultStick>,
    pub is_visible: bool,
    #[serde(skip)]
    pub meshes: Vec<Mesh>,
    #[serde(skip)]
    pub intersection_lines: Vec<Vec<[f32; 3]>>,
}

impl Fault {
    pub fn new(name: String, color: [f32; 4]) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            color,
            sticks: Vec::new(),
            is_visible: true,
            meshes: Vec::new(),
            intersection_lines: Vec::new(),
        }
    }

    /// Update the fault color (RGBA)
    #[allow(dead_code)]
    pub fn set_color(&mut self, color: [f32; 4]) {
        self.color = color;
    }

    /// Toggle visibility - reserved for future UI
    #[allow(dead_code)]
    pub fn set_visible(&mut self, visible: bool) {
        self.is_visible = visible;
    }

    /// Update fault name
    #[allow(dead_code)]
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn add_stick(&mut self, stick: FaultStick) {
        self.sticks.push(stick);
    }

    pub fn update_mesh(&mut self) {
        let mut points = Vec::new();
        for stick in &self.sticks {
            for &pick in &stick.picks {
                points.push(pick);
            }
        }

        if points.len() < 3 {
            return;
        }

        use sf_compute::interpolation::{RbfInterpolator, RbfType};

        if let Ok(interp) = RbfInterpolator::new(&points, RbfType::ThinPlateSpline) {
            self.meshes = vec![interp.generate_mesh_3d(20, 20)];
        }
    }
}

pub struct InterpretationState {
    pub horizons: Vec<Horizon>,
    pub faults: Vec<Fault>,
    pub active_horizon_id: Option<Uuid>,
    pub active_fault_id: Option<Uuid>,
    pub selected_horizon_ids: Vec<Uuid>,
    pub selected_fault_ids: Vec<Uuid>,
    pub picking_mode: PickingMode,
}

impl InterpretationState {
    pub fn new() -> Self {
        Self {
            horizons: Vec::new(),
            faults: Vec::new(),
            active_horizon_id: None,
            active_fault_id: None,
            selected_horizon_ids: Vec::new(),
            selected_fault_ids: Vec::new(),
            picking_mode: PickingMode::None,
        }
    }

    pub fn add_horizon(&mut self, horizon: Horizon) {
        self.horizons.push(horizon);
    }

    pub fn add_fault(&mut self, fault: Fault) {
        self.faults.push(fault);
    }

    /// Get active horizon - reserved for future UI
    #[allow(dead_code)]
    pub fn active_horizon(&self) -> Option<&Horizon> {
        self.active_horizon_id
            .and_then(|id| self.horizons.iter().find(|h| h.id == id))
    }

    pub fn active_horizon_mut(&mut self) -> Option<&mut Horizon> {
        self.active_horizon_id
            .and_then(|id| self.horizons.iter_mut().find(|h| h.id == id))
    }

    /// Get active fault - reserved for future UI
    #[allow(dead_code)]
    pub fn active_fault(&self) -> Option<&Fault> {
        self.active_fault_id
            .and_then(|id| self.faults.iter().find(|f| f.id == id))
    }

    /// Get mutable active fault - reserved for future UI
    #[allow(dead_code)]
    pub fn active_fault_mut(&mut self) -> Option<&mut Fault> {
        self.active_fault_id
            .and_then(|id| self.faults.iter_mut().find(|f| f.id == id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpretation_state_creation() {
        let mut state = InterpretationState::new();
        assert_eq!(state.horizons.len(), 0);
        assert!(state.active_horizon_id.is_none());

        let horizon = Horizon::new("H1".to_string(), [1.0, 0.0, 0.0, 0.7]);
        let h_id = horizon.id;
        state.add_horizon(horizon);
        state.active_horizon_id = Some(h_id);

        assert_eq!(state.horizons.len(), 1);
        assert_eq!(state.active_horizon().unwrap().name, "H1");
    }

    #[test]
    fn test_fault_sketching() {
        let mut fault = Fault::new("F1".to_string(), [1.0, 0.0, 0.0, 0.5]);
        let stick1 = FaultStick::new(vec![
            [10.0, 10.0, 250.0],
            [20.0, 20.0, 250.0],
            [30.0, 30.0, 250.0],
        ]);
        fault.add_stick(stick1);

        assert_eq!(fault.sticks.len(), 1);
        assert_eq!(fault.sticks[0].picks.len(), 3);

        // Add another stick to make it possible to form a surface
        let stick2 = FaultStick::new(vec![
            [10.0, 15.0, 250.0],
            [20.0, 25.0, 250.0],
            [30.0, 35.0, 250.0],
        ]);
        fault.add_stick(stick2);

        fault.update_mesh();
        assert!(!fault.meshes.is_empty());
        let mesh = &fault.meshes[0];
        assert!(mesh.vertices.len() > 0);
    }

    #[test]
    fn test_fault_color_rgba() {
        let mut fault = Fault::new("F1".to_string(), [1.0, 0.0, 0.0, 0.5]);
        assert_eq!(fault.color, [1.0, 0.0, 0.0, 0.5]);

        fault.set_color([0.0, 1.0, 0.0, 0.7]);
        assert_eq!(fault.color, [0.0, 1.0, 0.0, 0.7]);
    }

    #[test]
    fn test_fault_visibility() {
        let mut fault = Fault::new("F1".to_string(), [1.0, 0.0, 0.0, 0.5]);
        assert!(fault.is_visible);

        fault.set_visible(false);
        assert!(!fault.is_visible);

        fault.set_visible(true);
        assert!(fault.is_visible);
    }

    #[test]
    fn test_fault_name_update() {
        let mut fault = Fault::new("F1".to_string(), [1.0, 0.0, 0.0, 0.5]);
        assert_eq!(fault.name, "F1");

        fault.set_name("New Fault Name".to_string());
        assert_eq!(fault.name, "New Fault Name");
    }
}
