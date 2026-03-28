//! 3D scene management

use crate::{MeshRenderer, LineRenderer};

/// 3D scene containing renderable objects
pub struct Scene {
    pub meshes: Vec<MeshRenderer>,
    pub lines: Vec<LineRenderer>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            meshes: vec![],
            lines: vec![],
        }
    }
    
    pub fn add_mesh(&mut self, renderer: MeshRenderer) {
        self.meshes.push(renderer);
    }
    
    pub fn add_lines(&mut self, renderer: LineRenderer) {
        self.lines.push(renderer);
    }
    
    pub fn mesh_count(&self) -> usize {
        self.meshes.len()
    }
    
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}
