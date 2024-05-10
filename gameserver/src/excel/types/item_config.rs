use serde::{Serialize, Deserialize};

pub type ItemConfig = Vec<ItemConfigElement>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ItemConfigElement {
    pub item_icon_path: String,
    pub item_figure_icon_path: String,
    pub item_currency_icon_path: String,
    pub item_avatar_icon_path: String,
    pub custom_data_list: Vec<i64>,
    #[serde(rename = "ReturnItemIDList")]
    pub return_item_id_list: Vec<ReturnItemIdList>,
    #[serde(rename = "ID")]
    pub id: u32,
    pub item_main_type: ItemType,
    pub item_sub_type: ItemType,
    pub inventory_display_tag: i64,
    pub rarity: Rarity,
    pub purpose_type: i64,
    pub pile_limit: u32,
    pub use_method: UseMethod,
    #[serde(rename = "UseDataID")]
    pub use_data_id: i64,
    pub item_group: i64,
    pub sell_type: SellType,
    pub item_name: Item,
    pub item_desc: Item,
    #[serde(rename = "ItemBGDesc")]
    pub item_bg_desc: Item,
    #[serde(rename = "isVisible")]
    pub is_visible: bool,
    pub is_sellable: bool,
    pub is_show_red_dot: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub hash: i64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ItemType {
    #[serde(rename = "AetherSkill")]
    AetherSkill,
    #[serde(rename = "AetherSpirit")]
    AetherSpirit,
    #[serde(rename = "AvatarCard")]
    AvatarCard,
    Book,
    #[serde(rename = "ChatBubble")]
    ChatBubble,
    #[serde(rename = "ChessRogueDiceSurface")]
    ChessRogueDiceSurface,
    Display,
    Eidolon,
    Equipment,
    Food,
    #[serde(rename = "ForceOpitonalGift")]
    ForceOpitonalGift,
    Formula,
    Gift,
    #[serde(rename = "HeadIcon")]
    HeadIcon,
    Material,
    Mission,
    #[serde(rename = "MuseumExhibit")]
    MuseumExhibit,
    #[serde(rename = "MuseumStuff")]
    MuseumStuff,
    #[serde(rename = "MusicAlbum")]
    MusicAlbum,
    #[serde(rename = "PhoneTheme")]
    PhoneTheme,
    Relic,
    #[serde(rename = "RelicRarityShowOnly")]
    RelicRarityShowOnly,
    #[serde(rename = "RelicSetShowOnly")]
    RelicSetShowOnly,
    #[serde(rename = "RogueMedal")]
    RogueMedal,
    #[serde(rename = "TravelBrochurePaster")]
    TravelBrochurePaster,
    Usable,
    Virtual,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Rarity {
    Normal,
    #[serde(rename = "NotNormal")]
    NotNormal,
    Rare,
    #[serde(rename = "SuperRare")]
    SuperRare,
    #[serde(rename = "VeryRare")]
    VeryRare,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReturnItemIdList {
    #[serde(rename = "ItemID")]
    pub item_id: i64,
    pub item_num: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SellType {
    Destroy,
    Sell,
    #[serde(rename = "UnSellable")]
    UnSellable,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UseMethod {
    #[serde(rename = "AutoConversionItem")]
    AutoConversionItem,
    #[serde(rename = "BPUnlock128")]
    BpUnlock128,
    #[serde(rename = "BPUnlock68")]
    BpUnlock68,
    #[serde(rename = "BPUpgradeFrom68To128")]
    BpUpgradeFrom68To128,
    #[serde(rename = "ExternalSystemFoodBenefit")]
    ExternalSystemFoodBenefit,
    #[serde(rename = "FixedRewardGift")]
    FixedRewardGift,
    #[serde(rename = "MonthlyCard")]
    MonthlyCard,
    #[serde(rename = "PlayerSelectedReward")]
    PlayerSelectedReward,
    #[serde(rename = "RandomRewardGift")]
    RandomRewardGift,
    Recipe,
    #[serde(rename = "TeamSpecificFoodBenefit")]
    TeamSpecificFoodBenefit,
    #[serde(rename = "TravelBrochurePasterUse")]
    TravelBrochurePasterUse,
    #[serde(rename = "TravelBrochureUse")]
    TravelBrochureUse,
    #[serde(rename = "TreasureMap")]
    TreasureMap,
    Unknown,
}
