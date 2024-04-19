//this is stolen from ami too
use std::collections::HashMap;
use lazy_static::lazy_static;
use proto::{MotionInfo, Vector};
use serde::{Deserialize, Serialize};
use serde_json::Value;

const RESOURCES: &[u8] = include_bytes!("./data/resources.json");

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelGroup {
    #[serde(rename = "GroupGUID")]
    pub group_guid: String,
    #[serde(default)]
    #[serde(rename = "LoadSide")]
    pub load_side: LoadSide,
    #[serde(default)]
    #[serde(rename = "LoadOnInitial")]
    pub load_on_initial: bool,

    #[serde(default)]
    #[serde(rename = "AnchorList")]
    pub anchor_list: Vec<LevelAnchor>,
    #[serde(default)]
    #[serde(rename = "MonsterList")]
    pub monster_list: Vec<LevelMonster>,
    #[serde(default)]
    #[serde(rename = "PropList")]
    pub prop_list: Vec<LevelProp>,
    #[serde(default)]
    #[serde(rename = "NPCList")]
    pub npc_list: Vec<LevelNPC>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelProp {
    #[serde(rename = "ID")]
    pub id: u32, // need
    #[serde(default)]
    #[serde(rename = "Category")]
    pub category: String,
    #[serde(default)]
    #[serde(rename = "GroupName")]
    pub group_name: String,
    #[serde(default)]
    #[serde(rename = "LoadSide")]
    pub load_side: Option<LoadSide>, // need
    #[serde(default)]
    #[serde(rename = "PosX")]
    pub pos_x: f64, // n
    #[serde(default)]
    #[serde(rename = "PosY")]
    pub pos_y: f64, // n
    #[serde(default)]
    #[serde(rename = "PosZ")]
    pub pos_z: f64, // n
    #[serde(default)]
    #[serde(rename = "RotY")]
    pub rot_y: f64, // n
    #[serde(rename = "PropID")]
    pub prop_id: u32, // n
    #[serde(rename = "AnchorID")]
    pub anchor_id: Option<u32>, // n
    #[serde(rename = "AnchorGroupID")]
    pub anchor_group_id: Option<u32>, // n
    #[serde(rename = "MappingInfoID")]
    pub mapping_info_id: Option<u32>, // n

    #[serde(rename = "InitLevelGraph")]
    pub init_level_graph: Option<String>,

    #[serde(default)]
    #[serde(rename = "State")]
    pub state: PropState,

    #[serde(default)]
    pub prop_state_list: Vec<PropState>,
    #[serde(default)]
    pub group_id: u32,
    #[serde(rename = "IsDelete")]
    #[serde(default)]
    pub is_delete: bool, // n
    #[serde(rename = "IsClientOnly")]
    #[serde(default)]
    pub client_only: bool, // n

    // #[serde(default)]
    // pub is_door: bool,
    // #[serde(default)]
    // pub is_chest: bool,
    #[serde(default)]
    pub __test_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelAnchor {
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(default)]
    #[serde(rename = "PosX")]
    pub pos_x: f64,
    #[serde(default)]
    #[serde(rename = "PosY")]
    pub pos_y: f64,
    #[serde(default)]
    #[serde(rename = "PosZ")]
    pub pos_z: f64,
    #[serde(default)]
    #[serde(rename = "RotY")]
    pub rot_y: f64,
    #[serde(default)]
    pub group_id: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelNPC {
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(default)]
    #[serde(rename = "PosX")]
    pub pos_x: f64,
    #[serde(default)]
    #[serde(rename = "PosY")]
    pub pos_y: f64,
    #[serde(default)]
    #[serde(rename = "PosZ")]
    pub pos_z: f64,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "RotY")]
    pub rot_y: f64,
    #[serde(rename = "NPCID")]
    pub npcid: u32,
    #[serde(default)]
    pub group_id: u32,
    #[serde(rename = "IsDelete")]
    #[serde(default)]
    pub is_delete: bool,
    #[serde(rename = "IsClientOnly")]
    #[serde(default)]
    pub client_only: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelMonster {
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(default)]
    #[serde(rename = "RotY")]
    pub rot_y: f64,
    #[serde(default)]
    #[serde(rename = "PosX")]
    pub pos_x: f64,
    #[serde(default)]
    #[serde(rename = "PosY")]
    pub pos_y: f64,
    #[serde(default)]
    #[serde(rename = "PosZ")]
    pub pos_z: f64,
    #[serde(rename = "NPCMonsterID")]
    pub npcmonster_id: u32,
    #[serde(default)]
    #[serde(rename = "EventID")]
    pub event_id: u32,
    #[serde(default)]
    pub group_id: u32,
    #[serde(rename = "IsDelete")]
    #[serde(default)]
    pub is_delete: bool,
    #[serde(rename = "IsClientOnly")]
    #[serde(default)]
    pub client_only: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
pub enum LoadSide {
    Client = 0,
    Server = 1,
    Unk = 2,
}

