use super::types::*;

const ITEM_USE_BUFF_DATA_JSON: &str = include_str!("./data/ExcelOutput/ItemUseBuffData.json");
const PASTER_CONFIG_JSON: &str = include_str!("./data/ExcelOutput/PasterConfig.json");
const ITEM_CONFIG_JSON: &str = include_str!("./data/ExcelOutput/ItemConfig.json");
// const REWARD_DATA_JSON: &str = include_str!("./data/ExcelOutput/RewardData.json");
// const PLANE_EVENT_JSON: &str = include_str!("./data/ExcelOutput/PlaneEvent.json");

pub struct Item {
    pub buff_data: ItemUseBuffData,
    pub paster_config: PasterConfig,
    pub item_config: ItemConfig,
    // pub reward_data: RewardData,
    // pub plane_event: PlaneEvent,
}

impl Item {
    pub fn new() -> Self {
        Self {
            buff_data: serde_json::from_str(ITEM_USE_BUFF_DATA_JSON).expect("ItemUseBuffData parse failed"),
            paster_config: serde_json::from_str(PASTER_CONFIG_JSON).expect("PatsterConfig parse failed"),
            item_config: serde_json::from_str(ITEM_CONFIG_JSON).expect("ItemConfig parse failed"),
            // reward_data: serde_json::from_str(REWARD_DATA_JSON).expect("RewardData parse failed"),
            // plane_event: serde_json::from_str(PLANE_EVENT_JSON).expect("PlaneEvent parse failed"),
        }
    }
}