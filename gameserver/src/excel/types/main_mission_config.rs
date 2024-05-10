use serde::{Serialize, Deserialize};

pub type MainMissionConfig = Vec<MainMissionConfigElement>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MainMissionConfigElement {
    pub next_main_mission_list: Vec<Option<serde_json::Value>>,
    pub take_param: Vec<Param>,
    pub begin_param: Vec<Param>,
    pub sub_reward_list: Vec<i64>,
    #[serde(rename = "MainMissionID")]
    pub main_mission_id: u32,
    #[serde(rename = "Type")]
    pub main_mission_config_type: MainMissionConfigType,
    pub display_priority: i64,
    pub take_operation: Operation,
    pub begin_operation: Operation,
    pub next_track_main_mission: i64,
    pub track_weight: i64,
    pub mission_advance: i64,
    #[serde(rename = "RewardID")]
    pub reward_id: i64,
    #[serde(rename = "DisplayRewardID")]
    pub display_reward_id: i64,
    pub mission_pack: i64,
    #[serde(rename = "ChapterID")]
    pub chapter_id: i64,
    pub name: Name,
    pub is_in_raid: bool,
    pub is_display_activity_icon: bool,
    pub inited: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Operation {
    And,
    Or,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Param {
    #[serde(rename = "Type")]
    pub param_type: BeginParamType,
    pub value: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BeginParamType {
    Auto,
    #[serde(rename = "HeliobusPhaseReach")]
    HeliobusPhaseReach,
    Manual,
    #[serde(rename = "MultiSequence")]
    MultiSequence,
    #[serde(rename = "MuseumPhaseRenewPointReach")]
    MuseumPhaseRenewPointReach,
    #[serde(rename = "PlayerLevel")]
    PlayerLevel,
    #[serde(rename = "SequenceNextDay")]
    SequenceNextDay,
    #[serde(rename = "WorldLevel")]
    WorldLevel,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MainMissionConfigType {
    Branch,
    Companion,
    Daily,
    Gap,
    Main,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Name {
    pub hash: i64,
}
