//! Paragraph: alignment, indents and spacing of the selected text.

use super::{Module, Size};

pub const MODULE: Module = Module {
    id: "paragraph",
    title: "Paragraph",
    icon: "align-left",
    menu_ids: &["window.panel.paragraph", "type.panels.paragraph"],
    siblings: &["character"],
    size: Size::new(270.0, 160.0, 80.0),
    fills: false,
    group: crate::dock::Group::Character,
    menu: None,
};
