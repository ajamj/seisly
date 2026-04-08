use std::sync::Arc;
use eframe::egui_wgpu;
use eframe::wgpu;
use crate::interpretation::{HistoryManager, InterpretationState, Pick, PickSource, PickingMode, VelocityState};
use crate::interpretation::history::{AddPickCommand, AutoTrackCommand, AddFaultStickCommand};

use seisly_compute::seismic::SeismicVolume;
use seisly_compute::tracking::{snap_to_extrema, track_event};

use seisly_render::{FaultRenderer, SeismicRenderer, SeismicUniforms, ColormapManager, ColormapPreset};

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
    pub gain: f32,
    pub clip: f32,
    pub colormap: ColormapPreset,
    pub tracking_progress: Option<f32>, // 0.0 to 1.0, None = not tracking
}

impl ViewportWidget {
    pub fn new() -> Self {
        Self {
            target_format: None,
            sketch_points: Vec::new(),
            view_mode: ViewMode::Map,
            section_xline: 250,
            gain: 1.0,
            clip: 1.0,
            colormap: ColormapPreset::Seismic,
            tracking_progress: None,
        }
    }

    pub fn ui(
        &mut self,
        ui: &mut egui::Ui,
        interpretation: &mut InterpretationState,
        history: &mut HistoryManager,
        velocity: &VelocityState,
        volume: Option<&Arc<SeismicVolume>>,
    ) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.view_mode, ViewMode::Map, "Map View");
            ui.selectable_value(&mut self.view_mode, ViewMode::Section, "Section View");
            if self.view_mode == ViewMode::Section {
                if ui.add(egui::Slider::new(&mut self.section_xline, 0..=500).text("Xline Slice")).changed() {
                    ui.ctx().request_repaint();
                }
            }
            
            ui.separator();
            ui.label("Gain:");
            if ui.add(egui::Slider::new(&mut self.gain, 0.1..=10.0).logarithmic(true).show_value(false)).changed() {
                ui.ctx().request_repaint();
            }
            ui.label("Clip:");
            if ui.add(egui::Slider::new(&mut self.clip, 0.01..=2.0).show_value(false)).changed() {
                ui.ctx().request_repaint();
            }

            // Auto-tracking progress indicator
            if let Some(progress) = self.tracking_progress {
                ui.separator();
                ui.label("🔄 Auto-Tracking:");
                ui.add(
                    egui::ProgressBar::new(progress)
                        .show_percentage()
                        .desired_width(150.0),
                );
                if progress < 1.0 {
                    if ui.button("⏹ Cancel").clicked() {
                        self.tracking_progress = None;
                    }
                } else {
                    self.tracking_progress = None; // Auto-clear when complete
                }
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
                        ViewMode::Section => {
                            (
                                rel_x * 500.0,
                                self.section_xline as f32,
                                rel_y * sample_count,
                            )
                        }
                    };

                    // Simple distance check to avoid redundant points
                    if self.sketch_points.is_empty()
                        || (pos.to_vec2()
                            - self
                                .project_to_screen(
                                    [
                                        self.sketch_points.last().unwrap()[0],
                                        self.sketch_points.last().unwrap()[1],
                                        self.sketch_points.last().unwrap()[2],
                                    ],
                                    rect,
                                    sample_count,
                                    velocity,
                                )
                                .to_vec2())
                        .length()
                            > 5.0
                    {
                        self.sketch_points.push([iline, xline, sample]);
                    }
                }
            }
            if response.drag_stopped() {
                if !self.sketch_points.is_empty() {
                    if let Some(fault) = interpretation.active_fault_mut() {
                        let fault_id = fault.id;
                        let stick = crate::interpretation::FaultStick::new(
                            self.sketch_points.clone(),
                        );
                        // Use command for undo/redo support
                        let command = Box::new(AddFaultStickCommand::new(fault_id, stick));
                        history.execute(command, interpretation);
                    }
                    self.sketch_points.clear();
                }
            }
        } else if response.clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                if interpretation.picking_mode != PickingMode::None {
                    self.handle_click(pos, rect, interpretation, history, velocity, volume.map(|v| v.as_ref()));
                }
            }
        }

        // Only draw seismic in Time mode
        if !velocity.is_depth_mode {
            if let Some(format) = self.target_format {
                let callback = egui_wgpu::Callback::new_paint_callback(
                    rect,
                    ViewportCallback {
                        format,
                        volume: volume.cloned(),
                        view_mode: self.view_mode,
                        slice_idx: self.section_xline,
                        gain: self.gain,
                        clip: self.clip,
                        colormap: self.colormap,
                    },
                );
                ui.painter().add(callback);
            }
        } else {
            // Draw a background for depth mode
            ui.painter()
                .rect_filled(rect, 0.0, egui::Color32::from_black_alpha(20));
            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                "Depth Mode: Seismic Hidden",
                egui::FontId::proportional(20.0),
                egui::Color32::GRAY,
            );
        }

        // Add a visual fallback to confirm the widget area is correctly allocated
        ui.painter()
            .rect_stroke(rect, 0.0, (1.0, egui::Color32::DARK_GRAY));

        // 2D Overlay Visualization (Fallback for stub 3D renderer)
        self.draw_overlays(ui, rect, interpretation, sample_count, velocity);
        self.draw_fault_overlays(ui, rect, interpretation, sample_count, velocity);
    }

    fn project_to_screen(
        &self,
        pos: [f32; 3],
        rect: egui::Rect,
        sample_count: f32,
        velocity: &VelocityState,
    ) -> egui::Pos2 {
        let pos_p = velocity.project_to_depth(pos);
        match self.view_mode {
            ViewMode::Map => egui::pos2(
                rect.min.x + (pos_p[0] / 500.0) * rect.width(),
                rect.min.y + (pos_p[1] / 500.0) * rect.height(),
            ),
            ViewMode::Section => {
                let z_scale = if velocity.is_depth_mode {
                    velocity.model.sample_to_depth(sample_count)
                } else {
                    sample_count
                };
                egui::pos2(
                    rect.min.x + (pos_p[0] / 500.0) * rect.width(),
                    rect.min.y + (pos_p[2] / z_scale) * rect.height(),
                )
            }
        }
    }

    fn is_visible_in_view(&self, pos: [f32; 3]) -> bool {
        match self.view_mode {
            ViewMode::Map => true,
            ViewMode::Section => (pos[1] - self.section_xline as f32).abs() < 10.0,
        }
    }

    fn draw_overlays(
        &self,
        ui: &mut egui::Ui,
        rect: egui::Rect,
        interpretation: &InterpretationState,
        sample_count: f32,
        velocity: &VelocityState,
    ) {
        let painter = ui.painter().with_clip_rect(rect);

        for horizon in &interpretation.horizons {
            if !horizon.is_visible {
                continue;
            }

            let color = egui::Color32::from_rgba_unmultiplied(
                (horizon.color[0] * 255.0) as u8,
                (horizon.color[1] * 255.0) as u8,
                (horizon.color[2] * 255.0) as u8,
                (horizon.color[3] * 255.0) as u8,
            );

            let is_active = interpretation.active_horizon_id == Some(horizon.id);

            // Draw Picks
            for pick in &horizon.picks {
                if self.is_visible_in_view(pick.position) {
                    let screen_pos =
                        self.project_to_screen(pick.position, rect, sample_count, velocity);
                    // Larger circle for active horizon picks
                    let radius = if is_active { 5.0 } else { 3.0 };
                    painter.circle_filled(screen_pos, radius, color);
                    // Add white outline for better visibility
                    painter.circle_stroke(screen_pos, radius + 1.0, (1.0, egui::Color32::WHITE));
                }
            }

            // Draw Surface Meshes (as wireframe in 2D)
            for mesh in &horizon.meshes {
                for chunk in mesh.indices.chunks(3) {
                    if chunk.len() == 3 {
                        let p1 = mesh.vertices[chunk[0] as usize];
                        let p2 = mesh.vertices[chunk[1] as usize];
                        let p3 = mesh.vertices[chunk[2] as usize];

                        if self.is_visible_in_view(p1)
                            || self.is_visible_in_view(p2)
                            || self.is_visible_in_view(p3)
                        {
                            let pts = [p1, p2, p3]
                                .map(|p| self.project_to_screen(p, rect, sample_count, velocity));

                            // Thicker lines for active horizon
                            let line_width = if is_active { 1.5 } else { 0.5 };
                            let line_color = if is_active {
                                color
                            } else {
                                color.linear_multiply(0.3)
                            };

                            painter.line_segment([pts[0], pts[1]], (line_width, line_color));
                            painter.line_segment([pts[1], pts[2]], (line_width, line_color));
                            painter.line_segment([pts[2], pts[0]], (line_width, line_color));
                        }
                    }
                }
            }

            // Draw Intersection Lines
            for line in &horizon.intersection_lines {
                let pts: Vec<egui::Pos2> = line
                    .iter()
                    .map(|&p| self.project_to_screen(p, rect, sample_count, velocity))
                    .collect();
                for i in 0..pts.len().saturating_sub(1) {
                    painter.line_segment([pts[i], pts[i + 1]], (2.0, egui::Color32::WHITE));
                }
            }
        }
    }

    fn draw_fault_overlays(
        &self,
        ui: &mut egui::Ui,
        rect: egui::Rect,
        interpretation: &InterpretationState,
        sample_count: f32,
        velocity: &VelocityState,
    ) {
        let painter = ui.painter().with_clip_rect(rect);

        // Draw active sketch
        if !self.sketch_points.is_empty() {
            let color = egui::Color32::YELLOW;
            let pts: Vec<egui::Pos2> = self
                .sketch_points
                .iter()
                .map(|&p| self.project_to_screen(p, rect, sample_count, velocity))
                .collect();

            for i in 0..pts.len() - 1 {
                painter.line_segment([pts[i], pts[i + 1]], (2.0, color));
            }
            // Draw start and end points
            if let Some(first) = pts.first() {
                painter.circle_filled(*first, 4.0, egui::Color32::GREEN);
            }
            if let Some(last) = pts.last() {
                painter.circle_filled(*last, 4.0, egui::Color32::RED);
            }
        }

        for fault in &interpretation.faults {
            if !fault.is_visible {
                continue;
            }

            let color = egui::Color32::from_rgba_unmultiplied(
                (fault.color[0] * 255.0) as u8,
                (fault.color[1] * 255.0) as u8,
                (fault.color[2] * 255.0) as u8,
                (fault.color[3] * 255.0) as u8,
            );

            let is_active = interpretation.active_fault_id == Some(fault.id);

            // Draw Sticks
            for stick in &fault.sticks {
                let pts: Vec<egui::Pos2> = stick
                    .picks
                    .iter()
                    .filter(|&&p| self.is_visible_in_view(p))
                    .map(|&p| self.project_to_screen(p, rect, sample_count, velocity))
                    .collect();

                if pts.len() > 1 {
                    let line_width = if is_active { 2.5 } else { 1.5 };
                    for i in 0..pts.len() - 1 {
                        painter.line_segment([pts[i], pts[i + 1]], (line_width, color));
                    }
                }
                for pt in &pts {
                    let radius = if is_active { 4.0 } else { 2.0 };
                    painter.circle_filled(*pt, radius, color);
                    if is_active {
                        painter.circle_stroke(*pt, radius + 1.0, (1.0, egui::Color32::WHITE));
                    }
                }
            }

            // Draw Fault Meshes (wireframe)
            for mesh in &fault.meshes {
                for chunk in mesh.indices.chunks(3) {
                    if chunk.len() == 3 {
                        let p1 = mesh.vertices[chunk[0] as usize];
                        let p2 = mesh.vertices[chunk[1] as usize];
                        let p3 = mesh.vertices[chunk[2] as usize];

                        if self.is_visible_in_view(p1)
                            || self.is_visible_in_view(p2)
                            || self.is_visible_in_view(p3)
                        {
                            let pts = [p1, p2, p3]
                                .map(|p| self.project_to_screen(p, rect, sample_count, velocity));

                            painter
                                .line_segment([pts[0], pts[1]], (0.5, color.linear_multiply(0.5)));
                            painter
                                .line_segment([pts[1], pts[2]], (0.5, color.linear_multiply(0.5)));
                            painter
                                .line_segment([pts[2], pts[0]], (0.5, color.linear_multiply(0.5)));
                        }
                    }
                }
            }

            // Draw Intersection Lines
            for line in &fault.intersection_lines {
                let pts: Vec<egui::Pos2> = line
                    .iter()
                    .map(|&p| self.project_to_screen(p, rect, sample_count, velocity))
                    .collect();
                for i in 0..pts.len().saturating_sub(1) {
                    painter.line_segment([pts[i], pts[i + 1]], (2.0, egui::Color32::WHITE));
                }
            }
        }
    }

    fn handle_click(
        &self,
        pos: egui::Pos2,
        rect: egui::Rect,
        interpretation: &mut InterpretationState,
        history: &mut HistoryManager,
        velocity: &VelocityState,
        volume: Option<&SeismicVolume>,
    ) {
        let rel_x = (pos.x - rect.min.x) / rect.width();
        let rel_y = (pos.y - rect.min.y) / rect.height();

        let sample_count = volume.map(|v| v.provider.sample_count()).unwrap_or(500);

        let (iline, xline, mut sample) = match self.view_mode {
            ViewMode::Map => ((rel_x * 500.0) as i32, (rel_y * 500.0) as i32, 250usize),
            ViewMode::Section => {
                let s = if velocity.is_depth_mode {
                    (rel_y * sample_count as f32) as usize
                } else {
                    (rel_y * sample_count as f32) as usize
                };
                ((rel_x * 500.0) as i32, self.section_xline, s)
            }
        };

        if let Some(vol) = volume {
            if let Some(trace) = vol.provider.get_trace(iline, xline) {
                // Snap to nearest extrema
                sample = snap_to_extrema(&trace, sample, 20, true);

                if interpretation.picking_mode == PickingMode::AutoTrack {
                    let results = track_event(vol, iline, xline, sample, true, 0.5);
                    if let Some(horizon) = interpretation.active_horizon() {
                        let horizon_id = horizon.id;
                        let picks: Vec<Pick> = results
                            .iter()
                            .map(|(il, xl, s)| {
                                Pick::new([*il as f32, *xl as f32, *s as f32], PickSource::AutoTracked)
                            })
                            .collect();
                        // Use command for undo/redo support
                        let command = Box::new(AutoTrackCommand::new(horizon_id, picks));
                        history.execute(command, interpretation);
                    }
                    return;
                }
            }
        }

        // Manual or Seed pick
        let picking_mode = interpretation.picking_mode;
        if let Some(horizon) = interpretation.active_horizon() {
            let horizon_id = horizon.id;
            let source = match picking_mode {
                PickingMode::Seed => PickSource::Seed,
                _ => PickSource::Manual,
            };
            let pick = Pick::new([iline as f32, xline as f32, sample as f32], source);
            // Use command for undo/redo support
            let command = Box::new(AddPickCommand::new(horizon_id, pick));
            history.execute(command, interpretation);
        }
    }
}

