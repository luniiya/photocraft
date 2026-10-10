//! History: undo states and snapshots.

use super::{Module, Size};

pub const MODULE: Module = Module {
    id: "history",
    title: "History",
    icon: "clock",
    menu_ids: &["window.panel.history"],
    siblings: &["actions", "layerComps"],
    size: Size::new(200.0, 130.0, 80.0),
    fills: true,
    body: crate::panels::history,
    menu: None,
};
