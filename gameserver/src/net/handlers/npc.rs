use crate::{excel::tools_res::{Position, PropState, GAME_RESOURCES}, safe_unwrap_result};

use super::*;

pub async fn on_bciendaonnn(
    session: &PlayerSession,
    body: &Bciendaonnn,
) -> Result<()> {
    println!("{:?}", body);

    session
        .send(
            CMD_GET_FIRST_TALK_BY_PERFORMANCE_NPC_SC_RSP,
            Bemeopenkpd {
                retcode: 0,
                apiaadfldbe: body.khjfgfhhchj.iter().map(|id| 
                    Hndfedalldc {
                        jmnkdpdjilg: *id,
                        apcaodelfcp: false
                    }
                ).collect()
            },
        )
        .await
}

//GetNpcTakenRewardCsReq
pub async fn on_get_npc_taken_reward_cs_req(
    session: &PlayerSession,
    body: &GetNpcTakenRewardCsReq,
) -> Result<()> {
    println!("{:?}", body);

    session
        .send(
            CMD_GET_NPC_TAKEN_REWARD_SC_RSP,
            GetNpcTakenRewardScRsp {
                retcode: 0,
                egeneneoadj: body.egeneneoadj,
                bekdcnobfeo: vec![]
            }
        ).await
}

//FinishFirstTalkByPerformanceNpcCsReq


//SubmitOrigamiItemCsReq
pub async fn on_submit_origami_item_cs_req(
    session: &PlayerSession,
    body: &SubmitOrigamiItemCsReq,
) -> Result<()> {
    println!("{:?}", body);
    session
        .send(
            CMD_SUBMIT_ORIGAMI_ITEM_SC_RSP,
            SubmitOrigamiItemScRsp {
                retcode: 0,
                gbjdobijaoi: body.gbjdobijaoi
            }
        ).await
}