struct SeismicResources {
    texture: wgpu::Texture,
    texture_view: wgpu::TextureView,
    sampler: wgpu::Sampler,
    uniform_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    last_volume_id: Option<String>,
    last_slice_idx: i32,
    last_view_mode: ViewMode,
}

struct ViewportCallback {
    format: eframe::wgpu::TextureFormat,
    volume: Option<Arc<SeismicVolume>>,
    view_mode: ViewMode,
    slice_idx: i32,
    gain: f32,
    clip: f32,
    colormap: ColormapPreset,
}

impl egui_wgpu::CallbackTrait for ViewportCallback {
    fn prepare(
        &self,
        device: &egui_wgpu::wgpu::Device,
        queue: &egui_wgpu::wgpu::Queue,
        _screen_descriptor: &egui_wgpu::ScreenDescriptor,
        _egui_encoder: &mut egui_wgpu::wgpu::CommandEncoder,
        resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<egui_wgpu::wgpu::CommandBuffer> {
        if !resources.contains::<SeismicRenderer>() {
            resources.insert(SeismicRenderer::new(device, self.format));
        }
        if !resources.contains::<ColormapManager>() {
            resources.insert(ColormapManager::new(device, queue));
        }
        if !resources.contains::<seisly_render::Renderer>() {
            resources.insert(seisly_render::Renderer::new(device, self.format));
        }
        if !resources.contains::<seisly_render::Scene>() {
            resources.insert(seisly_render::Scene::new());
        }
        if !resources.contains::<FaultRenderer>() {
            resources.insert(FaultRenderer::new(device, self.format));
        }

        if let Some(volume) = &self.volume {
            let renderer = resources.get::<SeismicRenderer>().unwrap();
            let colormap_manager = resources.get::<ColormapManager>().unwrap();

            // Check if we need to update resources
            let mut needs_update = false;
            if !resources.contains::<SeismicResources>() {
                needs_update = true;
            } else {
                let sr = resources.get::<SeismicResources>().unwrap();
                if sr.last_slice_idx != self.slice_idx || sr.last_view_mode != self.view_mode {
                    needs_update = true;
                }
            }

            if needs_update {
                // Extract slice data
                let slice_data = match self.view_mode {
                    ViewMode::Map => vec![0.0; 501 * 501], // TODO: Implement time slice
                    ViewMode::Section => volume.get_crossline(self.slice_idx as usize),
                };

                let width = (volume.provider.inline_range().1 - volume.provider.inline_range().0 + 1) as u32;
                let height = volume.provider.sample_count() as u32;

                if slice_data.len() as u32 == width * height {
                    let texture = device.create_texture(&wgpu::TextureDescriptor {
                        label: Some("Seismic Slice Texture"),
                        size: wgpu::Extent3d {
                            width,
                            height,
                            depth_or_array_layers: 1,
                        },
                        mip_level_count: 1,
                        sample_count: 1,
                        dimension: wgpu::TextureDimension::D2,
                        format: wgpu::TextureFormat::R32Float,
                        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                        view_formats: &[],
                    });

                    queue.write_texture(
                        wgpu::ImageCopyTexture {
                            texture: &texture,
                            mip_level: 0,
                            origin: wgpu::Origin3d::ZERO,
                            aspect: wgpu::TextureAspect::All,
                        },
                        bytemuck::cast_slice(&slice_data),
                        wgpu::ImageDataLayout {
                            offset: 0,
                            bytes_per_row: Some(width * 4),
                            rows_per_image: None,
                        },
                        wgpu::Extent3d {
                            width,
                            height,
                            depth_or_array_layers: 1,
                        },
                    );

                    let texture_view = texture.create_view(&Default::default());
                    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
                        label: Some("Seismic Sampler"),
                        address_mode_u: wgpu::AddressMode::ClampToEdge,
                        address_mode_v: wgpu::AddressMode::ClampToEdge,
                        mag_filter: wgpu::FilterMode::Linear,
                        min_filter: wgpu::FilterMode::Linear,
                        ..Default::default()
                    });

                    let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                        label: Some("Seismic Uniforms"),
                        size: std::mem::size_of::<SeismicUniforms>() as u64,
                        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                        mapped_at_creation: false,
                    });

                    let colormap_view = colormap_manager.get_view(self.colormap).unwrap();
                    let colormap_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
                        label: Some("Colormap Sampler"),
                        mag_filter: wgpu::FilterMode::Linear,
                        min_filter: wgpu::FilterMode::Linear,
                        ..Default::default()
                    });

                    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                        label: Some("Seismic Bind Group"),
                        layout: &renderer.bind_group_layout,
                        entries: &[
                            wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&texture_view) },
                            wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&sampler) },
                            wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::TextureView(colormap_view) },
                            wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::Sampler(&colormap_sampler) },
                            wgpu::BindGroupEntry { binding: 4, resource: uniform_buffer.as_entire_binding() },
                        ],
                    });

                    resources.insert(SeismicResources {
                        texture,
                        texture_view,
                        sampler,
                        uniform_buffer,
                        bind_group,
                        last_volume_id: None,
                        last_slice_idx: self.slice_idx,
                        last_view_mode: self.view_mode,
                    });
                }
            }

            // Always update uniforms
            if let Some(sr) = resources.get::<SeismicResources>() {
                let uniforms = SeismicUniforms {
                    gain: self.gain,
                    clip: self.clip,
                    colormap_index: 0,
                    _padding: 0,
                };
                queue.write_buffer(&sr.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
            }
        }

        Vec::new()
    }

    fn paint(
        &self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut eframe::wgpu::RenderPass<'static>,
        resources: &eframe::egui_wgpu::CallbackResources,
    ) {
        if let (Some(renderer), Some(sr)) = (resources.get::<SeismicRenderer>(), resources.get::<SeismicResources>()) {
            render_pass.set_pipeline(&renderer.pipeline);
            render_pass.set_bind_group(0, &sr.bind_group, &[]);
            render_pass.draw(0..4, 0..1);
        }

        let renderer = resources.get::<seisly_render::Renderer>();
        let scene = resources.get::<seisly_render::Scene>();

        if let (Some(renderer), Some(scene)) = (renderer, scene) {
            let camera_pos = [0.0, 0.0, 5.0];
            renderer.render(render_pass, scene, camera_pos);
        }
    }
}

