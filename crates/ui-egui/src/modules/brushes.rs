//! Brushes: the brush presets panel.

use super::{Module, Size};

pub const MODULE: Module = Module {
    id: "brushes",
    title: "Brushes",
    icon: "brush",
    menu_ids: &["window.panel.brushes"],
    siblings: &[],
    size: Size::new(200.0, 150.0, 100.0),
    fills: false,
    group: crate::dock::Group::Brushes,
    menu: None,
};
