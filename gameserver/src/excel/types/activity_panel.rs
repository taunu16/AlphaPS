use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

pub type ActivityPanel = BTreeMap<String, ActivityPanelValue>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ActivityPanelValue {
    #[serde(rename = "UIPrefab")]
    pub ui_prefab: String,
    pub type_param: Vec<i64>,
    pub unlock_conditions: String,
    pub tab_icon: String,
    pub display_item_list: Vec<DisplayItemList>,
    pub action_name_list: Vec<ActionNameList>,
    pub action_name_list_on_tab: Vec<ActionNameList>,
    pub finish_conditions: Vec<FinishCondition>,
    #[serde(rename = "PanelID")]
    pub panel_id: u32,
    #[serde(rename = "Type")]
    pub activity_panel_type: i64,
    #[serde(rename = "ActivityModuleID")]
    pub activity_module_id: u32,
    pub hide_quest: i64,
    pub sort_weight: i64,
    #[serde(rename = "ActivityThemeID")]
    pub activity_theme_id: i64,
    #[serde(rename = "ResidentPanelUnlockModuleID")]
    pub resident_panel_unlock_module_id: i64,
    pub tab_name: IntroDesc,
    pub title_name: IntroDesc,
    pub panel_desc: IntroDesc,
    pub tag_desc: IntroDesc,
    pub intro_desc: IntroDesc,
    pub display_item_manual_sort: bool,
    pub is_activity_have_resident_part: bool,
    pub is_resident_panel: bool,
    pub daily_hint: bool,
    pub is_skip_switch_story_line: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ActionNameList {
    #[serde(rename = "ActionGroup_Cancel")]
    ActionGroupCancel,
    #[serde(rename = "ActionGroup_Return")]
    ActionGroupReturn,
    #[serde(rename = "ActionGroup_Scroll")]
    ActionGroupScroll,
    #[serde(rename = "ItemDetail")]
    ItemDetail,
    #[serde(rename = "Menu_Cancel")]
    MenuCancel,
    #[serde(rename = "Menu_Confirm")]
    MenuConfirm,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DisplayItemList {
    #[serde(rename = "ItemID")]
    pub item_id: i64,
    pub item_num: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FinishCondition {
    pub param: String,
    #[serde(rename = "Type")]
    pub finish_condition_type: Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "FinishMainMission")]
    FinishMainMission,
    #[serde(rename = "FinishQuest")]
    FinishQuest,
    #[serde(rename = "QuestClose")]
    QuestClose,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntroDesc {
    pub hash: i64,
}