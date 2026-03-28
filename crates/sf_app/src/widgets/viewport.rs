use eframe::egui;
use eframe::egui_wgpu;
use crate::interpretation::{InterpretationState, Pick, PickSource, PickingMode};
use sf_compute::seismic::SeismicVolume;
use sf_compute::tracking::{snap_to_extrema, track_event};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    Map,
    Section,
}

pub struct ViewportWidget {
    pub target_format: Option<eframe::wgpu::TextureFormat>,
    pub sketch_points: Vec<[f32; 3]>,
    pub view_mode: ViewMode,
    pub section_xline: i32,
}

impl ViewportWidget {
    pub fn new() -> Self {
        Self {
            target_format: None,
            sketch_points: Vec::new(),
            view_mode: ViewMode::Map,
            section_xline: 250,
        }
    }

    pub fn ui(
        &mut self,
        ui: &mut egui::Ui,
        interpretation: &mut InterpretationState,
        volume: Option<&SeismicVolume>,
    ) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.view_mode, ViewMode::Map, "Map View");
            ui.selectable_value(&mut self.view_mode, ViewMode::Section, "Section View");
            if self.view_mode == ViewMode::Section {
                ui.add(egui::Slider::new(&mut self.section_xline, 0..=500).text("Xline Slice"));
            }
        });

        let (rect, response) = ui.allocate_at_least(ui.available_size(), egui::Sense::drag());
        
        let sample_count = volume.map(|v| v.provider.sample_count()).unwrap_or(500) as f32;

        if interpretation.picking_mode == PickingMode::SketchFault {
            if response.drag_started() {
                self.sketch_points.clear();
            }
            if response.dragged() {
                if let Some(pos) = response.interact_pointer_pos() {
                    let rel_x = (pos.x - rect.min.x) / rect.width();
                    let rel_y = (pos.y - rect.min.y) / rect.height();
                    
                    let (iline, xline, sample) = match self.view_mode {
                        ViewMode::Map => (rel_x * 500.0, rel_y * 500.0, 250.0),
                        ViewMode::Section => (rel_x * 500.0, self.section_xline as f32, rel_y * sample_count),
                    };
                    
                    // Simple distance check to avoid redundant points
                    if self.sketch_points.is_empty() || 
                       (pos.to_vec2() - self.project_to_screen([
                           self.sketch_points.last().unwrap()[0],
                           self.sketch_points.last().unwrap()[1],
                           self.sketch_points.last().unwrap()[2],
                       ], rect, sample_count).to_vec2()).length() > 5.0 
                    {
                        self.sketch_points.push([iline, xline, sample]);
                    }
                }
            }
            if response.drag_released() {
                if !self.sketch_points.is_empty() {
                    if let Some(fault) = interpretation.active_fault_mut() {
                        fault.add_stick(crate::interpretation::FaultStick::new(self.sketch_points.clone()));
                        fault.update_mesh();
                    }
                    self.sketch_points.clear();
                }
            }
        } else if response.clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                if interpretation.picking_mode != PickingMode::None {
                    self.handle_click(pos, rect, interpretation, volume);
                }
            }
        }

        if let Some(format) = self.target_format {
            let callback = egui_wgpu::Callback::new_paint_callback(
                rect,
                ViewportCallback { format },
            );
            ui.painter().add(callback);
        }
        
        // Add a visual fallback to confirm the widget area is correctly allocated
        ui.painter().rect_stroke(rect, 0.0, (1.0, egui::Color32::DARK_GRAY));

        // 2D Overlay Visualization (Fallback for stub 3D renderer)
        self.draw_overlays(ui, rect, interpretation, sample_count);
        self.draw_fault_overlays(ui, rect, interpretation, sample_count);
    }

    fn project_to_screen(&self, pos: [f32; 3], rect: egui::Rect, sample_count: f32) -> egui::Pos2 {
        match self.view_mode {
            ViewMode::Map => egui::pos2(
                rect.min.x + (pos[0] / 500.0) * rect.width(),
                rect.min.y + (pos[1] / 500.0) * rect.height(),
            ),
            ViewMode::Section => egui::pos2(
                rect.min.x + (pos[0] / 500.0) * rect.width(),
                rect.min.y + (pos[2] / sample_count) * rect.height(),
            ),
        }
    }

    fn is_visible_in_view(&self, pos: [f32; 3]) -> bool {
        match self.view_mode {
            ViewMode::Map => true,
            ViewMode::Section => (pos[1] - self.section_xline as f32).abs() < 10.0,
        }
    }

    fn draw_overlays(&self, ui: &mut egui::Ui, rect: egui::Rect, interpretation: &InterpretationState, sample_count: f32) {
        let painter = ui.painter().with_clip_rect(rect);

        for horizon in &interpretation.horizons {
            if !horizon.is_visible { continue; }
            
            let color = egui::Color32::from_rgb(
                (horizon.color[0] * 255.0) as u8,
                (horizon.color[1] * 255.0) as u8,
                (horizon.color[2] * 255.0) as u8,
            );

            // Draw Picks
            for pick in &horizon.picks {
                if self.is_visible_in_view(pick.position) {
                    let screen_pos = self.project_to_screen(pick.position, rect, sample_count);
                    painter.circle_filled(screen_pos, 3.0, color);
                }
            }

            // Draw Surface Mesh (as wireframe in 2D)
            if let Some(mesh) = &horizon.mesh {
                for chunk in mesh.indices.chunks(3) {
                    if chunk.len() == 3 {
                        let p1 = mesh.vertices[chunk[0] as usize];
                        let p2 = mesh.vertices[chunk[1] as usize];
                        let p3 = mesh.vertices[chunk[2] as usize];

                        if self.is_visible_in_view(p1) || self.is_visible_in_view(p2) || self.is_visible_in_view(p3) {
                            let pts = [p1, p2, p3].map(|p| self.project_to_screen(p, rect, sample_count));

                            painter.line_segment([pts[0], pts[1]], (0.5, color.linear_multiply(0.3)));
                            painter.line_segment([pts[1], pts[2]], (0.5, color.linear_multiply(0.3)));
                            painter.line_segment([pts[2], pts[0]], (0.5, color.linear_multiply(0.3)));
                        }
                    }
                }
            }
        }
    }

    fn draw_fault_overlays(&self, ui: &mut egui::Ui, rect: egui::Rect, interpretation: &InterpretationState, sample_count: f32) {
        let painter = ui.painter().with_clip_rect(rect);

        // Draw active sketch
        if !self.sketch_points.is_empty() {
            let color = egui::Color32::YELLOW;
            let pts: Vec<egui::Pos2> = self.sketch_points.iter()
                .map(|&p| self.project_to_screen(p, rect, sample_count))
                .collect();
            
            for i in 0..pts.len() - 1 {
                painter.line_segment([pts[i], pts[i+1]], (2.0, color));
            }
        }

        for fault in &interpretation.faults {
            if !fault.is_visible { continue; }
            
            let color = egui::Color32::from_rgb(
                (fault.color[0] * 255.0) as u8,
                (fault.color[1] * 255.0) as u8,
                (fault.color[2] * 255.0) as u8,
            );

            // Draw Sticks
            for stick in &fault.sticks {
                let pts: Vec<egui::Pos2> = stick.picks.iter()
                    .filter(|&&p| self.is_visible_in_view(p))
                    .map(|&p| self.project_to_screen(p, rect, sample_count))
                    .collect();
                
                if pts.len() > 1 {
                    for i in 0..pts.len() - 1 {
                        painter.line_segment([pts[i], pts[i+1]], (1.5, color));
                    }
                }
                for pt in &pts {
                    painter.circle_filled(*pt, 2.0, color);
                }
            }

            // Draw Fault Mesh (wireframe)
            if let Some(mesh) = &fault.mesh {
                for chunk in mesh.indices.chunks(3) {
                    if chunk.len() == 3 {
                        let p1 = mesh.vertices[chunk[0] as usize];
                        let p2 = mesh.vertices[chunk[1] as usize];
                        let p3 = mesh.vertices[chunk[2] as usize];

                        if self.is_visible_in_view(p1) || self.is_visible_in_view(p2) || self.is_visible_in_view(p3) {
                            let pts = [p1, p2, p3].map(|p| self.project_to_screen(p, rect, sample_count));

                            painter.line_segment([pts[0], pts[1]], (0.5, color.linear_multiply(0.5)));
                            painter.line_segment([pts[1], pts[2]], (0.5, color.linear_multiply(0.5)));
                            painter.line_segment([pts[2], pts[0]], (0.5, color.linear_multiply(0.5)));
                        }
                    }
                }
            }
        }
    }

    fn handle_click(
        &self,
        pos: egui::Pos2,
        rect: egui::Rect,
        interpretation: &mut InterpretationState,
        volume: Option<&SeismicVolume>,
    ) {
        let rel_x = (pos.x - rect.min.x) / rect.width();
        let rel_y = (pos.y - rect.min.y) / rect.height();

        let sample_count = volume.map(|v| v.provider.sample_count()).unwrap_or(500);

        let (iline, xline, mut sample) = match self.view_mode {
            ViewMode::Map => (
                (rel_x * 500.0) as i32,
                (rel_y * 500.0) as i32,
                250usize,
            ),
            ViewMode::Section => (
                (rel_x * 500.0) as i32,
                self.section_xline,
                (rel_y * sample_count as f32) as usize,
            ),
        };

        if let Some(vol) = volume {
            if let Some(trace) = vol.provider.get_trace(iline, xline) {
                // Snap to nearest extrema
                sample = snap_to_extrema(&trace, sample, 20, true);
                
                if interpretation.picking_mode == PickingMode::AutoTrack {
                    let results = track_event(vol, iline, xline, sample, true, 0.5);
                    if let Some(horizon) = interpretation.active_horizon_mut() {
                        for (il, xl, s) in results {
                            horizon.add_pick(Pick::new([il as f32, xl as f32, s as f32], PickSource::AutoTracked));
                        }
                        horizon.update_mesh();
                    }
                    return;
                }
            }
        }

        // Manual or Seed pick
        let picking_mode = interpretation.picking_mode;
        if let Some(horizon) = interpretation.active_horizon_mut() {
            let source = match picking_mode {
                PickingMode::Seed => PickSource::Seed,
                _ => PickSource::Manual,
            };
            horizon.add_pick(Pick::new([iline as f32, xline as f32, sample as f32], source));
            horizon.update_mesh();
        }
    }
}

