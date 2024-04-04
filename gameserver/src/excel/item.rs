use super::types::ItemUseBuffData;

const ITEM_USE_BUFF_DATA_JSON: &str = include_str!("./data/ExcelOutput/ItemUseBuffData.json");

pub struct Item {
    pub buff_data: ItemUseBuffData
}

impl Item {
    pub fn new() -> Self {
        Self {
            buff_data: serde_json::from_str(ITEM_USE_BUFF_DATA_JSON).expect("ItemUseBuffData parse failed")
        }
    }
}