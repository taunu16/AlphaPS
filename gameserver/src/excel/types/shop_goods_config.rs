use serde::{Serialize, Deserialize};

pub type ShopGoodsConfig = Vec<ShopGoodsConfigElement>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ShopGoodsConfigElement {
    pub currency_list: Vec<i64>,
    pub currency_cost_list: Vec<i64>,
    pub limit_value1_list: Vec<i64>,
    pub limit_value2_list: Vec<Option<serde_json::Value>>,
    pub on_shelf_value1_list: Vec<i64>,
    #[serde(rename = "GoodsID")]
    pub goods_id: u32,
    #[serde(rename = "ItemID")]
    pub item_id: u32,
    #[serde(rename = "ItemGroupID")]
    pub item_group_id: u32,
    pub item_count: u32,
    pub level: u32,
    pub rank: u32,
    #[serde(rename = "GoodsSortID")]
    pub goods_sort_id: i64,
    pub limit_type1: LimitType,
    pub limit_type2: LimitType,
    pub on_shelf_type1: OnShelfType1,
    pub limit_times: i64,
    pub refresh_type: RefreshType,
    pub cycle_days: i64,
    pub refresh_offset: i64,
    #[serde(rename = "ShopID")]
    pub shop_id: u32,
    #[serde(rename = "ScheduleDataID")]
    pub schedule_data_id: i64,
    pub tag_type: i64,
    pub tag_param: i64,
    #[serde(rename = "ActivityModuleID")]
    pub activity_module_id: i64,
    pub is_limited_time_purchase: bool,
    pub is_on_sale: bool,
    pub is_new: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LimitType {
    #[serde(rename = "HasNoRefreshGoods")]
    HasNoRefreshGoods,
    Level,
    Null,
    #[serde(rename = "PreGoods")]
    PreGoods,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OnShelfType1 {
    #[serde(rename = "MainMission")]
    MainMission,
    Null,
    #[serde(rename = "SubMission")]
    SubMission,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RefreshType {
    #[serde(rename = "CYCLE")]
    Cycle,
    #[serde(rename = "DAILY")]
    Daily,
    #[serde(rename = "MONTH")]
    Month,
    None,
    #[serde(rename = "WEEK")]
    Week,
}