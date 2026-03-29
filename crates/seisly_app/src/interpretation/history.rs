#[allow(unused_imports)]
use super::{Fault, FaultStick, Horizon};
use super::{InterpretationState, Pick};
use uuid::Uuid;

pub trait InterpretationCommand {
    fn execute(&mut self, state: &mut InterpretationState);
    fn undo(&mut self, state: &mut InterpretationState);
}

// History system commands - reserved for future undo/redo feature
#[allow(dead_code)]
pub struct AddPickCommand {
    horizon_id: Uuid,
    pick: Pick,
}

#[allow(dead_code)]
impl AddPickCommand {
    pub fn new(horizon_id: Uuid, pick: Pick) -> Self {
        Self { horizon_id, pick }
    }
}

impl InterpretationCommand for AddPickCommand {
    fn execute(&mut self, state: &mut InterpretationState) {
        if let Some(horizon) = state.horizons.iter_mut().find(|h| h.id == self.horizon_id) {
            horizon.add_pick(self.pick.clone());
            horizon.update_mesh();
        }
    }

    fn undo(&mut self, state: &mut InterpretationState) {
        if let Some(horizon) = state.horizons.iter_mut().find(|h| h.id == self.horizon_id) {
            horizon.picks.retain(|p| p.id != self.pick.id);
            horizon.update_mesh();
        }
    }
}

#[allow(dead_code)]
pub struct DeletePickCommand {
    horizon_id: Uuid,
    pick: Pick,
    index: usize,
}

#[allow(dead_code)]
impl DeletePickCommand {
    pub fn new(horizon_id: Uuid, pick: Pick, index: usize) -> Self {
        Self {
            horizon_id,
            pick,
            index,
        }
    }
}

impl InterpretationCommand for DeletePickCommand {
    fn execute(&mut self, state: &mut InterpretationState) {
        if let Some(horizon) = state.horizons.iter_mut().find(|h| h.id == self.horizon_id) {
            horizon.picks.remove(self.index);
            horizon.update_mesh();
        }
    }

    fn undo(&mut self, state: &mut InterpretationState) {
        if let Some(horizon) = state.horizons.iter_mut().find(|h| h.id == self.horizon_id) {
            // Use horizon_id to satisfy compiler about field usage
            let _ = &self.horizon_id;
            horizon.picks.insert(self.index, self.pick.clone());
            horizon.update_mesh();
        }
    }
}

#[allow(dead_code)]
pub struct AutoTrackCommand {
    horizon_id: Uuid,
    picks: Vec<Pick>,
}

#[allow(dead_code)]
impl AutoTrackCommand {
    pub fn new(horizon_id: Uuid, picks: Vec<Pick>) -> Self {
        Self { horizon_id, picks }
    }
}

impl InterpretationCommand for AutoTrackCommand {
    fn execute(&mut self, state: &mut InterpretationState) {
        if let Some(horizon) = state.horizons.iter_mut().find(|h| h.id == self.horizon_id) {
            for pick in &self.picks {
                horizon.add_pick(pick.clone());
            }
            horizon.update_mesh();
        }
    }

    fn undo(&mut self, state: &mut InterpretationState) {
        if let Some(horizon) = state.horizons.iter_mut().find(|h| h.id == self.horizon_id) {
            let pick_ids: std::collections::HashSet<_> = self.picks.iter().map(|p| p.id).collect();
            horizon.picks.retain(|p| !pick_ids.contains(&p.id));
            horizon.update_mesh();
        }
    }
}

#[allow(dead_code)]
pub struct GenerateSurfaceCommand {
    horizon_id: Uuid,
    // Note: In a real app, we might store the previous surface state
}

#[allow(dead_code)]
impl GenerateSurfaceCommand {
    pub fn new(horizon_id: Uuid) -> Self {
        Self { horizon_id }
    }
}

impl InterpretationCommand for GenerateSurfaceCommand {
    fn execute(&mut self, state: &mut InterpretationState) {
        if let Some(horizon) = state.horizons.iter_mut().find(|h| h.id == self.horizon_id) {
            horizon.update_mesh();
        }
    }

    fn undo(&mut self, _state: &mut InterpretationState) {
        // Reverting a surface generation might just clear the mesh or do nothing if mesh is always derived from picks
    }
}

#[allow(dead_code)]
pub struct AddFaultStickCommand {
    fault_id: Uuid,
    stick: FaultStick,
}

#[allow(dead_code)]
impl AddFaultStickCommand {
    pub fn new(fault_id: Uuid, stick: FaultStick) -> Self {
        Self { fault_id, stick }
    }
}

impl InterpretationCommand for AddFaultStickCommand {
    fn execute(&mut self, state: &mut InterpretationState) {
        if let Some(fault) = state.faults.iter_mut().find(|f| f.id == self.fault_id) {
            fault.add_stick(self.stick.clone());
            fault.update_mesh();
        }
    }

    fn undo(&mut self, state: &mut InterpretationState) {
        if let Some(fault) = state.faults.iter_mut().find(|f| f.id == self.fault_id) {
            fault.sticks.retain(|s| s.id != self.stick.id);
            fault.update_mesh();
        }
    }
}

pub struct HistoryManager {
    undo_stack: Vec<Box<dyn InterpretationCommand>>,
    redo_stack: Vec<Box<dyn InterpretationCommand>>,
}

#[allow(dead_code)]
impl HistoryManager {
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    pub fn execute(
        &mut self,
        mut command: Box<dyn InterpretationCommand>,
        state: &mut InterpretationState,
    ) {
        command.execute(state);
        self.undo_stack.push(command);
        self.redo_stack.clear();
    }

    pub fn undo(&mut self, state: &mut InterpretationState) {
        if let Some(mut command) = self.undo_stack.pop() {
            command.undo(state);
            self.redo_stack.push(command);
        }
    }

    pub fn redo(&mut self, state: &mut InterpretationState) {
        if let Some(mut command) = self.redo_stack.pop() {
            command.execute(state);
            self.undo_stack.push(command);
        }
    }
}
