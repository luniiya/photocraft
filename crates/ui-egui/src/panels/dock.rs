use super::history::history;
use super::info::info_panel;
use super::layers::{channels, layers};
use super::navigator::navigator;
use super::properties::{adjustments_grid, properties_body};
use super::{PhotocraftApp, Stroke, Tokens, icons, pos2};

// ----------------------------------------------------------------------------- dock

/// The panels and the canvas in the space left by the menus, bars and toolbar: craft-ui's
/// docking layout, with the icon rail at the window's end edge in the Studio themes.
pub fn dock_area(app: &mut PhotocraftApp, ui: &mut egui::Ui) {
    let t = Tokens::get(ui.ctx());
    if t.kind.is_bubble() {
        let workspace = bubble_canvas_panel(app, ui, &t, true);
        if app.ui.panels.rail {
            floating_rail(app, ui.ctx(), workspace);
        }
    } else {
        if app.ui.panels.rail && !t.pro {
            rail(app, ui, &t);
        }
        egui::CentralPanel::default().frame(egui::Frame::NONE).show(ui, |ui| crate::panel_docking::show(app, ui));
    }
    crate::dock::persist(app, ui.ctx());
    crate::color_panel_ui::persist(app);
}

pub(crate) fn bubble_frame_rect(rect: egui::Rect) -> egui::Rect {
    rect.shrink(8.0)
}

pub(crate) fn bubble_canvas_panel(app: &mut PhotocraftApp, ui: &mut egui::Ui, t: &Tokens, docking: bool) -> egui::Rect {
    let workspace = ui.available_rect_before_wrap();
    let backdrop = crate::prefs_ui::pasteboard_color(app).unwrap_or(t.canvas);
    egui::CentralPanel::default().frame(egui::Frame::NONE.fill(t.chrome)).show(ui, |ui| {
        let canvas = bubble_frame_rect(workspace);
        ui.painter().rect(canvas, t.frame_radius(), backdrop, Stroke::NONE, egui::StrokeKind::Inside);
        let mut canvas_ui = ui.new_child(egui::UiBuilder::new().max_rect(canvas));
        crate::canvas::document_area(app, &mut canvas_ui);
        if docking {
            let dock_rect = egui::Rect::from_min_max(workspace.min + egui::vec2(20.0, 12.0), workspace.max - egui::vec2(72.0, 20.0));
            let mut overlay = ui.new_child(egui::UiBuilder::new().max_rect(dock_rect));
            crate::panel_docking::show(app, &mut overlay);
        }
    });
    workspace
}

/// The icon rail on its own, for the Bubble styles' rail-only workspace.
pub(crate) fn rail_panel(app: &mut PhotocraftApp, ui: &mut egui::Ui) {
    let t = Tokens::get(ui.ctx());
    if app.ui.panels.rail {
        rail(app, ui, &t);
    }
}

/// The docking layout's protected canvas panel: the document area, with the icon rail on its end
/// side, beside the dock, in the Pro themes.
pub(crate) fn canvas_panel(app: &mut PhotocraftApp, ui: &mut egui::Ui) {
    let t = Tokens::get(ui.ctx());
    let backdrop = if app.ui.view.hides_chrome() { egui::Color32::BLACK } else { crate::prefs_ui::pasteboard_color(app).unwrap_or(t.canvas) };
    if app.ui.panels.rail && t.pro {
        rail(app, ui, &t);
    }
    egui::CentralPanel::default().frame(egui::Frame::NONE.fill(backdrop)).show(ui, |ui| crate::canvas::document_area(app, ui));
}

/// Narrow icon rail, unless an embedding app hides it: shows, expands or collapses panel groups.
fn rail(app: &mut PhotocraftApp, ui: &mut egui::Ui, t: &Tokens) {
    let (rw, _, margin) = rail_metrics(t);
    crate::direction::end_panel("rail")
        .resizable(false)
        .exact_size(rw)
        .frame(egui::Frame::NONE.fill(t.chrome).inner_margin(margin))
        .show(ui, |ui| rail_body(app, ui, t));
}

fn rail_metrics(t: &Tokens) -> (f32, f32, egui::Margin) {
    if t.kind.is_bubble() {
        (44.0, 34.0, egui::Margin::symmetric(5, 6))
    } else if t.pro {
        (36.0, 28.0, egui::Margin::symmetric(4, 8))
    } else {
        (44.0, 32.0, egui::Margin::symmetric(4, 8))
    }
}

/// The Bubble rail as a floating card over the canvas.
pub(crate) fn floating_rail(app: &mut PhotocraftApp, ctx: &egui::Context, rect: egui::Rect) {
    let t = Tokens::get(ctx);
    let (rw, _, margin) = rail_metrics(&t);
    egui::Area::new(egui::Id::unique("bubble-rail")).order(egui::Order::Middle).fixed_pos(egui::pos2(rect.right() - 20.0 - rw, rect.top() + 12.0)).show(
        ctx,
        |ui| {
            ui.set_width(rw);
            ui.set_height(rect.height() - 32.0);
            egui::Frame::NONE
                .fill(t.glass_card())
                .stroke(Stroke::new(1.0, t.card_border))
                .corner_radius(t.radius_lg)
                .inner_margin(margin)
                .show(ui, |ui| rail_body(app, ui, &t));
        },
    );
}

