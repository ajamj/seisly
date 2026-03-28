use uuid::Uuid;
use super::{InterpretationState, Pick};

pub trait InterpretationCommand: std::fmt::Debug {
    fn execute(&mut self, state: &mut InterpretationState);
    fn undo(&mut self, state: &mut InterpretationState);
}

#[derive(Debug)]
pub struct AddPick {
    pub horizon_id: Uuid,
    pub pick: Pick,
}

impl InterpretationCommand for AddPick {
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

#[derive(Debug)]
pub struct DeletePick {
    pub horizon_id: Uuid,
    pub pick_id: Uuid,
    pub deleted_pick: Option<Pick>,
}

impl InterpretationCommand for DeletePick {
    fn execute(&mut self, state: &mut InterpretationState) {
        if let Some(horizon) = state.horizons.iter_mut().find(|h| h.id == self.horizon_id) {
            if let Some(index) = horizon.picks.iter().position(|p| p.id == self.pick_id) {
                self.deleted_pick = Some(horizon.picks.remove(index));
                horizon.update_mesh();
            }
        }
    }

    fn undo(&mut self, state: &mut InterpretationState) {
        if let Some(horizon) = state.horizons.iter_mut().find(|h| h.id == self.horizon_id) {
            if let Some(pick) = self.deleted_pick.take() {
                horizon.add_pick(pick);
                horizon.update_mesh();
            }
        }
    }
}

#[derive(Debug)]
pub struct AutoTrack {
    pub horizon_id: Uuid,
    pub new_picks: Vec<Pick>,
}

impl InterpretationCommand for AutoTrack {
    fn execute(&mut self, state: &mut InterpretationState) {
        if let Some(horizon) = state.horizons.iter_mut().find(|h| h.id == self.horizon_id) {
            for pick in &self.new_picks {
                horizon.add_pick(pick.clone());
            }
            horizon.update_mesh();
        }
    }

    fn undo(&mut self, state: &mut InterpretationState) {
        if let Some(horizon) = state.horizons.iter_mut().find(|h| h.id == self.horizon_id) {
            let ids: std::collections::HashSet<Uuid> = self.new_picks.iter().map(|p| p.id).collect();
            horizon.picks.retain(|p| !ids.contains(&p.id));
            horizon.update_mesh();
        }
    }
}

#[derive(Debug)]
pub struct GenerateSurface {
    pub horizon_id: Uuid,
}

impl InterpretationCommand for GenerateSurface {
    fn execute(&mut self, state: &mut InterpretationState) {
        if let Some(horizon) = state.horizons.iter_mut().find(|h| h.id == self.horizon_id) {
            horizon.update_mesh();
        }
    }

    fn undo(&mut self, _state: &mut InterpretationState) {
        // RBF is deterministic, so undoing just leaves the mesh as it was.
        // In a real system we might want to store the previous mesh state.
    }
}

pub struct HistoryManager {
    undo_stack: Vec<Box<dyn InterpretationCommand>>,
    redo_stack: Vec<Box<dyn InterpretationCommand>>,
}

impl HistoryManager {
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    pub fn execute(&mut self, mut command: Box<dyn InterpretationCommand>, state: &mut InterpretationState) {
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

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }
}