impl Default for LoadSide {
    fn default() -> Self {
        Self::Client
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelFloor {
    #[serde(rename = "FloorID")]
    pub floor_id: u32,
    #[serde(rename = "FloorName")]
    pub floor_name: String,
    #[serde(rename = "StartGroupID")]
    pub start_group_id: u32,
    #[serde(rename = "StartAnchorID")]
    pub start_anchor_id: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapEntrance {
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(default)]
    #[serde(rename = "EntranceType")]
    pub entrance_type: PlaneType,
    #[serde(rename = "PlaneID")]
    pub plane_id: u32,
    #[serde(rename = "FloorID")]
    pub floor_id: u32,
    #[serde(rename = "BeginMainMissionList")]
    pub begin_main_mission_list: Vec<Value>,
    #[serde(rename = "FinishMainMissionList")]
    pub finish_main_mission_list: Vec<Value>,
    #[serde(rename = "FinishSubMissionList")]
    pub finish_sub_mission_list: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MazePlane {
    #[serde(rename = "PlaneID")]
    pub plane_id: u32,
    #[serde(rename = "PlaneType")]
    pub plane_type: PlaneType,
    #[serde(rename = "SubType")]
    pub sub_type: u32,
    #[serde(rename = "MazePoolType")]
    pub maze_pool_type: u32,
    #[serde(rename = "WorldID")]
    pub world_id: u32,
    #[serde(rename = "StartFloorID")]
    pub start_floor_id: u32,
    #[serde(rename = "FloorIDList")]
    pub floor_idlist: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
pub enum PlaneType {
    Unknown = 0,
    Maze = 2,
    Train = 3,
    Challenge = 4,
    Rogue = 5,
    Raid = 6,
    AetherDivide = 7,
    TrialActivity = 8,
    #[serde(other)]
    Town = 1,
}

impl Default for PlaneType {
    fn default() -> Self {
        Self::Maze
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MazeProp {
    #[serde(rename = "ID")]
    pub id: i64,
    #[serde(rename = "PropType")]
    pub prop_type: String,
    #[serde(rename = "PropStateList")]
    pub prop_state_list: Vec<PropState>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PropState {
    Closed = 0,
    Open = 1,
    Locked = 2,
    BridgeState1 = 3,
    BridgeState2 = 4,
    BridgeState3 = 5,
    BridgeState4 = 6,
    CheckPointDisable = 7,
    CheckPointEnable = 8,
    TriggerDisable = 9,
    TriggerEnable = 10,
    ChestLocked = 11,
    ChestClosed = 12,
    ChestUsed = 13,
    Elevator1 = 14,
    Elevator2 = 15,
    Elevator3 = 16,
    WaitActive = 17,
    EventClose = 18,
    EventOpen = 19,
    Hidden = 20,
    TeleportGate0 = 21,
    TeleportGate1 = 22,
    TeleportGate2 = 23,
    TeleportGate3 = 24,
    Destructed = 25,
    CustomState01 = 101,
    CustomState02 = 102,
    CustomState03 = 103,
    CustomState04 = 104,
    CustomState05 = 105,
    CustomState06 = 106,
    CustomState07 = 107,
    CustomState08 = 108,
    CustomState09 = 109,
}

impl Default for PropState {
    fn default() -> Self {
        PropState::Closed
    }
}

pub type IntMap<T> = HashMap<u32, T>;
pub type StringMap<T> = HashMap<String, T>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimpleLevelGroup {
    pub teleports: IntMap<LevelProp>,
    pub group_items: IntMap<LevelGroupItem>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LevelGroupItem {
    pub props: Vec<LevelProp>,
    pub npcs: Vec<LevelNPC>,
    pub monsters: Vec<LevelMonster>,
    pub anchors: HashMap<u32, LevelAnchor>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameResources {
    pub map_entrance: IntMap<MapEntrance>,
    /// Key is P{PLANE_ID}_F{FLOOR_ID}
    pub level_group: StringMap<SimpleLevelGroup>,
    pub maze_prop: IntMap<MazeProp>,
    pub maze_plane: IntMap<MazePlane>,
    pub level_floor: StringMap<LevelFloor>,
}

impl GameResources {
    pub fn new() -> Self {
        // let str = std::fs::read_to_string("./resources.json").unwrap();
        // let res: Self = serde_json::from_str(&str).unwrap();
        // res

        serde_json::from_slice(RESOURCES).unwrap()
    }

    pub async fn get_level_group(
        &self,
        entry_id: u32,
    ) -> Option<(SimpleLevelGroup, MapEntrance, Option<MazePlane>)> {
        let resources = self;
        let enterance = resources.map_entrance.get(&entry_id);

        if let Some(enterance) = enterance {
            let plane = resources.maze_plane.get(&enterance.plane_id);
            if let Some(level) = resources
                .level_group
                .get(&format!("P{}_F{}", enterance.plane_id, enterance.floor_id))
            {
                // TODO: use reference somehow, not cloning
                return Some((level.clone(), enterance.clone(), plane.cloned()));
            };
        }

        None
    }

    pub fn get_custom_teleport(teleport_id: &u32) -> Option<MotionInfo> {
        match teleport_id {
            1030402 => Some(MotionInfo {
                pos: Some(Vector {
                    x: 1759,
                    y: 16899,
                    z: -498092
                }),
                rot: Some(Vector {
                    x: 0,
                    y: 360000,
                    z: 0
                })
            }),
            1030403 => Some(MotionInfo {
                pos: Some(Vector {
                    x: 79728,
                    y: 14612,
                    z: -482211
                }),
                rot: Some(Vector {
                    x: 0,
                    y: 270000,
                    z: 0
                })
            }),
            _ => None
        }
    }
}

lazy_static! {
    pub static ref GAME_RESOURCES: GameResources = {
        GameResources::new()
    };
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub rot_y: i32,
}

impl Position {
    pub fn is_empty(&self) -> bool {
        return self.x == 0 && self.y == 0 && self.z == 0;
    }

    pub fn to_motion(&self) -> MotionInfo {
        MotionInfo {
            // rot
            rot: Some(Vector {
                x: 0,
                y: self.rot_y,
                z: 0,
            }),
            // pos
            pos: Some(Vector {
                x: self.x,
                y: self.y,
                z: self.z,
            }),
        }
    }
}