use serde::{Serialize, Deserialize};

pub type ItemUseBuffData = Vec<ItemUseBuffDatum>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ItemUseBuffDatum {
    pub consume_tag: Vec<ConsumeTag>,
    pub maze_buff_param: Vec<Option<serde_json::Value>>,
    pub maze_buff_param2: Vec<Option<serde_json::Value>>,
    pub use_effect: UseEffect,
    #[serde(rename = "UseDataID")]
    pub use_data_id: u32,
    pub consume_type: i64,
    pub use_target_type: UseTargetType,
    #[serde(rename = "MazeBuffID")]
    pub maze_buff_id: i64,
    #[serde(rename = "MazeBuffID2")]
    pub maze_buff_id2: i64,
    pub satiety_value: i64,
    pub use_multiple_max: i64,
    #[serde(rename = "PreviewHPRecoveryPercent")]
    pub preview_hp_recovery_percent: f64,
    #[serde(rename = "PreviewHPRecoveryValue")]
    pub preview_hp_recovery_value: f64,
    pub preview_skill_point: f64,
    pub preview_power_percent: f64,
    pub activity_count: i64,
    #[serde(rename = "IsCheckHP")]
    pub is_check_hp: bool,
    pub is_show_item_desc: bool,
    pub is_show_use_multiple_slider: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ConsumeTag {
    #[serde(rename = "BP")]
    Bp,
    #[serde(rename = "HP")]
    Hp,
    #[serde(rename = "SP")]
    Sp,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub enum UseTargetType {
    Alive,
    All,
    Dead,
}
