use super::types::{ItemUseBuffData, PasterConfig};

const ITEM_USE_BUFF_DATA_JSON: &str = include_str!("./data/ExcelOutput/ItemUseBuffData.json");
const PASTER_CONFIG_JSON: &str = include_str!("./data/ExcelOutput/PasterConfig.json");

pub struct Item {
    pub buff_data: ItemUseBuffData,
    pub paster_config: PasterConfig
}

impl Item {
    pub fn new() -> Self {
        Self {
            buff_data: serde_json::from_str(ITEM_USE_BUFF_DATA_JSON).expect("ItemUseBuffData parse failed"),
            paster_config: serde_json::from_str(PASTER_CONFIG_JSON).expect("PatsterConfig parse failed")
        }
    }
}