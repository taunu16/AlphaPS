use serde::{Serialize, Deserialize};
use std::collections::HashMap;

pub type PasterConfig = HashMap<String, PasterConfigValue>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PasterConfigValue {
    #[serde(rename = "TravelBrochureID")]
    pub travel_brochure_id: Vec<i64>,
    pub text_paster_prefab: String,
    #[serde(rename = "ID")]
    pub id: u32,
    pub increase_completion: i64,
    #[serde(rename = "Type")]
    pub paster_config_type: Type,
    pub paster_textmap: Paster,
    pub paster_unlock_desc: Paster,
    pub default_unlock: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    Image,
    Text,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Paster {
    pub hash: i64,
}