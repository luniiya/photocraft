//! Gradients: gradient presets.

use super::{Module, Size};

pub const MODULE: Module = Module {
    id: "gradients",
    title: "Gradients",
    icon: "blend",
    menu_ids: &["window.panel.gradients"],
    siblings: &["color", "swatches", "patterns"],
    size: Size::new(190.0, 130.0, 80.0),
    fills: false,
    group: crate::dock::Group::Color,
    menu: None,
};
