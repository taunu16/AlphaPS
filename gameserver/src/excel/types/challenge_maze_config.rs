use serde::{Serialize, Deserialize};

pub type ChallengeMazeConfig = Vec<ChallengeMazeConfigElement>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChallengeMazeConfigElement {
    pub damage_type1: Vec<DamageType>,
    pub damage_type2: Vec<DamageType>,
    #[serde(rename = "ChallengeTargetID")]
    pub challenge_target_id: Vec<i64>,
    #[serde(rename = "MonsterID1")]
    pub monster_id1: Vec<Option<serde_json::Value>>,
    #[serde(rename = "MonsterID2")]
    pub monster_id2: Vec<Option<serde_json::Value>>,
    pub config_list1: Vec<i64>,
    #[serde(rename = "NpcMonsterIDList1")]
    pub npc_monster_id_list1: Vec<i64>,
    #[serde(rename = "EventIDList1")]
    pub event_id_list1: Vec<i64>,
    pub config_list2: Vec<i64>,
    #[serde(rename = "NpcMonsterIDList2")]
    pub npc_monster_id_list2: Vec<i64>,
    #[serde(rename = "EventIDList2")]
    pub event_id_list2: Vec<i64>,
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(rename = "GroupID")]
    pub group_id: i64,
    #[serde(rename = "MapEntranceID")]
    pub map_entrance_id: u32,
    pub pre_level: i64,
    #[serde(rename = "PreChallengeMazeID")]
    pub pre_challenge_maze_id: i64,
    pub floor: i64,
    #[serde(rename = "RewardID")]
    pub reward_id: i64,
    pub stage_num: i64,
    pub challenge_count_down: i64,
    #[serde(rename = "MazeGroupID1")]
    pub maze_group_id1: i64,
    #[serde(rename = "MazeGroupID2")]
    pub maze_group_id2: i64,
    #[serde(rename = "MazeBuffID")]
    pub maze_buff_id: i64,
    pub name: Name,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DamageType {
    Fire,
    Ice,
    Imaginary,
    Physical,
    Quantum,
    Thunder,
    Wind,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Name {
    pub hash: i64,
}