//
pub async fn on_interact_prop_cs_req(
    session: &PlayerSession,
    body: &InteractPropCsReq
) -> Result<()> {
    let player_info = session.player_info();
    let entity_state_manager = &mut session.entity_state_manager_mut();
    let mut state = PropState::Open;

    println!("{:#?}", body);

    //TODO: refactor this mess

    if let Some(prop) = player_info.scene_prop_cache.get(&body.prop_entity_id) {
        state = prop.state.clone();
        // println!("{:#?}", prop);
        if let Some((_, interact)) = session.player_info().excel_manager.interact.iter().find(|(_, v)| v.interact_id == body.interact_id && v.src_state == prop.state) {
            // state = match &prop.prop_state_list {
            //     x if x.contains(&PropState::Destructed) => PropState::Destructed,
            //     x if x.contains(&PropState::ChestUsed) => PropState::ChestUsed,
            //     x if x.contains(&PropState::EventOpen) => PropState::EventOpen,
            //     _ => PropState::Open
            // };
            state = interact.target_state.clone();

            safe_unwrap_result!(session.send(
                CMD_SCENE_PLANE_EVENT_SC_NOTIFY, 
                Bkpebkeapjh { 
                    ckbbbokbdao: Some(ItemList {
                        item_list: interact.item_cost_list.iter().map(|i| Item {
                            item_id: i.item_id,
                            num: i.item_num,
                            ..Default::default()
                        }).collect()
                    }),
                    ..Default::default()
                }
            ).await);
        }

        let prop_position = Position {
            x: (prop.pos_x * 1000f64) as i32,
            y: (prop.pos_y * 1000f64) as i32,
            z: (prop.pos_z * 1000f64) as i32,
            rot_y: (prop.rot_y * 1000f64) as i32,
        };

        let mut refresh_entity = vec![
            SceneEntityRefreshInfo {
                glalelmdamm: Some(SceneEntityInfo {
                    inst_id: prop.id,
                    group_id: prop.group_id,
                    motion: Some(prop_position.to_motion()),
                    prop: Some(ScenePropInfo {
                        prop_id: prop.prop_id,
                        prop_state: state.clone() as u32,
                        ..Default::default()
                    }),
                    entity_id: body.prop_entity_id,
                    ..Default::default()
                }),
                ..Default::default()
            }
        ];
        
        if let Some(prop_info) = GAME_RESOURCES.maze_prop.get(&prop.prop_id) {
            // if puzzle unlock everything in prop's group
            println!("PROP INFORMED {:?}", prop_info);
            if (prop_info.prop_type == "PROP_MAZE_PUZZLE" || prop_info.prop_type == "PROP_MAZE_JIGSAW") && (state == PropState::Open || state == PropState::Closed) {
                refresh_entity.append(&mut player_info.scene_prop_cache
                    .iter()
                    .filter(|(eid, p)| p.group_id == prop.group_id && **eid != body.prop_entity_id)
                    .map(|(eid, prop)| {
                        println!("KUMALA eid({:?}) {:?}", eid, prop_info);
                        let prop_position = Position {
                            x: (prop.pos_x * 1000f64) as i32,
                            y: (prop.pos_y * 1000f64) as i32,
                            z: (prop.pos_z * 1000f64) as i32,
                            rot_y: (prop.rot_y * 1000f64) as i32,
                        };

                        let state = if let Some(prop_info) = GAME_RESOURCES.maze_prop.get(&prop.prop_id) {
                            match prop_info.prop_type.as_str() {
                                "PROP_TREASURE_CHEST" => PropState::ChestClosed,
                                "PROP_MAZE_PUZZLE" => prop.state.clone(),
                                _ => PropState::Open
                            }
                        } else {
                            PropState::Open
                        };

                        entity_state_manager.set_entity_state(player_info.position.entry_id, *eid, state.clone());

                        SceneEntityRefreshInfo {
                            glalelmdamm: Some(SceneEntityInfo {
                                inst_id: prop.id,
                                group_id: prop.group_id,
                                motion: Some(prop_position.to_motion()),
                                prop: Some(ScenePropInfo {
                                    prop_id: prop.prop_id,
                                    prop_state: state.clone() as u32,
                                    ..Default::default()
                                }),
                                entity_id: body.prop_entity_id,
                                ..Default::default()
                            }),
                            ..Default::default()
                        }
                    })
                    .collect()
                );   
            }
        }

        //if console open all bridges in its group
        if prop.clone().init_level_graph.map(|j| j.contains("Common_Console")) == Some(true) {
            refresh_entity.append(&mut player_info.scene_prop_cache
                .iter()
                .filter(|(eid, p)| p.group_id == prop.group_id && **eid != body.prop_entity_id && GAME_RESOURCES.maze_prop.get(&p.prop_id).map(|i| i.prop_type == "PROP_PLATFORM") == Some(true))
                .map(|(eid, prop)| {
                    let prop_position = Position {
                        x: (prop.pos_x * 1000f64) as i32,
                        y: (prop.pos_y * 1000f64) as i32,
                        z: (prop.pos_z * 1000f64) as i32,
                        rot_y: (prop.rot_y * 1000f64) as i32,
                    };

                    entity_state_manager.set_entity_state(player_info.position.entry_id, *eid, state.clone());

                    SceneEntityRefreshInfo {
                        glalelmdamm: Some(SceneEntityInfo {
                            inst_id: prop.id,
                            group_id: prop.group_id,
                            motion: Some(prop_position.to_motion()),
                            prop: Some(ScenePropInfo {
                                prop_id: prop.prop_id,
                                prop_state: state.clone() as u32,
                                ..Default::default()
                            }),
                            entity_id: body.prop_entity_id,
                            ..Default::default()
                        }),
                        ..Default::default()
                    }
                })
                .collect()
            );  
        }

        entity_state_manager.set_entity_state(player_info.position.entry_id, body.prop_entity_id, state.clone());

        safe_unwrap_result!(session.send(
            CMD_SCENE_GROUP_REFRESH_SC_NOTIFY, 
            SceneGroupRefreshScNotify {
                group_refresh_info: vec![
                    SceneGroupRefreshInfo {
                        group_id: prop.group_id,
                        state: state.clone() as u32,
                        refresh_entity,
                        ..Default::default()
                    }
                ]
            }
        ).await);
    }

    if state == PropState::ChestUsed {
        safe_unwrap_result!(session.send(
            CMD_SCENE_PLANE_EVENT_SC_NOTIFY, 
            Bkpebkeapjh { 
                ckbbbokbdao: Some(ItemList {
                    item_list: vec![
                        Item {
                            item_id: 102,
                            num: 2137,
                            ..Default::default()
                        }
                    ]
                }),
                ..Default::default()
            }
        ).await);
    }

    session
        .send(
            CMD_INTERACT_PROP_SC_RSP,
            InteractPropScRsp {
                retcode: 0,
                prop_entity_id: body.prop_entity_id,
                prop_state: state as u32
            }
        ).await
}

//GetMainMissionCustomValueCsReq
pub async fn on_get_main_mission_custom_value_cs_req(
    session: &PlayerSession,
    body: &GetMainMissionCustomValueCsReq
) -> Result<()> {//println!("{:?}", body);
    session.send(
        CMD_GET_MAIN_MISSION_CUSTOM_VALUE_SC_RSP,
        GetMainMissionCustomValueScRsp {
            retcode: 0,
            mmmedgnoljo: body.sub_mission_id_list.iter().map(|a| Ebeeijpilmi {
                id: *a,
                status: 2,
                miadakiaoln:vec![]
            }).collect()
        }
    ).await
}

pub async fn on_get_first_talk_npc_cs_req(
    session: &PlayerSession,
    body: &GetFirstTalkNpcCsReq
) -> Result<()> {
    session.send(
        CMD_GET_FIRST_TALK_NPC_SC_RSP,
        Abojckcendm {
            retcode: 0,
            apiaadfldbe: body.aammpfgpknj.iter().map(|q| Oijcllopbih {
                ihbalhicnej: *q,
                ..Default::default()
            }).collect()
        }
    ).await
}

//GroupStateChangeCsReq
pub async fn on_group_state_change_cs_req(
    session: &PlayerSession,
    body: &GroupStateChangeCsReq
) -> Result<()> {
    println!("{:#?}", body);
    session.send(
        CMD_GROUP_STATE_CHANGE_SC_RSP, 
        GroupStateChangeScRsp {
            group_state_info: body.group_state_info.clone(),
            retcode: 0
        }
    ).await
}