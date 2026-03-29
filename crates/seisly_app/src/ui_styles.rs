//! StrataForge UI Styles and Constants
//!
//! Modern design system with consistent colors, spacing, and typography.

use eframe::egui;
use egui::{Color32, FontFamily, FontId, Margin, Rounding, Stroke, Visuals};

/// Primary color scheme
pub mod colors {
    use super::*;

    // Dark theme
    pub const DARK_BACKGROUND: Color32 = Color32::from_rgb(30, 30, 30);
    pub const DARK_PANEL: Color32 = Color32::from_rgb(37, 37, 37);
    pub const DARK_HOVER: Color32 = Color32::from_rgb(62, 62, 66);
    pub const DARK_SELECTED: Color32 = Color32::from_rgb(9, 71, 113);

    // Light theme
    pub const LIGHT_BACKGROUND: Color32 = Color32::from_rgb(248, 248, 248);
    pub const LIGHT_PANEL: Color32 = Color32::from_rgb(255, 255, 255);
    pub const LIGHT_HOVER: Color32 = Color32::from_rgb(230, 230, 230);
    pub const LIGHT_SELECTED: Color32 = Color32::from_rgb(200, 220, 240);

    // Accent colors (same for both themes)
    pub const SEISMIC: Color32 = Color32::from_rgb(79, 195, 247);
    pub const HORIZON: Color32 = Color32::from_rgb(129, 199, 132);
    pub const FAULT: Color32 = Color32::from_rgb(229, 115, 115);
    pub const WELL: Color32 = Color32::from_rgb(255, 183, 77);
    pub const ACTIVE: Color32 = Color32::from_rgb(100, 181, 246);

    // Text colors
    pub const TEXT_DARK_PRIMARY: Color32 = Color32::WHITE;
    pub const TEXT_DARK_SECONDARY: Color32 = Color32::from_rgb(176, 176, 176);
    pub const TEXT_LIGHT_PRIMARY: Color32 = Color32::from_rgb(30, 30, 30);
    pub const TEXT_LIGHT_SECONDARY: Color32 = Color32::from_rgb(100, 100, 100);

    // Border
    pub const BORDER_DARK: Color32 = Color32::from_rgb(62, 62, 66);
    pub const BORDER_LIGHT: Color32 = Color32::from_rgb(200, 200, 200);
}

/// UI Theme
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiTheme {
    Dark,
    Light,
}

impl Default for UiTheme {
    fn default() -> Self {
        Self::Dark
    }
}

/// Spacing constants (8px grid system)
pub mod spacing {
    pub const PANEL_PADDING: f32 = 8.0;
    pub const SECTION_SPACING: f32 = 16.0;
    pub const ITEM_SPACING: f32 = 4.0;
    pub const BUTTON_PADDING: f32 = 8.0;

    // Panel sizes
    pub const LEFT_PANEL_WIDTH: f32 = 260.0;
    pub const RIGHT_PANEL_WIDTH: f32 = 320.0;
    pub const BOTTOM_PANEL_HEIGHT: f32 = 200.0;
    pub const TOP_RIBBON_HEIGHT: f32 = 64.0;
    pub const STATUS_BAR_HEIGHT: f32 = 28.0;
}

/// Typography with modern font sizes
pub mod typography {
    use super::*;

    pub fn heading() -> FontId {
        FontId::new(13.0, FontFamily::Proportional)
    }

    pub fn body() -> FontId {
        FontId::new(11.0, FontFamily::Proportional)
    }

    pub fn label() -> FontId {
        FontId::new(10.0, FontFamily::Proportional)
    }

    pub fn status() -> FontId {
        FontId::new(10.0, FontFamily::Monospace)
    }

    pub fn button() -> FontId {
        FontId::new(11.0, FontFamily::Proportional)
    }
}

/// Apply theme to egui context
pub fn apply_theme(ctx: &egui::Context, theme: UiTheme) {
    let mut style = (*ctx.style()).clone();
    let visuals = match theme {
        UiTheme::Dark => {
            let mut v = Visuals::dark();
            v.panel_fill = colors::DARK_PANEL;
            v.window_fill = colors::DARK_PANEL;
            v.selection.bg_fill = colors::DARK_SELECTED;
            v.selection.stroke = Stroke::new(1.0, colors::ACTIVE);
            v.widgets.noninteractive.bg_fill = colors::DARK_BACKGROUND;
            v.widgets.hovered.bg_fill = colors::DARK_HOVER;
            v.widgets.active.bg_fill = colors::DARK_SELECTED;
            v.override_text_color = Some(colors::TEXT_DARK_PRIMARY);
            v
        }
        UiTheme::Light => {
            let mut v = Visuals::light();
            v.panel_fill = colors::LIGHT_PANEL;
            v.window_fill = colors::LIGHT_PANEL;
            v.selection.bg_fill = colors::LIGHT_SELECTED;
            v.selection.stroke = Stroke::new(1.0, colors::ACTIVE);
            v.widgets.noninteractive.bg_fill = colors::LIGHT_BACKGROUND;
            v.widgets.hovered.bg_fill = colors::LIGHT_HOVER;
            v.widgets.active.bg_fill = colors::LIGHT_SELECTED;
            v.override_text_color = Some(colors::TEXT_LIGHT_PRIMARY);
            v
        }
    };

    style.visuals = visuals;
    style.spacing.item_spacing = egui::vec2(spacing::ITEM_SPACING, spacing::ITEM_SPACING);
    style.spacing.button_padding = egui::vec2(spacing::BUTTON_PADDING, spacing::BUTTON_PADDING);
    style.spacing.menu_margin = Margin::same(spacing::PANEL_PADDING);
    style.visuals.window_rounding = Rounding::same(6.0);
    style.visuals.menu_rounding = Rounding::same(6.0);

    ctx.set_style(style);
}

/// Theme toggle state
pub struct ThemeManager {
    pub current_theme: UiTheme,
}

impl ThemeManager {
    pub fn new() -> Self {
        Self {
            current_theme: UiTheme::default(),
        }
    }

    pub fn toggle(&mut self) {
        self.current_theme = match self.current_theme {
            UiTheme::Dark => UiTheme::Light,
            UiTheme::Light => UiTheme::Dark,
        };
    }

    pub fn icon(&self) -> &'static str {
        match self.current_theme {
            UiTheme::Dark => "☀",  // Sun icon for light mode
            UiTheme::Light => "☾", // Moon icon for dark mode
        }
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Modern panel styling
pub struct PanelStyle;

impl PanelStyle {
    pub fn section(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
        ui.add_space(spacing::SECTION_SPACING);
        add_contents(ui);
        ui.add_space(spacing::SECTION_SPACING);
        ui.separator();
    }

    pub fn labeled_row(ui: &mut egui::Ui, label: &str, add_contents: impl FnOnce(&mut egui::Ui)) {
        ui.horizontal(|ui| {
            ui.label(label);
            add_contents(ui);
        });
    }

    pub fn icon_button(ui: &mut egui::Ui, icon: &str, tooltip: &str) -> egui::Response {
        let button = egui::Button::new(icon)
            .min_size(egui::vec2(32.0, 32.0))
            .fill(colors::DARK_BACKGROUND);

        let response = ui.add(button);
        if response.hovered() {
            egui::show_tooltip(ui.ctx(), response.id, |ui| {
                ui.label(tooltip);
            });
        }
        response
    }
}
