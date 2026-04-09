use crate::interpretation::{FaultStick, InterpretationState, Pick};
use seisly_core::Command;
use std::any::Any;
use uuid::Uuid;

pub struct HistoryManager {
    pub inner: seisly_core::UndoRedoStack,
}

impl HistoryManager {
    pub fn new(max_size: usize) -> Self {
        Self {
            inner: seisly_core::UndoRedoStack::new(max_size),
        }
    }

    pub fn execute(&mut self, command: Box<dyn Command>, target: &mut InterpretationState) {
        self.inner.execute(command, target);
    }

    pub fn undo(&mut self, target: &mut InterpretationState) {
        self.inner.undo(target);
    }

    pub fn redo(&mut self, target: &mut InterpretationState) {
        self.inner.redo(target);
    }
}

// Commands

pub struct AddPickCommand {
    pub horizon_id: Uuid,
    pub pick: Pick,
}

impl AddPickCommand {
    pub fn new(horizon_id: Uuid, pick: Pick) -> Self {
        Self { horizon_id, pick }
    }
}

impl Command for AddPickCommand {
    fn name(&self) -> &'static str {
        "Add Pick"
    }

    fn execute(&mut self, target: &mut dyn Any) {
        if let Some(state) = target.downcast_mut::<InterpretationState>() {
            if let Some(horizon) = state.horizons.iter_mut().find(|h| h.id == self.horizon_id) {
                horizon.add_pick(self.pick.clone());
            }
        }
    }

    fn undo(&mut self, target: &mut dyn Any) {
        if let Some(state) = target.downcast_mut::<InterpretationState>() {
            if let Some(horizon) = state.horizons.iter_mut().find(|h| h.id == self.horizon_id) {
                horizon.picks.retain(|p| p.id != self.pick.id);
            }
        }
    }
}

pub struct AddFaultStickCommand {
    pub fault_id: Uuid,
    pub stick: FaultStick,
}

impl AddFaultStickCommand {
    pub fn new(fault_id: Uuid, stick: FaultStick) -> Self {
        Self { fault_id, stick }
    }
}

impl Command for AddFaultStickCommand {
    fn name(&self) -> &'static str {
        "Add Fault Stick"
    }

    fn execute(&mut self, target: &mut dyn Any) {
        if let Some(state) = target.downcast_mut::<InterpretationState>() {
            if let Some(fault) = state.faults.iter_mut().find(|f| f.id == self.fault_id) {
                fault.add_stick(self.stick.clone());
            }
        }
    }

    fn undo(&mut self, target: &mut dyn Any) {
        if let Some(state) = target.downcast_mut::<InterpretationState>() {
            if let Some(fault) = state.faults.iter_mut().find(|f| f.id == self.fault_id) {
                fault.sticks.retain(|s| s.id != self.stick.id);
            }
        }
    }
}

pub struct AutoTrackCommand {
    pub horizon_id: Uuid,
    pub new_picks: Vec<Pick>,
    pub old_picks: Vec<Pick>,
}

impl AutoTrackCommand {
    pub fn new(horizon_id: Uuid, new_picks: Vec<Pick>) -> Self {
        Self {
            horizon_id,
            new_picks,
            old_picks: Vec::new(),
        }
    }
}

impl Command for AutoTrackCommand {
    fn name(&self) -> &'static str {
        "Auto Track"
    }

    fn execute(&mut self, target: &mut dyn Any) {
        if let Some(state) = target.downcast_mut::<InterpretationState>() {
            if let Some(horizon) = state.horizons.iter_mut().find(|h| h.id == self.horizon_id) {
                self.old_picks = horizon.picks.clone();
                horizon.picks.extend(self.new_picks.clone());
            }
        }
    }

    fn undo(&mut self, target: &mut dyn Any) {
        if let Some(state) = target.downcast_mut::<InterpretationState>() {
            if let Some(horizon) = state.horizons.iter_mut().find(|h| h.id == self.horizon_id) {
                horizon.picks = self.old_picks.clone();
            }
        }
    }
}
