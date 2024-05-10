use crate::{excel::{types::StageConfigElement, EXCEL}, game::globals, safe_unwrap_result};

use super::*;

// static BATTLE_LINEUP: [u32; 4] = [1309, 1308, 1307, 1315];

pub async fn on_start_cocoon_stage_cs_req(
    session: &PlayerSession,
    body: &StartCocoonStageCsReq,
) -> Result<()> {
    //let inventory = &session.player_info().inventory;
    let globalsb = &globals.borrow();

    let mut monster_wave_list = vec![];

    for i in 0..globalsb.monster_wave_list.len() {
        let wave = &globalsb.monster_wave_list[i];
        let level = globalsb.monster_levels[i];

        monster_wave_list.push(SceneMonsterWave {
            monster_list: wave.iter().map(|monster_id| SceneMonsterData {
                monster_id: *monster_id,
                ..Default::default()
            }).collect(),
            acpannfhach: Some(Kjfnknacfin { //scenemonsterwave
                level,
                ..Default::default()
            }),
            ..Default::default()
        })
    }

    let rsp = StartCocoonStageScRsp {
        retcode: 0,
        prop_entity_id: body.prop_entity_id,
        cocoon_id: body.cocoon_id,
        wave: body.wave,
        battle_info: Some(SceneBattleInfo {
            stage_id: globalsb.stage_id,
            logic_random_seed: 4444,
            //rounds_limit: 30,
            battle_id: 1,
            buff_list: vec![
                // BattleBuff {
                //     id: 130701,
                //     owner_index: 2,
                //     level: 1,
                //     wave_flag: 0xffffffff,
                //     dynamic_values: HashMap::from([(String::from("SkillIndex"), 0 as f32)]),
                //     ..Default::default()
                // },
            ],
            battle_avatar_list: session.player_info().inventory.create_battle_lineup(&session.player_info()),
            monster_wave_list,
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

pub async fn on_scene_enter_stage_cs_req(
    session: &PlayerSession,
    body: &SceneEnterStageCsReq,
) -> Result<()> {
    println!("{:#?}", body);
    let stage: &StageConfigElement = match EXCEL.stage_config.iter().find(|c| c.stage_id == body.event_id) {
        Some(v) => v,
        Option::None => return session.send(
            CMD_SCENE_ENTER_STAGE_SC_RSP,
            SceneEnterStageScRsp {
                retcode: 1,
                battle_info: Option::None,
            }
        ).await
    };

    session.send(
        CMD_SCENE_ENTER_STAGE_SC_RSP,
        SceneEnterStageScRsp {
            retcode: 0,
            battle_info: Some(SceneBattleInfo {
                logic_random_seed: 4444,
                stage_id: body.event_id,
                monster_wave_list: stage.monster_list.iter().map(|mon| SceneMonsterWave {
                    monster_list: vec![
                        SceneMonsterData {
                            monster_id: mon.monster0,
                            ..Default::default()
                        },
                        SceneMonsterData {
                            monster_id: mon.monster1,
                            ..Default::default()
                        },
                        SceneMonsterData {
                            monster_id: mon.monster2,
                            ..Default::default()
                        },
                        SceneMonsterData {
                            monster_id: mon.monster3,
                            ..Default::default()
                        },
                        SceneMonsterData {
                            monster_id: mon.monster4,
                            ..Default::default()
                        }
                    ],
                    acpannfhach: Some(Kjfnknacfin { //scenemonsterwave
                        level: stage.level,
                        ..Default::default()
                    }),
                    ..Default::default()
                }).collect(),
                battle_id: 2,
                battle_avatar_list: session.player_info().inventory.create_battle_lineup(&session.player_info()),
                ..Default::default()
            })
        }
    ).await
}

//TODO after found proto
// pub async fn on_start_challenge_cs_req(
//     session: &PlayerSession,
//     body: &StartChallengeCsReq,
// ) -> Result<()> {
//     let player_info = &mut session.player_info_mut();

//     let challenge = match EXCEL.challenge_maze_config.iter().find(|mc| mc.id == body.challenge_id) {
//         Some(v) => v,
//         Option::None => return session.send(
//             CMD_SCENE_ENTER_STAGE_SC_RSP,
//             StartChallengeScRsp {
//                 retcode: 1,
//                 ..Default::default()
//             }
//         ).await
//     };

//     session.send(
//         CMD_SCENE_ENTER_STAGE_SC_RSP,
//         StartChallengeScRsp {
//             retcode: 0,
//             scene: Some(load_scene(&session, challenge.map_entrance_id, true, Option::None, player_info).await.unwrap()),
//             lineup: Some(player_info.lineup.clone()),
//             ofljdjolifn: Option::None
//         }
//     ).await
// }