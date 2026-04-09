//! Seisly UI Styles and Theming System
//!
//! Semantic token-based design system for complete theme flexibility.

use eframe::egui;
use egui::{Color32, FontFamily, FontId, Rounding, Stroke, Visuals};
use serde::{Deserialize, Serialize};

/// Semantic theme tokens (VS Code-style)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub is_dark: bool,

    // Activity Bar
    pub activity_bar_bg: Color32,
    pub activity_bar_fg: Color32,
    pub activity_bar_active_icon: Color32,
    pub activity_bar_inactive_icon: Color32,
    pub activity_bar_border: Color32,

    // Side Bar
    pub side_bar_bg: Color32,
    pub side_bar_fg: Color32,
    pub side_bar_border: Color32,
    pub side_bar_header_fg: Color32,

    // Editor / Viewport
    pub editor_bg: Color32,
    pub editor_fg: Color32,

    // Bottom Panel
    pub panel_bg: Color32,
    pub panel_border: Color32,
    pub panel_header_fg: Color32,

    // Status Bar
    pub status_bar_bg: Color32,
    pub status_bar_fg: Color32,

    // Accents & UI Elements
    pub accent: Color32,
    pub selection_bg: Color32,
    pub hover_bg: Color32,
    pub border: Color32,
    pub text_primary: Color32,
    pub text_secondary: Color32,
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            name: "Seisly Dark".to_string(),
            is_dark: true,
            activity_bar_bg: Color32::from_rgb(51, 51, 51),
            activity_bar_fg: Color32::from_rgb(255, 255, 255),
            activity_bar_active_icon: Color32::from_rgb(255, 255, 255),
            activity_bar_inactive_icon: Color32::from_rgb(133, 133, 133),
            activity_bar_border: Color32::from_rgb(37, 37, 38),
            side_bar_bg: Color32::from_rgb(37, 37, 38),
            side_bar_fg: Color32::from_rgb(204, 204, 204),
            side_bar_border: Color32::from_rgb(37, 37, 38),
            side_bar_header_fg: Color32::from_rgb(187, 187, 187),
            editor_bg: Color32::from_rgb(30, 30, 30),
            editor_fg: Color32::from_rgb(255, 255, 255),
            panel_bg: Color32::from_rgb(30, 30, 30),
            panel_border: Color32::from_rgb(37, 37, 38),
            panel_header_fg: Color32::from_rgb(204, 204, 204),
            status_bar_bg: Color32::from_rgb(0, 122, 204),
            status_bar_fg: Color32::from_rgb(255, 255, 255),
            accent: Color32::from_rgb(0, 122, 204),
            selection_bg: Color32::from_rgb(38, 79, 120),
            hover_bg: Color32::from_rgb(42, 45, 46),
            border: Color32::from_rgb(61, 61, 61),
            text_primary: Color32::from_rgb(255, 255, 255),
            text_secondary: Color32::from_rgb(176, 176, 176),
        }
    }

    pub fn light() -> Self {
        Self {
            name: "Seisly Light".to_string(),
            is_dark: false,
            activity_bar_bg: Color32::from_rgb(44, 44, 44),
            activity_bar_fg: Color32::from_rgb(255, 255, 255),
            activity_bar_active_icon: Color32::from_rgb(255, 255, 255),
            activity_bar_inactive_icon: Color32::from_rgb(133, 133, 133),
            activity_bar_border: Color32::from_rgb(229, 229, 229),
            side_bar_bg: Color32::from_rgb(243, 243, 243),
            side_bar_fg: Color32::from_rgb(97, 97, 97),
            side_bar_border: Color32::from_rgb(229, 229, 229),
            side_bar_header_fg: Color32::from_rgb(107, 107, 107),
            editor_bg: Color32::from_rgb(255, 255, 255),
            editor_fg: Color32::from_rgb(51, 51, 51),
            panel_bg: Color32::from_rgb(255, 255, 255),
            panel_border: Color32::from_rgb(229, 229, 229),
            panel_header_fg: Color32::from_rgb(97, 97, 97),
            status_bar_bg: Color32::from_rgb(0, 122, 204),
            status_bar_fg: Color32::from_rgb(255, 255, 255),
            accent: Color32::from_rgb(0, 122, 204),
            selection_bg: Color32::from_rgb(173, 214, 255),
            hover_bg: Color32::from_rgb(232, 232, 232),
            border: Color32::from_rgb(206, 206, 206),
            text_primary: Color32::from_rgb(51, 51, 51),
            text_secondary: Color32::from_rgb(100, 100, 100),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

/// Spacing constants (IDE layout)
pub mod spacing {
    pub const ACTIVITY_BAR_WIDTH: f32 = 48.0;
    pub const SIDEBAR_DEFAULT_WIDTH: f32 = 260.0;
    pub const BOTTOM_PANEL_DEFAULT_HEIGHT: f32 = 200.0;
    pub const STATUS_BAR_HEIGHT: f32 = 22.0;

    pub const ITEM_SPACING: f32 = 4.0;
    pub const BUTTON_PADDING: f32 = 6.0;
}

/// Typography
pub mod typography {
    use super::*;

    #[allow(dead_code)]
    pub fn heading() -> FontId {
        FontId::new(13.0, FontFamily::Proportional)
    }

    #[allow(dead_code)]
    pub fn body() -> FontId {
        FontId::new(11.0, FontFamily::Proportional)
    }

    #[allow(dead_code)]
    pub fn label() -> FontId {
        FontId::new(10.0, FontFamily::Proportional)
    }

    #[allow(dead_code)]
    pub fn status() -> FontId {
        FontId::new(10.0, FontFamily::Monospace)
    }
}

/// Apply theme to egui context
pub fn apply_theme(ctx: &egui::Context, theme: &Theme) {
    let mut style = (*ctx.style()).clone();

    let mut visuals = if theme.is_dark {
        Visuals::dark()
    } else {
        Visuals::light()
    };

    visuals.panel_fill = theme.side_bar_bg;
    visuals.window_fill = theme.side_bar_bg;
    visuals.selection.bg_fill = theme.selection_bg;
    visuals.selection.stroke = Stroke::new(1.0, theme.accent);
    visuals.widgets.noninteractive.bg_fill = theme.editor_bg;
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, theme.text_secondary);
    visuals.widgets.hovered.bg_fill = theme.hover_bg;
    visuals.widgets.active.bg_fill = theme.selection_bg;
    visuals.override_text_color = Some(theme.text_primary);

    style.visuals = visuals;
    style.spacing.item_spacing = egui::vec2(spacing::ITEM_SPACING, spacing::ITEM_SPACING);
    style.spacing.button_padding = egui::vec2(spacing::BUTTON_PADDING, spacing::BUTTON_PADDING);
    style.visuals.window_rounding = Rounding::same(4.0);
    style.visuals.menu_rounding = Rounding::same(4.0);

    ctx.set_style(style);
}

/// Theme toggle state
pub struct ThemeManager {
    pub current_theme: Theme,
}

impl ThemeManager {
    pub fn new() -> Self {
        Self {
            current_theme: Theme::dark(),
        }
    }

    pub fn toggle(&mut self) {
        if self.current_theme.is_dark {
            self.current_theme = Theme::light();
        } else {
            self.current_theme = Theme::dark();
        }
    }

    pub fn set_theme(&mut self, name: &str) {
        match name {
            "Seisly Dark" => self.current_theme = Theme::dark(),
            "Seisly Light" => self.current_theme = Theme::light(),
            _ => self.current_theme = Theme::dark(),
        }
    }

    pub fn icon(&self) -> &'static str {
        if self.current_theme.is_dark {
            "☀" // Sun for dark mode
        } else {
            "☾" // Moon for light mode
        }
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}

pub mod colors {
    // Keep legacy accent colors for now to avoid breaking existing widgets
    use egui::Color32;
    #[allow(dead_code)]
    pub const SEISMIC: Color32 = Color32::from_rgb(79, 195, 247);
    #[allow(dead_code)]
    pub const HORIZON: Color32 = Color32::from_rgb(129, 199, 132);
    #[allow(dead_code)]
    pub const FAULT: Color32 = Color32::from_rgb(229, 115, 115);
}