struct ViewportCallback {
    format: eframe::wgpu::TextureFormat,
}

impl egui_wgpu::CallbackTrait for ViewportCallback {
    fn prepare(
        &self,
        device: &egui_wgpu::wgpu::Device,
        _queue: &egui_wgpu::wgpu::Queue,
        _screen_descriptor: &egui_wgpu::ScreenDescriptor,
        _egui_encoder: &mut egui_wgpu::wgpu::CommandEncoder,
        resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<egui_wgpu::wgpu::CommandBuffer> {
        if !resources.contains::<sf_render::Renderer>() {
            resources.insert(sf_render::Renderer::new(device, self.format));
        }
        if !resources.contains::<sf_render::Scene>() {
            resources.insert(sf_render::Scene::new());
        }
        Vec::new()
    }

    fn paint<'a>(
        &'a self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut egui_wgpu::wgpu::RenderPass<'a>,
        resources: &'a egui_wgpu::CallbackResources,
    ) {
        let renderer = resources.get::<sf_render::Renderer>();
        let scene = resources.get::<sf_render::Scene>();

        if let (Some(renderer), Some(scene)) = (renderer, scene) {
            let camera_pos = [0.0, 0.0, 5.0]; // Default camera
            renderer.render(render_pass, scene, camera_pos);
        }
    }
}
