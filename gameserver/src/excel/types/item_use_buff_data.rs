use serde::{Serialize, Deserialize};

use std::collections::HashMap;

pub type ItemUseBuffData = HashMap<String, ItemUseBuffDataValue>;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ItemUseBuffDataValue {
    #[serde(rename = "UseDataID")]
    pub use_data_id: i64,
    pub consume_type: i64,
    pub consume_tag: Vec<ConsumeTag>,
    pub use_target_type: Option<UseTargetType>,
    #[serde(rename = "MazeBuffID")]
    pub maze_buff_id: i64,
    pub maze_buff_param: Vec<Option<serde_json::Value>>,
    pub maze_buff_param2: Vec<Option<serde_json::Value>>,
    pub use_multiple_max: i64,
    #[serde(rename = "IsCheckHP")]
    pub is_check_hp: Option<bool>,
    pub use_effect: UseEffect,
    #[serde(rename = "PreviewHPRecoveryPercent")]
    pub preview_hp_recovery_percent: Option<f64>,
    #[serde(rename = "PreviewHPRecoveryValue")]
    pub preview_hp_recovery_value: Option<i64>,
    pub is_show_item_desc: bool,
    pub activity_count: i64,
    #[serde(rename = "MazeBuffID2")]
    pub maze_buff_id2: Option<i64>,
    pub preview_skill_point: Option<u32>,
    pub preview_power_percent: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub enum ConsumeTag {
    #[serde(rename = "BP")]
    Bp,
    #[serde(rename = "HP")]
    Hp,
    #[serde(rename = "SP")]
    Sp,
}

#[derive(Serialize, Deserialize)]
pub enum UseEffect {
    #[serde(rename = "AvatarItemIcon_Eff_AtkUp")]
    AvatarItemIconEffAtkUp,
    #[serde(rename = "AvatarItemIcon_Eff_Common")]
    AvatarItemIconEffCommon,
    #[serde(rename = "AvatarItemIcon_Eff_DefUp")]
    AvatarItemIconEffDefUp,
    #[serde(rename = "AvatarItemIcon_Eff_Heal")]
    AvatarItemIconEffHeal,
    #[serde(rename = "AvatarItemIcon_Eff_Hurt")]
    AvatarItemIconEffHurt,
    #[serde(rename = "AvatarItemIcon_Eff_MpGet")]
    AvatarItemIconEffMpGet,
    #[serde(rename = "AvatarItemIcon_Eff_SkillPoints")]
    AvatarItemIconEffSkillPoints,
}

#[derive(Serialize, Deserialize)]
pub enum UseTargetType {
    Alive,
    Dead,
}