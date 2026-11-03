//! Swatches: saved colours, in groups.

use super::{Module, Size};

pub const MODULE: Module = Module {
    id: "swatches",
    title: "Swatches",
    icon: "swatch-book",
    menu_ids: &["window.panel.swatches"],
    siblings: &["color", "gradients", "patterns"],
    size: Size::new(190.0, 130.0, 80.0),
    fills: false,
    group: crate::dock::Group::Color,
    menu: Some(crate::swatches_ui::panel_menu),
};
