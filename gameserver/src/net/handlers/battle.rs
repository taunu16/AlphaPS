use std::collections::HashMap;

use crate::game::globals;

use super::*;

// static BATTLE_LINEUP: [u32; 4] = [1309, 1308, 1307, 1315];

pub async fn on_start_cocoon_stage_cs_req(
    session: &PlayerSession,
    body: &StartCocoonStageCsReq,
) -> Result<()> {
    let inventory = &session.player_info().inventory;

    let rsp = StartCocoonStageScRsp {
        retcode: 0,
        prop_entity_id: body.prop_entity_id,
        cocoon_id: body.cocoon_id,
        wave: body.wave,
        battle_info: Some(SceneBattleInfo {
            stage_id: 201012311,
            logic_random_seed: 4444,
            battle_id: 1,
            buff_list: vec![
                BattleBuff {
                    id: 130701,
                    owner_index: 2,
                    level: 1,
                    wave_flag: 0xffffffff,
                    dynamic_values: HashMap::from([(String::from("SkillIndex"), 0 as f32)]),
                    ..Default::default()
                },
            ],
            battle_avatar_list: session.player_info().inventory.create_battle_lineup(&session.player_info()),
            monster_wave_list: vec![SceneMonsterWave {
                monster_list: vec![SceneMonsterParam {
                    monster_id: 3013010,
                    ..Default::default()
                }],
                ejahmdkklbn: Some(Holldlkceof { 
                    level: 80,
                    ..Default::default()
                }),
                ..Default::default()
            }],
            ..Default::default()
        }),
    };

    session.send(CMD_START_COCOON_STAGE_SC_RSP, rsp).await
}

pub async fn on_pve_battle_result_cs_req(
    session: &PlayerSession,
    body: &PveBattleResultCsReq,
) -> Result<()> {
    session
        .send(
            CMD_P_V_E_BATTLE_RESULT_SC_RSP,
            PveBattleResultScRsp {
                retcode: 0,
                end_status: body.end_status,
                battle_id: body.battle_id,
                ..Default::default()
            },
        )
        .await
}
