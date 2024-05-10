use serde::{Serialize, Deserialize};

use crate::excel::tools_res::PropState;

pub type Interact = Vec<InteractValue>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InteractValue {
    pub item_cost_list: Vec<ItemCostList>,
    #[serde(rename = "InteractID")]
    pub interact_id: u32,
    pub src_state: PropState,
    pub target_state: PropState,
    pub interact_cost_type: InteractCostType,
    pub interact_desc: InteractDesc,
    pub is_event: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum InteractCostType {
    #[serde(rename = "CheckItem")]
    CheckItem,
    #[serde(rename = "CostItem")]
    CostItem,
    None,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InteractDesc {
    pub hash: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ItemCostList {
    #[serde(rename = "ItemID")]
    pub item_id: u32,
    pub item_num: u32,
}