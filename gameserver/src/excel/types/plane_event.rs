use serde::{Serialize, Deserialize};

pub type PlaneEvent = Vec<PlaneEventElement>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlaneEventElement {
    pub drop_list: Vec<i64>,
    pub display_item_list: Vec<Option<serde_json::Value>>,
    #[serde(rename = "EventID")]
    pub event_id: i64,
    pub world_level: i64,
    #[serde(rename = "StageID")]
    pub stage_id: i64,
    pub reward: i64,
    pub avatar_exp_reward: i64,
    pub is_use_monster_drop: bool,
}