fn rail_body(app: &mut PhotocraftApp, ui: &mut egui::Ui, t: &Tokens) {
    let customized = app.ui.docking.is_some();
    let p = app.ui.panels.clone();
    let bubble = t.kind.is_bubble();
    let (_, rb, _) = rail_metrics(t);
    if bubble {
        ui.add_space(4.0);
    } else {
        // The separator is on the canvas side.
        let r = ui.max_rect();
        let x = if crate::direction::is_rtl() { r.right() + 6.0 } else { r.left() - 6.0 };
        ui.painter().line_segment([pos2(x, r.top() - 8.0), pos2(x, r.bottom() + 8.0)], Stroke::new(1.0, t.separator));
    }
    ui.spacing_mut().item_spacing.y = 4.0;
    use crate::dock::Group;
    let entries: [(&str, &str, Group); 6] = [
        ("sliders-horizontal", tl!("Properties"), Group::Properties),
        ("navigation", tl!("Navigator"), Group::Navigator),
        ("palette", tl!("Color & Swatches"), Group::Color),
        ("layers", tl!("Layers"), Group::Layers),
        ("clock", tl!("History"), Group::History),
        ("brush", tl!("Brushes"), Group::Brushes),
    ];
    for (icon, name, g) in entries {
        // Studio floats Properties outside the dock; Bubble docks it with the rest.
        let docked = t.pro || t.kind.is_bubble() || g != Group::Properties;
        let on = if customized { crate::panel_docking::rail_on(app, g) } else { g.shown(&p) && !(docked && app.ui.dock.is_collapsed(g)) };
        if icons::rail_button(ui, icon, rb, on, name).clicked() {
            let opened = bubble && !app.ui.panels.dock;
            if opened {
                app.ui.panels.dock = true;
            }
            if customized {
                crate::panel_docking::rail_click(app, g);
            } else if opened {
                crate::dock::reveal(app, g);
            } else {
                crate::dock::rail_click(app, g, docked);
            }
        }
    }
}

/// The dock's narrowest width (points). It is the right dock, on the left in a right-to-left UI.
pub(crate) const DOCK_MIN_WIDTH: f32 = 250.0;

/// What the dock leaves the toolbar and the canvas when it is widest (points).
const DOCK_LEAVES: f32 = 400.0;

/// The dock's width range (points) in a window `window_width` wide: from [`DOCK_MIN_WIDTH`] to
/// everything but [`DOCK_LEAVES`], and never capped below the old 520, so two-column layouts can
/// be as wide as the window allows (#7896).
pub(crate) fn dock_width_range(window_width: f32) -> std::ops::RangeInclusive<f32> {
    let max = if window_width.is_finite() { (window_width - DOCK_LEAVES).max(520.0) } else { 520.0 };
    DOCK_MIN_WIDTH..=max
}

pub(crate) fn dock_width_id() -> egui::Id {
    egui::Id::unique("dock-width-request")
}

/// Set the right dock's width on the next frame (`ui.set {dockWidth}`); clamped to its range.
pub fn request_dock_width(ctx: &egui::Context, w: f32) {
    let range = dock_width_range(ctx.content_rect().width());
    let w = if w.is_finite() { w.clamp(*range.start(), *range.end()) } else { *range.start() };
    ctx.data_mut(|d| d.insert_temp(dock_width_id(), w));
}

/// One dock group's tab content; `panel_docking` bounds it and scrolls it when it's taller.
pub(crate) fn dock_body(app: &mut PhotocraftApp, ui: &mut egui::Ui, group: crate::dock::Group, tab: usize) {
    use crate::dock::Group;
    let pro = Tokens::get(ui.ctx()).pro;
    match (group, tab) {
        (Group::Color, 2) => crate::preset_panels::gradients_panel(app, ui),
        (Group::Color, 3) => crate::preset_panels::patterns_panel(app, ui),
        // The Color tab is the same in every theme, in the mode picked from its panel menu
        // (#6000); only the tab order differs between themes.
        (Group::Color, tab) if group.tabs(pro).get(tab) == Some(&"Color") => crate::color_panel_ui::panel(app, ui),
        (Group::Color, _) => crate::swatches_ui::panel(app, ui),
        (Group::Properties, 0) => properties_body(app, ui),
        (Group::Properties, _) => adjustments_grid(app, ui),
        (Group::Character, tab) => crate::type_tool::character_panel(app, ui, tab == 1),
        (Group::Navigator, 0) => navigator(app, ui),
        (Group::Navigator, 1) => crate::histogram_panel::show(app, ui),
        (Group::Navigator, _) => info_panel(app, ui),
        (Group::History, 0) => history(app, ui),
        (Group::History, 1) => crate::actions::panel(app, ui),
        (Group::History, _) => crate::comps_ui::panel(app, ui),
        (Group::Layers, 0) => layers(app, ui),
        (Group::Layers, 1) => channels(app, ui),
        (Group::Layers, _) => crate::vector_ui::paths_panel(app, ui),
        (Group::Brushes, _) => crate::brushes_tab::show(app, ui),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn bubble_frame_has_a_uniform_band() {
        let outer = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(100.0, 80.0));
        assert_eq!(super::bubble_frame_rect(outer), egui::Rect::from_min_max(egui::pos2(8.0, 8.0), egui::pos2(92.0, 72.0)));
    }
}
