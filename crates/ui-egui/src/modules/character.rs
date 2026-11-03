//! Character: type settings of the selected text.

use super::{Module, Size};

pub const MODULE: Module = Module {
    id: "character",
    title: "Character",
    icon: "type",
    menu_ids: &["window.panel.character", "type.panels.character"],
    siblings: &["paragraph"],
    size: Size::new(270.0, 160.0, 80.0),
    fills: false,
    group: crate::dock::Group::Character,
    menu: None,
};
