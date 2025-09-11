// file: src/ui/rendering/themes.rs
// description: Enhanced theme management with corner style support

use crate::types::UISettings;
use egui;

pub fn apply_theme(ctx: &egui::Context, settings: &UISettings) {
    let bg_color = settings.get_background_color();
    let text_color = settings.get_text_color();
    let accent_color = settings.get_accent_color();
    let rounding = settings.get_rounding();

    let mut style = (*ctx.style()).clone();

    // Basic colors
    style.visuals.window_fill = bg_color;
    style.visuals.panel_fill = bg_color;
    style.visuals.override_text_color = Some(text_color);

    // Corner rounding
    style.visuals.window_corner_radius = rounding;
    style.visuals.menu_corner_radius = rounding;

    // Widget rounding
    style.visuals.widgets.noninteractive.corner_radius = rounding;
    style.visuals.widgets.inactive.corner_radius = rounding;
    style.visuals.widgets.hovered.corner_radius = rounding;
    style.visuals.widgets.active.corner_radius = rounding;
    style.visuals.widgets.open.corner_radius = rounding;

    // Selection colors
    style.visuals.selection.bg_fill = accent_color.gamma_multiply(0.3);
    style.visuals.selection.stroke.color = accent_color;

    // Hyperlink color
    style.visuals.hyperlink_color = settings.get_link_color();

    // Button styling with corner preference
    if settings.is_rounded() {
        // More pronounced rounding for buttons in rounded mode
        style.visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(6);
        style.visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(6);
        style.visuals.widgets.active.corner_radius = egui::CornerRadius::same(6);
    }

    // Enhanced spacing
    style.spacing.item_spacing = egui::Vec2::splat(8.0);
    style.spacing.button_padding = egui::Vec2::new(12.0, 6.0);
    style.spacing.menu_margin = egui::Margin::same(8);
    style.spacing.window_margin = egui::Margin::same(8);

    ctx.set_style(style);
}
