use super::{types::{ActivityPanel, BackgroundMusicConfig}, Item};

const ACTIVITY_PANEL_JSON: &str = include_str!("./data/ExcelOutput/ActivityPanel.json");
const BACKGROUND_MUSIC_CONFIG_JSON: &str = include_str!("./data/ExcelOutput/BackGroundMusicConfig.json");

pub struct ExcelManager {
    pub item: Item,
    pub activity_panel: ActivityPanel,
    pub background_music_config: BackgroundMusicConfig,
}

impl ExcelManager {
    pub fn new() -> Self {
        Self {
            item: Item::new(),
            activity_panel: serde_json::from_str(ACTIVITY_PANEL_JSON).expect("ActivityPanel parse failed"),
            background_music_config: serde_json::from_str(BACKGROUND_MUSIC_CONFIG_JSON).expect("BackGroundMusicConfig parse failed"),
        }
    }
}