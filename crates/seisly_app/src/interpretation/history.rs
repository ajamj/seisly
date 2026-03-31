//! History Management for Interpretation Operations
//!
//! This module provides undo/redo functionality for interpretation operations
//! using the command pattern from seisly_core.

use seisly_core::Command;
use std::any::Any;

use super::{FaultStick, InterpretationState, Pick};
use uuid::Uuid;

/// Marker trait for interpretation-specific commands
/// This allows us to work with the Any trait for downcasting
pub trait InterpretationCommand: Command {}

// ============================================================================
// Horizon Pick Commands
// ============================================================================

/// Add a single pick to a horizon
pub struct AddPickCommand {
    horizon_id: Uuid,
    pick: Pick,
}

impl AddPickCommand {
    pub fn new(horizon_id: Uuid, pick: Pick) -> Self {
        Self { horizon_id, pick }
    }
}

impl Command for AddPickCommand {
    fn execute(&mut self, target: &mut dyn Any) {
        if let Some(state) = target.downcast_mut::<InterpretationState>() {
            if let Some(horizon) = state.horizons.iter_mut().find(|h| h.id == self.horizon_id) {
                horizon.add_pick(self.pick.clone());
                horizon.update_mesh();
            }
        }
    }

    fn undo(&mut self, target: &mut dyn Any) {
        if let Some(state) = target.downcast_mut::<InterpretationState>() {
            if let Some(horizon) = state.horizons.iter_mut().find(|h| h.id == self.horizon_id) {
                horizon.picks.retain(|p| p.id != self.pick.id);
                horizon.update_mesh();
            }
        }
    }

    fn name(&self) -> &'static str {
        "AddPickCommand"
    }
}

impl InterpretationCommand for AddPickCommand {}

/// Delete a pick from a horizon
pub struct DeletePickCommand {
    horizon_id: Uuid,
    pick: Pick,
    index: usize,
}

impl DeletePickCommand {
    pub fn new(horizon_id: Uuid, pick: Pick, index: usize) -> Self {
        Self {
            horizon_id,
            pick,
            index,
        }
    }
}

impl Command for DeletePickCommand {
    fn execute(&mut self, target: &mut dyn Any) {
        if let Some(state) = target.downcast_mut::<InterpretationState>() {
            if let Some(horizon) = state.horizons.iter_mut().find(|h| h.id == self.horizon_id) {
                if self.index < horizon.picks.len() {
                    horizon.picks.remove(self.index);
                    horizon.update_mesh();
                }
            }
        }
    }

    fn undo(&mut self, target: &mut dyn Any) {
        if let Some(state) = target.downcast_mut::<InterpretationState>() {
            if let Some(horizon) = state.horizons.iter_mut().find(|h| h.id == self.horizon_id) {
                horizon.picks.insert(self.index, self.pick.clone());
                horizon.update_mesh();
            }
        }
    }

    fn name(&self) -> &'static str {
        "DeletePickCommand"
    }
}

impl InterpretationCommand for DeletePickCommand {}

/// Add multiple picks from auto-tracking
pub struct AutoTrackCommand {
    horizon_id: Uuid,
    picks: Vec<Pick>,
}

impl AutoTrackCommand {
    pub fn new(horizon_id: Uuid, picks: Vec<Pick>) -> Self {
        Self { horizon_id, picks }
    }
}

impl Command for AutoTrackCommand {
    fn execute(&mut self, target: &mut dyn Any) {
        if let Some(state) = target.downcast_mut::<InterpretationState>() {
            if let Some(horizon) = state.horizons.iter_mut().find(|h| h.id == self.horizon_id) {
                for pick in &self.picks {
                    horizon.add_pick(pick.clone());
                }
                horizon.update_mesh();
            }
        }
    }

    fn undo(&mut self, target: &mut dyn Any) {
        if let Some(state) = target.downcast_mut::<InterpretationState>() {
            if let Some(horizon) = state.horizons.iter_mut().find(|h| h.id == self.horizon_id) {
                let pick_ids: std::collections::HashSet<_> =
                    self.picks.iter().map(|p| p.id).collect();
                horizon.picks.retain(|p| !pick_ids.contains(&p.id));
                horizon.update_mesh();
            }
        }
    }

    fn name(&self) -> &'static str {
        "AutoTrackCommand"
    }
}

impl InterpretationCommand for AutoTrackCommand {}

// ============================================================================
// Fault Commands
// ============================================================================

/// Add a fault stick to a fault
pub struct AddFaultStickCommand {
    fault_id: Uuid,
    stick: FaultStick,
}

impl AddFaultStickCommand {
    pub fn new(fault_id: Uuid, stick: FaultStick) -> Self {
        Self { fault_id, stick }
    }
}

impl Command for AddFaultStickCommand {
    fn execute(&mut self, target: &mut dyn Any) {
        if let Some(state) = target.downcast_mut::<InterpretationState>() {
            if let Some(fault) = state.faults.iter_mut().find(|f| f.id == self.fault_id) {
                fault.add_stick(self.stick.clone());
                fault.update_mesh();
            }
        }
    }

