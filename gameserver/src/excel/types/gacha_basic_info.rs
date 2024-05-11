use serde::{Serialize, Deserialize};

pub type GachaBasicInfo = Vec<GachaBasicInfoElement>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GachaBasicInfoElement {
    pub start_time: String,
    pub end_time: String,
    pub prefab_path: String,
    pub pool_label_icon: String,
    pub pool_label_icon_selected: String,
    #[serde(rename = "GachaID")]
    pub gacha_id: u32,
    pub gacha_type: GachaType,
    #[serde(rename = "SortID")]
    pub sort_id: u32,
    pub pool_name: PoolDesc,
    pub pool_desc: PoolDesc,
    pub type_title: PoolDesc,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum GachaType {
    #[serde(rename = "AvatarUp")]
    AvatarUp,
    Newbie,
    Normal,
    #[serde(rename = "WeaponUp")]
    WeaponUp,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PoolDesc {
    pub hash: i64,
}
