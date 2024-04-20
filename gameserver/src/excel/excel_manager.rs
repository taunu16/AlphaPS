use super::{types::ActivityPanel, Item};

const ACTIVITY_PANEL_JSON: &str = include_str!("./data/ExcelOutput/ActivityPanel.json");

pub struct ExcelManager {
    pub item: Item,
    pub activity_panel: ActivityPanel
}

impl ExcelManager {
    pub fn new() -> Self {
        Self {
            item: Item::new(),
            activity_panel: serde_json::from_str(ACTIVITY_PANEL_JSON).expect("ActivityPanel parse failed")
        }
    }
}