    fn undo(&mut self, target: &mut dyn Any) {
        if let Some(state) = target.downcast_mut::<InterpretationState>() {
            if let Some(fault) = state.faults.iter_mut().find(|f| f.id == self.fault_id) {
                fault.sticks.retain(|s| s.id != self.stick.id);
                fault.update_mesh();
            }
        }
    }

    fn name(&self) -> &'static str {
        "AddFaultStickCommand"
    }
}

impl InterpretationCommand for AddFaultStickCommand {}

// ============================================================================
// History Manager
// ============================================================================

/// Manages undo/redo history for interpretation operations
pub struct HistoryManager {
    undo_stack: Vec<Box<dyn Command>>,
    redo_stack: Vec<Box<dyn Command>>,
    max_stack_size: usize,
}

impl Default for HistoryManager {
    fn default() -> Self {
        Self::new(100)
    }
}

impl HistoryManager {
    /// Create a new history manager with a maximum stack size
    pub fn new(max_size: usize) -> Self {
        Self {
            undo_stack: Vec::with_capacity(max_size.min(100)),
            redo_stack: Vec::new(),
            max_stack_size: max_size,
        }
    }

    /// Execute a command and push it onto the undo stack
    pub fn execute(&mut self, mut command: Box<dyn Command>, state: &mut InterpretationState) {
        command.execute(state);
        self.undo_stack.push(command);
        self.redo_stack.clear();

        // Trim stack if it exceeds max size
        if self.undo_stack.len() > self.max_stack_size {
            self.undo_stack.remove(0);
        }
    }

    /// Undo the last executed command
    pub fn undo(&mut self, state: &mut InterpretationState) -> bool {
        if let Some(mut command) = self.undo_stack.pop() {
            command.undo(state);
            self.redo_stack.push(command);
            true
        } else {
            false
        }
    }

    /// Redo a previously undone command
    pub fn redo(&mut self, state: &mut InterpretationState) -> bool {
        if let Some(mut command) = self.redo_stack.pop() {
            command.execute(state);
            self.undo_stack.push(command);
            true
        } else {
            false
        }
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Get the number of undoable operations
    #[allow(dead_code)]
    pub fn undo_count(&self) -> usize {
        self.undo_stack.len()
    }

    /// Get the number of redoable operations
    #[allow(dead_code)]
    pub fn redo_count(&self) -> usize {
        self.redo_stack.len()
    }

    /// Clear all history
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpretation::{Horizon, PickSource};

    #[test]
    fn test_add_pick_command() {
        let mut state = InterpretationState::new();
        let horizon = Horizon::new("H1".to_string(), [1.0, 0.0, 0.0, 0.7]);
        let h_id = horizon.id;
        state.add_horizon(horizon);
        state.active_horizon_id = Some(h_id);

        let mut command = AddPickCommand::new(
            h_id,
            Pick::new([100.0, 200.0, 1500.0], PickSource::Manual),
        );

        // Execute
        command.execute(&mut state);
        assert_eq!(state.active_horizon().unwrap().picks.len(), 1);

        // Undo
        command.undo(&mut state);
        assert_eq!(state.active_horizon().unwrap().picks.len(), 0);
    }

    #[test]
    fn test_history_manager_basic() {
        let mut manager = HistoryManager::new(10);
        let mut state = InterpretationState::new();
        
        let horizon = Horizon::new("H1".to_string(), [1.0, 0.0, 0.0, 0.7]);
        let h_id = horizon.id;
        state.add_horizon(horizon);
        state.active_horizon_id = Some(h_id);

        // Add pick via history manager
        let command = Box::new(AddPickCommand::new(
            h_id,
            Pick::new([100.0, 200.0, 1500.0], PickSource::Manual),
        ));
        manager.execute(command, &mut state);

        assert!(manager.can_undo());
        assert!(!manager.can_redo());
        assert_eq!(state.active_horizon().unwrap().picks.len(), 1);

        // Undo
        manager.undo(&mut state);
        assert!(!manager.can_undo());
        assert!(manager.can_redo());
        assert_eq!(state.active_horizon().unwrap().picks.len(), 0);

        // Redo
        manager.redo(&mut state);
        assert!(manager.can_undo());
        assert!(!manager.can_redo());
        assert_eq!(state.active_horizon().unwrap().picks.len(), 1);
    }

    #[test]
    fn test_auto_track_command() {
        let mut state = InterpretationState::new();
        let horizon = Horizon::new("H1".to_string(), [1.0, 0.0, 0.0, 0.7]);
        let h_id = horizon.id;
        state.add_horizon(horizon);
        state.active_horizon_id = Some(h_id);

        let picks = vec![
            Pick::new([100.0, 200.0, 1500.0], PickSource::AutoTracked),
            Pick::new([110.0, 210.0, 1510.0], PickSource::AutoTracked),
            Pick::new([120.0, 220.0, 1520.0], PickSource::AutoTracked),
        ];

        let mut command = AutoTrackCommand::new(h_id, picks);

        // Execute
        command.execute(&mut state);
        assert_eq!(state.active_horizon().unwrap().picks.len(), 3);

        // Undo
        command.undo(&mut state);
        assert_eq!(state.active_horizon().unwrap().picks.len(), 0);
    }
}
