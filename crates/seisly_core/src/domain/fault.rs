//! Fault domain entity

use crate::types::EntityId;

/// A fault stick representing a line segment on a 2D seismic section
#[derive(Debug, Clone)]
pub struct FaultStick {
    pub id: EntityId,
    pub points: Vec<[f32; 3]>,
}

impl FaultStick {
    pub fn new(points: Vec<[f32; 3]>) -> Self {
        Self {
            id: EntityId::new_v4(),
            points,
        }
    }
}

/// A 3D fault surface composed of sticks
#[derive(Debug, Clone)]
pub struct Fault {
    pub id: EntityId,
    pub name: String,
    pub color: [f32; 4],
    pub sticks: Vec<FaultStick>,
    pub is_visible: bool,
}

impl Fault {
    pub fn new(name: String, color: [f32; 4]) -> Self {
        Self {
            id: EntityId::new_v4(),
            name,
            color,
            sticks: vec![],
            is_visible: true,
        }
    }

    pub fn add_stick(&mut self, stick: FaultStick) {
        self.sticks.push(stick);
    }
}
