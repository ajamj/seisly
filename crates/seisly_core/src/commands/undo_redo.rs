//! Command Pattern for Undo/Redo
//!
//! This module provides a centralized command pattern for reversible operations
//! in the Seisly application. All interpretation operations that modify state
//! should be implemented as commands.

use std::any::Any;

/// Core trait for all undoable commands
pub trait Command: Any {
    /// Execute the command, modifying the target state
    fn execute(&mut self, target: &mut dyn Any);

    /// Undo the command, reverting the target to its previous state
    fn undo(&mut self, target: &mut dyn Any);

    /// Get a human-readable name for this command
    fn name(&self) -> &'static str;
}

/// Stack-based undo/redo manager
pub struct UndoRedoStack {
    undo_stack: Vec<Box<dyn Command>>,
    redo_stack: Vec<Box<dyn Command>>,
    max_stack_size: usize,
}

impl Default for UndoRedoStack {
    fn default() -> Self {
        Self::new(100)
    }
}

impl UndoRedoStack {
    /// Create a new undo/redo stack with a maximum size
    pub fn new(max_size: usize) -> Self {
        Self {
            undo_stack: Vec::with_capacity(max_size.min(100)),
            redo_stack: Vec::new(),
            max_stack_size: max_size,
        }
    }

    /// Execute a command and push it onto the undo stack
    pub fn execute(&mut self, mut command: Box<dyn Command>, target: &mut dyn Any) {
        command.execute(target);
        self.undo_stack.push(command);
        self.redo_stack.clear();

        // Trim stack if it exceeds max size
        if self.undo_stack.len() > self.max_stack_size {
            self.undo_stack.remove(0);
        }
    }

    /// Undo the last executed command
    pub fn undo(&mut self, target: &mut dyn Any) -> bool {
        if let Some(mut command) = self.undo_stack.pop() {
            command.undo(target);
            self.redo_stack.push(command);
            true
        } else {
            false
        }
    }

    /// Redo a previously undone command
    pub fn redo(&mut self, target: &mut dyn Any) -> bool {
        if let Some(mut command) = self.redo_stack.pop() {
            command.execute(target);
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
    pub fn undo_count(&self) -> usize {
        self.undo_stack.len()
    }

    /// Get the number of redoable operations
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

    // Simple test command for demonstration
    struct TestCommand {
        value: i32,
        previous_value: Option<i32>,
    }

    impl TestCommand {
        fn new(value: i32) -> Self {
            Self {
                value,
                previous_value: None,
            }
        }
    }

    impl Command for TestCommand {
        fn execute(&mut self, target: &mut dyn Any) {
            if let Some(val) = target.downcast_mut::<i32>() {
                self.previous_value = Some(*val);
                *val = self.value;
            }
        }

        fn undo(&mut self, target: &mut dyn Any) {
            if let Some(val) = target.downcast_mut::<i32>() {
                if let Some(prev) = self.previous_value {
                    *val = prev;
                }
            }
        }

        fn name(&self) -> &'static str {
            "TestCommand"
        }
    }

    #[test]
    fn test_undo_redo_stack_basic() {
        let mut stack = UndoRedoStack::new(10);
        let mut value: i32 = 0;

        // Execute first command
        stack.execute(Box::new(TestCommand::new(10)), &mut value);
        assert_eq!(value, 10);
        assert!(stack.can_undo());
        assert!(!stack.can_redo());

        // Execute second command
        stack.execute(Box::new(TestCommand::new(20)), &mut value);
        assert_eq!(value, 20);
        assert_eq!(stack.undo_count(), 2);

        // Undo second command
        let result = stack.undo(&mut value);
        assert!(result);
        assert_eq!(value, 10);
        assert!(stack.can_undo());
        assert!(stack.can_redo());
        assert_eq!(stack.redo_count(), 1);

        // Redo second command
        let result = stack.redo(&mut value);
        assert!(result);
        assert_eq!(value, 20);
        assert!(stack.can_undo());
        assert!(!stack.can_redo());
    }

    #[test]
    fn test_undo_clears_redo() {
        let mut stack = UndoRedoStack::new(10);
        let mut value: i32 = 0;

        stack.execute(Box::new(TestCommand::new(10)), &mut value);
        stack.execute(Box::new(TestCommand::new(20)), &mut value);

        // Undo once
        stack.undo(&mut value);
        assert!(stack.can_redo());

        // Execute new command - should clear redo stack
        stack.execute(Box::new(TestCommand::new(30)), &mut value);
        assert!(!stack.can_redo());
        assert_eq!(value, 30);
    }

    #[test]
    fn test_stack_size_limit() {
        let mut stack = UndoRedoStack::new(5);
        let mut value: i32 = 0;

        // Execute 10 commands
        for i in 0..10 {
            stack.execute(Box::new(TestCommand::new(i)), &mut value);
        }

        // Should only have 5 commands in undo stack
        assert_eq!(stack.undo_count(), 5);
    }

    #[test]
    fn test_empty_stack_operations() {
        let mut stack = UndoRedoStack::new(10);
        let mut value: i32 = 0;

        // Undo on empty stack should return false
        assert!(!stack.undo(&mut value));
        assert!(!stack.redo(&mut value));
        assert!(!stack.can_undo());
        assert!(!stack.can_redo());
    }

    #[test]
    fn test_clear() {
        let mut stack = UndoRedoStack::new(10);
        let mut value: i32 = 0;

        stack.execute(Box::new(TestCommand::new(10)), &mut value);
        stack.execute(Box::new(TestCommand::new(20)), &mut value);

        stack.clear();

        assert!(!stack.can_undo());
        assert!(!stack.can_redo());
        assert_eq!(stack.undo_count(), 0);
        assert_eq!(stack.redo_count(), 0);
    }
}
