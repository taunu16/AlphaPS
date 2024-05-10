use serde::{Serialize, Deserialize};

pub type BackgroundMusicConfig = Vec<BackgroundMusicConfigValue>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BackgroundMusicConfigValue {
    pub music_switch_name: String,
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(rename = "GroupID")]
    pub group_id: u32,
    #[serde(rename = "BPM")]
    pub bpm: i64,
    pub rhythm_colour: RhythmColour,
    pub music_name: BgmDesc,
    pub unlock_desc: BgmDesc,
    #[serde(rename = "BGMDesc")]
    pub bgm_desc: BgmDesc,
    pub unlock: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BgmDesc {
    pub hash: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RhythmColour {
    #[serde(rename = "TYPE_EASY")]
    TypeEasy,
    #[serde(rename = "TYPE_TENSE")]
    TypeTense,
}