//! Color: the foreground/background colour field (Pro) or picker.

use super::{Module, Size};

pub const MODULE: Module = Module {
    id: "color",
    title: "Color",
    icon: "palette",
    menu_ids: &["window.panel.color"],
    siblings: &["swatches", "gradients", "patterns"],
    size: Size::new(190.0, 130.0, 80.0),
    fills: false,
    group: crate::dock::Group::Color,
    menu: None,
};
