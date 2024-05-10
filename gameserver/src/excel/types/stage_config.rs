use serde::{Serialize, Deserialize};
use std::collections::HashMap;

pub type StageConfig = Vec<StageConfigElement>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StageConfigElement {
    pub level_graph_path: LevelGraphPath,
    pub stage_ability_config: Vec<String>,
    pub sub_level_graphs: Vec<HashMap<String, Option<String>>>,
    pub stage_config_data: Vec<StageConfigDatum>,
    pub monster_list: Vec<MonsterList>,
    pub level_lose_condition: Vec<LevelLoseCondition>,
    pub level_win_condition: Vec<LevelWinCondition>,
    pub trial_avatar_list: Vec<i64>,
    #[serde(rename = "StageID")]
    pub stage_id: u32,
    pub stage_type: StageType,
    pub hard_level_group: i64,
    pub level: u32,
    pub elite_group: i64,
    pub battle_scoring_group: i64,
    pub monster_warning_ratio: f64,
    pub stage_name: StageName,
    pub forbid_auto_battle: bool,
    pub release: bool,
    pub forbid_exit_battle: bool,
    pub reset_battle_speed: bool,
    pub processed_sub_level_graph_list: ProcessedSubLevelGraphList,
    pub processed_template_variables: ProcessedTemplateVariables,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LevelGraphPath {
    #[serde(rename = "Config/Level/StageCommonTemplate.json")]
    ConfigLevelStageCommonTemplateJson,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LevelLoseCondition {
    #[serde(rename = "[CDT_WaitCustomString:Level_SpecialLose]")]
    CdtWaitCustomStringLevelSpecialLose,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LevelWinCondition {
    #[serde(rename = "[CDT_CharactorDie:1206]")]
    CdtCharactorDie1206,
    #[serde(rename = "[CDT_MonsterDie:100301001]")]
    CdtMonsterDie100301001,
    #[serde(rename = "[CDT_MonsterDie:100301004]")]
    CdtMonsterDie100301004,
    #[serde(rename = "[CDT_MonsterDie:100401001]")]
    CdtMonsterDie100401001,
    #[serde(rename = "[CDT_MonsterDie:100401002]")]
    CdtMonsterDie100401002,
    #[serde(rename = "[CDT_MonsterDie:100402002]")]
    CdtMonsterDie100402002,
    #[serde(rename = "[CDT_MonsterDie:100403001]")]
    CdtMonsterDie100403001,
    #[serde(rename = "[CDT_MonsterDie:101301002]")]
    CdtMonsterDie101301002,
    #[serde(rename = "[CDT_MonsterDie:101301003]")]
    CdtMonsterDie101301003,
    #[serde(rename = "[CDT_MonsterDie:101301008]")]
    CdtMonsterDie101301008,
    #[serde(rename = "[CDT_MonsterDie:101302006]")]
    CdtMonsterDie101302006,
    #[serde(rename = "[CDT_MonsterDie:101401001]")]
    CdtMonsterDie101401001,
    #[serde(rename = "[CDT_MonsterDie:101401002]")]
    CdtMonsterDie101401002,
    #[serde(rename = "[CDT_MonsterDie:200401001]")]
    CdtMonsterDie200401001,
    #[serde(rename = "[CDT_MonsterDie:200401003]")]
    CdtMonsterDie200401003,
    #[serde(rename = "[CDT_MonsterDie:202302005]")]
    CdtMonsterDie202302005,
    #[serde(rename = "[CDT_MonsterDie:202303001]")]
    CdtMonsterDie202303001,
    #[serde(rename = "[CDT_MonsterDie:202303002]")]
    CdtMonsterDie202303002,
    #[serde(rename = "[CDT_MonsterDie:202303006]")]
    CdtMonsterDie202303006,
    #[serde(rename = "[CDT_MonsterDie:300401001]")]
    CdtMonsterDie300401001,
    #[serde(rename = "[CDT_MonsterDie:800303003]")]
    CdtMonsterDie800303003,
    #[serde(rename = "[CDT_MonsterDie:801301003]")]
    CdtMonsterDie801301003,
    #[serde(rename = "[CDT_MonsterDie:8013012]")]
    CdtMonsterDie8013012,
    #[serde(rename = "[CDT_MonsterDie:8015021]")]
    CdtMonsterDie8015021,
    #[serde(rename = "[CDT_MonsterHurt:100302001:0.2]")]
    CdtMonsterHurt10030200102,
    #[serde(rename = "[CDT_MonsterHurt:8002040:0.3]")]
    CdtMonsterHurt800204003,
    #[serde(rename = "[CDT_WaitCustomString:Cocolia_Skill11]")]
    CdtWaitCustomStringCocoliaSkill11,
    #[serde(rename = "[CDT_WaitCustomString:Level_BattleCollege_SpeicalWin]")]
    CdtWaitCustomStringLevelBattleCollegeSpeicalWin,
    #[serde(rename = "[CDT_WaitCustomString:Level_SpecialWin]")]
    CdtWaitCustomStringLevelSpecialWin,
    #[serde(rename = "[CDT_WaitCustomString:Level_SpecialWin_Less30]")]
    CdtWaitCustomStringLevelSpecialWinLess30,
    #[serde(rename = "[CDT_WaitCustomString:Level_SpecialWin_Rogue_S1]")]
    CdtWaitCustomStringLevelSpecialWinRogueS1,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MonsterList {
    pub monster0: u32,
    pub monster1: u32,
    pub monster2: u32,
    pub monster3: u32,
    pub monster4: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessedSubLevelGraphList {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessedTemplateVariables {
    #[serde(rename = "_Wave")]
    pub wave: i64,
    #[serde(rename = "_IsEliteBattle")]
    pub is_elite_battle: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct StageConfigDatum {
    pub lfkffcjnfkn: Lfkffcjnfkn,
    pub epbooffckpj: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Lfkffcjnfkn {
    #[serde(rename = "_BattleCondition")]
    BattleCondition,
    #[serde(rename = "_BattleTarget")]
    BattleTarget,
    #[serde(rename = "_BGM")]
    Bgm,
    #[serde(rename = "_BindingMazeBuff")]
    BindingMazeBuff,
    #[serde(rename = "_CloseBattleStartDialog")]
    CloseBattleStartDialog,
    #[serde(rename = "_CreateBattleActionEvent")]
    CreateBattleActionEvent,
    #[serde(rename = "_CreateBattleEvent")]
    CreateBattleEvent,
    #[serde(rename = "_DeferCreateTrialPlayer")]
    DeferCreateTrialPlayer,
    #[serde(rename = "_EnsureTeamAliveKey")]
    EnsureTeamAliveKey,
    #[serde(rename = "_IsEliteBattle")]
    IsEliteBattle,
    #[serde(rename = "_SpecialBattleStartCamera")]
    SpecialBattleStartCamera,
    #[serde(rename = "_StageBannedAvatarID")]
    StageBannedAvatarId,
    #[serde(rename = "_StageInfiniteGroup")]
    StageInfiniteGroup,
    #[serde(rename = "_Wave")]
    Wave,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StageName {
    pub hash: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StageType {
    #[serde(rename = "AetherDivide")]
    AetherDivide,
    #[serde(rename = "BattleCollege")]
    BattleCollege,
    #[serde(rename = "BoxingClub")]
    BoxingClub,
    Challenge,
    #[serde(rename = "ClockParkActivity")]
    ClockParkActivity,
    Cocoon,
    #[serde(rename = "EvolveBuildActivity")]
    EvolveBuildActivity,
    #[serde(rename = "FantasticStory")]
    FantasticStory,
    #[serde(rename = "FarmElement")]
    FarmElement,
    #[serde(rename = "FeverTimeActivity")]
    FeverTimeActivity,
    #[serde(rename = "FightActivity")]
    FightActivity,
    Heliobus,
    Mainline,
    #[serde(rename = "PunkLord")]
    PunkLord,
    #[serde(rename = "RogueChallengeActivity")]
    RogueChallengeActivity,
    #[serde(rename = "RogueEndlessActivity")]
    RogueEndlessActivity,
    #[serde(rename = "RogueRelic")]
    RogueRelic,
    #[serde(rename = "StarFightActivity")]
    StarFightActivity,
    #[serde(rename = "StrongChallengeActivity")]
    StrongChallengeActivity,
    #[serde(rename = "TelevisionActivity")]
    TelevisionActivity,
    #[serde(rename = "TreasureDungeon")]
    TreasureDungeon,
    Trial,
    #[serde(rename = "VerseSimulation")]
    VerseSimulation,
}
