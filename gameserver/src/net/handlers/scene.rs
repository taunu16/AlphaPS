use crate::{excel::tools_res::*, find_by_id, safe_unwrap_result};

use super::*;

const TELEPORTS: [u32; 368] = [2215, 2206, 2219, 1000101, 1000102, 2258, 2264, 5002, 1010101, 1010102, 1010103, 1010201, 1010202, 1010203, 1020101, 1020102, 1020103, 5004, 1020202, 1020201, 1020203, 1020204, 1030101, 1030102, 1030103, 1030104, 1030105, 1030106, 1030301, 1030201, 2231, 5005, 2000101, 2000102, 1101, 2000201, 2000202, 2000203, 2000204, 1201, 1004, 2000301, 2000302, 2000303, 1301, 1005, 2368, 2000401, 2000402, 2000403, 2000404, 1304, 2368, 2010102, 2010103, 1006, 1001, 1107, 2011101, 2011102, 2011103, 1007, 1002, 1010, 1112, 2012101, 2012102, 2012103, 2012105, 1105, 1003, 1408, 1403, 2012201, 2012202, 2012203, 1008, 1102, 2012301, 2012302, 2012303, 1009, 1401, 2013101, 2013102, 2013103, 1405, 1104, 1202, 2013201, 2013204, 2013203, 2013202, 1203, 2013205, 1103, 2013205, 1404, 2013205, 1106, 2344, 2102, 2013401, 2013402, 1302, 1204, 2013501, 2013502, 2013601, 2347, 2021101, 2021102, 2021103, 2021104, 1402, 1409, 1011, 1108, 1402, 1205, 1406, 2021201, 2021202, 2021203, 2021204, 1206, 1109, 2102, 2102, 2102, 2102, 1012, 2022101, 2022102, 2022103, 2022104, 1110, 1013, 2022201, 2022202, 2022203, 2022204, 2253, 2022301, 2022302, 2022303, 1114, 2022304, 1021, 2308, 2310, 2311, 2312, 2313, 2313, 1208, 2346, 2313, 2023101, 2023102, 2023103, 2023104, 1113, 1017, 1207, 2023201, 2023202, 2023203, 1018, 1303, 1111, 1016, 1019, 2397, 2031101, 2031102, 2031103, 2031104, 2031105, 1115, 1209, 1015, 2031201, 2031202, 2031203, 1014, 2031301, 2031302, 2031303, 2031304, 1116, 1020, 2032101, 2032102, 2032103, 2032104, 1117, 2032201, 2032202, 2032203, 2022101, 2022102, 2022103, 2022104, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2022201, 2022202, 2022203, 2022204, 2102, 2102, 2102, 2102, 2022201, 2022202, 2022203, 2022204, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2022101, 2022102, 2022103, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2314, 2315, 2316, 2349, 2320, 2318, 2319, 2317, 2350, 2321, 2322, 2351, 2323, 2324, 2325, 2352, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2102, 2219, 2220];

macro_rules! generate_calyx {
    ($pos:expr) => {
    SceneEntityInfo {
                group_id: 19,
                inst_id: 300001,
                entity_id: 228,
                prop: Some(ScenePropInfo {
                    prop_id: 808,
                    prop_state: 1,
                    ..Default::default()
                }),
                motion: Some(MotionInfo {
                    pos: Some($pos),
                    rot: Some(Vector {
                        z: 4480,
                        y: 19364,
                        x: -570,
                    }),
                }),
                ..Default::default()
            }
    };
    ($pos:expr, $grid:expr) => {
        SceneEntityInfo {
                group_id: $grid,
                inst_id: 300001,
                entity_id: 228,
                prop: Some(ScenePropInfo {
                    prop_id: 808,
                    prop_state: 1,
                    ..Default::default()
                }),
                motion: Some(MotionInfo {
                    pos: Some($pos),
                    rot: Some(Vector {
                        z: 4480,
                        y: 19364,
                        x: -570,
                    }),
                }),
                ..Default::default()
            }
    };
}

pub async fn on_get_cur_scene_info_cs_req(
    session: &PlayerSession,
    _body: &GetCurSceneInfoCsReq,
) -> Result<()> {
    session
        .send(
            CMD_GET_CUR_SCENE_INFO_SC_RSP,
            GetCurSceneInfoScRsp {
                retcode: 0,
                scene: Some(SceneInfo {
                    plane_id: 20101,
                    floor_id: 20101001,
                    entry_id: 2010101,
                    game_mode_type: 2,
                    lpehniejlmg: TELEPORTS.to_vec(),
                    phicefeaigb: TELEPORTS.to_vec(),
                    chhmmbdhjpg: vec![
                        Dhkacjhaoid {
                            state: 1,
                            group_id: 0,
                            entity_list: vec![SceneEntityInfo {
                                group_id: 0,
                                inst_id: 0,
                                entity_id: 0,
                                actor: Some(SceneActorInfo {
                                    avatar_type: 3,
                                    base_avatar_id: 1309,
                                    map_layer: 2,
                                    uid: 2137,
                                }),
                                motion: Some(MotionInfo {
                                    pos: Some(Vector {
                                        z: 4480,
                                        y: 19364,
                                        x: -550,
                                    }),
                                    rot: Some(Vector {
                                        z: 4480,
                                        y: 19364,
                                        x: -550,
                                    }),
                                }),
                                ..Default::default()
                            }],
                        },
                        Dhkacjhaoid {
                            state: 1,
                            group_id: 19,
                            entity_list: vec![
                                generate_calyx!(Vector {
                                    z: 4480,
                                    y: 19364,
                                    x: -570,
                                })
                            ]
                        }
                    ],
                    ..Default::default()
                }),
            },
        )
        .await
}


/*
pub async fn on_get_scene_map_info_cs_req(
    session: &PlayerSession,
    _body: &GetSceneMapInfoCsReq,
) -> Result<()> {
    let mut lightened_sections = vec![];

    for i in 0..300 {
        lightened_sections.push(i);
    }

    session
        .send(
            CMD_GET_SCENE_MAP_INFO_SC_RSP,
            Cegeebldbke {
                retcode: 0,
                entry_id: _body.dmkkkfnkofh[0],
                cgkfbhoadpc: vec![],
                dcbdhkkkpgd: vec![],
                ojlnmnehgai: lightened_sections.clone(),
                phicefeaigb: TELEPORTS.to_vec(),
                mhefdgcamjl: vec![
                    Fjniajephmj {
                        retcode: 0,
                        kjlbpaefaff: _body.entry_id,
                        entry_id: _body.dmkkkfnkofh[0],
                        ojlnmnehgai: lightened_sections,
                        phicefeaigb: TELEPORTS.to_vec(),
                        ..Default::default()
                    }
                ],
                pmolfbcbfpe: vec![],
                kjlbpaefaff: _body.entry_id,
                ..Default::default()
            },
        )
        .await
}


pub async fn on_lckgkdehclb( //ENTER_SCENE_CS_Req
    session: &PlayerSession,
    _body: &Lckgkdehclb,
) -> Result<()> {println!("{:?}", _body);
    safe_unwrap_result!(session
        .send(
            CMD_ENTER_SCENE_BY_SERVER_SC_NOTIFY,
            log_through(EnterSceneByServerScNotify {
                lineup: Some(session.player_info().lineup.clone()),
                scene: Some(SceneInfo {
                    game_mode_type: 2,
                    entry_id: _body.entry_id,
                    plane_id: (_body.entry_id as f32 / 100f32).floor() as u32,
                    floor_id: ((_body.entry_id as f32 / 100f32).floor() * 1000f32).floor() as u32 + 1u32,
                    ..Default::default()
                }),
                bpodijpdnnk: Ffnhcbelgpd::EnterSceneReasonNone.into()
            })
        )
        .await);

    session
        .send(
            CMD_ENTER_SCENE_SC_RSP,
            Mmnkgmcafeh {
                retcode: 0,
                ..Default::default()
            },
        )
        .await
}

//EnterSectionCsReq
*/

//stole this from ami https://github.com/amizing25/robinsr

// enterscene
pub async fn on_lckgkdehclb(session: &PlayerSession, request: &Lckgkdehclb) -> Result<()> {
    // send packet first
    session
        .send(CMD_ENTER_SCENE_SC_RSP, GetBagCsReq{})
        .await?;

        println!("{:?}", request);

    let _ = load_scene(session, request.entry_id, true, Some(request.maplanefddc)).await;


    Ok(())
}

// getscenemapinfocsreq
pub async fn on_get_scene_map_info_cs_req(sesison: &PlayerSession, request: &GetSceneMapInfoCsReq) -> Result<()> {
    let mut map_infos = Vec::<Fjniajephmj>::new();

    for entry_id in &request.dmkkkfnkofh {
        let mut map_info = Fjniajephmj {
            retcode: 0,
            dcbdhkkkpgd: vec![
                Gbiimoglajl {
                    gommoeicmjg: Kihbdaniehp::MapInfoChestTypeNormal.into(),
                    ..Default::default()
                },
                Gbiimoglajl {
                    gommoeicmjg: Kihbdaniehp::MapInfoChestTypePuzzle.into(),
                    ..Default::default()
                },
                Gbiimoglajl {
                    gommoeicmjg: Kihbdaniehp::MapInfoChestTypeChallenge.into(),
                    ..Default::default()
                },
            ],
            entry_id: *entry_id,
            ..Default::default()
        };

        for i in 0..100 {
            map_info.phicefeaigb.push(i)
        }

        let group_config = GAME_RESOURCES
            .map_entrance
            .get(&entry_id)
            .map(|v| {
                GAME_RESOURCES
                    .level_group
                    .get(&format!("P{}_F{}", v.plane_id, v.floor_id))
            })
            .flatten();

        if let Some(level) = group_config {
            // add teleports
            for teleport in &level.teleports {
                map_info.ojlnmnehgai.push(*teleport.0)
            }

            for (group_id, group) in &level.group_items {
                map_info.pmolfbcbfpe.push(Gecjjlmabhp {
                    group_id: *group_id,
                    ..Default::default()
                });

                // prop
                for prop in &group.props {
                    map_info.cgkfbhoadpc.push(Kangcibfhee {
                        group_id: prop.group_id,
                        state: if prop.prop_state_list.contains(&PropState::CheckPointEnable) {
                            PropState::CheckPointEnable as u32
                        } else {
                            prop.state.clone() as u32
                        },
                        ifjocipnpgd: prop.id as u32,
                    });
                }
            }
        }

        // Debug
        // tokio::fs::write(format!("./text-{}.txt", entry_id), format!("{:#?}", map_info)).await?;
        map_infos.push(map_info)
    }

    sesison
        .send(
            CMD_GET_SCENE_MAP_INFO_SC_RSP,
            Cegeebldbke {
                retcode: 0,
                mhefdgcamjl: map_infos,
                ..Default::default()
            },
        )
        .await?;

    Ok(())
}

// lazy_static! {
//     static ref NEXT_SCENE_SAVE: Mutex<u64> = Mutex::new(0);
// }

pub async fn on_scene_entity_move_cs_req(session: &PlayerSession, request: &SceneEntityMoveCsReq) -> Result<()> {
    let player = &mut session.player_info_mut();
    // let mut timestamp = NEXT_SCENE_SAVE.lock().await;

    // if util::cur_timestamp_ms() <= *timestamp {
    //     session
    //     .send(CMD_SCENE_ENTITY_MOVE_SC_RSP, Dummy::default())
    //     .await?;

    //     return Ok(());
    // }
    
    // // save every 5 minute
    // *timestamp = util::cur_timestamp_ms() + (5 * 1000);
    
    for entity in &request.entity_motion_list {
        if entity.entity_id != 0 {
            continue;
        }

        if let Some(motion) = &entity.motion {
            if let Some(pos) = &motion.pos {
                player.position.x = pos.x;
                player.position.y = pos.y;
                player.position.z = pos.z;
            }
            if let Some(rot) = &motion.rot {
                player.position.rot_y = rot.y;
            }
        }
        player.position.entry_id = request.entry_id;
    }

    // player.save().await;
    session
        .send(CMD_SCENE_ENTITY_MOVE_SC_RSP, GetBagCsReq{})
        .await?;

    Ok(())
}

pub async fn on_scene_entity_teleport_cs_req(
    session: &PlayerSession, 
    request: &SceneEntityTeleportCsReq
) -> Result<()> {
    session
        .send(CMD_SCENE_ENTITY_TELEPORT_SC_RSP, SceneEntityTeleportScRsp{
            entity_motion: request.entity_motion.clone(),
            retcode: 0,
            oeipegeoedk: 0
        })
        .await
}

async fn load_scene(
    session: &PlayerSession,
    entry_id: u32,
    _save: bool,
    teleport_id: Option<u32>,
) -> Result<SceneInfo> {
    let player_info = &mut session.player_info_mut();
    let json = &GAME_RESOURCES;

    let enterance = GAME_RESOURCES
        .map_entrance
        .get(&entry_id)
        .ok_or_else(|| anyhow::format_err!("Map Entrance Not Found"))?;

    let plane = GAME_RESOURCES
        .maze_plane
        .get(&enterance.plane_id)
        .ok_or_else(|| anyhow::format_err!("Map Plane Not Found"))?;

    let group_config = GAME_RESOURCES
        .level_group
        .get(&format!("P{}_F{}", enterance.plane_id, enterance.floor_id))
        .ok_or_else(|| anyhow::format_err!("Group Config Not Found"))?;

    let mut position = player_info.position.clone();
    if let Some(teleport_id) = teleport_id {
        if let Some(teleport) = group_config.teleports.get(&teleport_id) {
            let anchor = group_config
                .group_items
                .get(&teleport.anchor_group_id.unwrap_or_default())
                .map(|v| v.anchors.get(&teleport.anchor_id.unwrap_or_default()))
                .flatten();
            if let Some(anchor) = anchor {
                position.x = (anchor.pos_x * 1000f64) as i32;
                position.y = (anchor.pos_y * 1000f64) as i32;
                position.z = (anchor.pos_z * 1000f64) as i32;
                position.rot_y = (anchor.rot_y * 1000f64) as i32;
            }
        }
    }

    let mut scene_info = SceneInfo {
        floor_id: enterance.floor_id as u32,
        plane_id: enterance.plane_id as u32,
        entry_id,
        game_mode_type: plane.plane_type as u32,
        pbfgagecpcd: plane.world_id,
        ..Default::default()
    };

    let lineup_info = player_info.lineup.clone();
    let player_pos = MotionInfo {
        // rot
        rot: Some(Vector {
            x: 0,
            y: position.rot_y,
            z: 0,
        }),
        // pos
        pos: Some(Vector {
            x: position.x,
            y: position.y,
            z: position.z,
        }),
    };

    let mut loaded_npc: Vec<u32> = vec![];
    let mut prop_entity_id = 10;
    let mut npc_entity_id = 10_000;
    let mut monster_entity_id = 20_000;

    for (group_id, group) in &group_config.group_items {
        let mut group_info = Dhkacjhaoid {
            state: 0,
            group_id: *group_id,
            ..Default::default()
        };

        // Load Props
        for prop in &group.props {
            let prop_state = if prop.prop_state_list.contains(&PropState::CheckPointEnable) {
                PropState::CheckPointEnable
            } else {
                prop.state.clone()
            };

            prop_entity_id += 1;

            let prop_position = Position {
                x: (prop.pos_x * 1000f64) as i32,
                y: (prop.pos_y * 1000f64) as i32,
                z: (prop.pos_z * 1000f64) as i32,
                rot_y: (prop.rot_y * 1000f64) as i32,
            };

            let entity_info = SceneEntityInfo {
                inst_id: prop.id as u32,
                group_id: prop.group_id,
                motion: Some(prop_position.to_motion()),
                prop: Some(ScenePropInfo {
                    prop_id: prop.prop_id,
                    prop_state: prop_state as u32,
                    ..Default::default()
                }),
                entity_id: prop_entity_id,
                ..Default::default()
            };

            group_info.entity_list.push(entity_info);
        }

        // Load NPCs
        for npc in &group.npcs {
            if loaded_npc.contains(&(npc.npcid as u32))
                || player_info.inventory.avatars.iter().find(|a| a.id == npc.npcid).is_some()
            {
                continue;
            }
            npc_entity_id += 1;
            loaded_npc.push(npc.npcid as u32);

            let npc_position = Position {
                x: (npc.pos_x * 1000f64) as i32,
                y: (npc.pos_y * 1000f64) as i32,
                z: (npc.pos_z * 1000f64) as i32,
                rot_y: (npc.rot_y * 1000f64) as i32,
            };

            let info = SceneEntityInfo {
                inst_id: npc.id as u32,
                group_id: npc.group_id,
                entity_id: npc_entity_id,
                motion: Some(npc_position.to_motion()),
                npc: Some(SceneNpcInfo {
                    egeneneoadj: npc.npcid as u32,
                    ..Default::default()
                }),
                ..Default::default()
            };

            group_info.entity_list.push(info);
        }

        // Load Monsters
        for monster in &group.monsters {
            monster_entity_id += 1;
            let monster_position = Position {
                x: (monster.pos_x * 1000f64) as i32,
                y: (monster.pos_y * 1000f64) as i32,
                z: (monster.pos_z * 1000f64) as i32,
                rot_y: (monster.rot_y * 1000f64) as i32,
            };

            let npc_monster = SceneNpcMonsterInfo {
                monster_id: monster.npcmonster_id as u32,
                event_id: monster.event_id as u32,
                world_level: 6,
                ..Default::default()
            };

            let info = SceneEntityInfo {
                inst_id: monster.id as u32,
                group_id: monster.group_id,
                entity_id: monster_entity_id,
                motion: Some(monster_position.to_motion()),
                npc_monster: Some(npc_monster),
                ..Default::default()
            };

            group_info.entity_list.push(info);
        }

        scene_info.chhmmbdhjpg.push(group_info);
    }

    // load player entity
    let mut player_group = Dhkacjhaoid {
        state: 0,
        group_id: 0,
        ..Default::default()
    };
    for (slot, avatar_id) in player_info.lineup.avatar_list.iter().map(|av| (av.slot, av.id)) {
        player_group.entity_list.push(SceneEntityInfo {
            inst_id: 0,
            entity_id: slot + 1,
            motion: Some(MotionInfo {
                // pos
                pos: Some(Vector {
                    x: player_info.position.x,
                    y: player_info.position.y,
                    z: player_info.position.z,
                }),
                // rot
                rot: Some(Vector {
                    x: 0,
                    y: player_info.position.rot_y,
                    z: 0,
                }),
            }),
            actor: Some(SceneActorInfo {
                avatar_type: AvatarType::AvatarFormalType.into(),
                base_avatar_id: avatar_id,
                map_layer: 0,
                uid: 0,
            }),
            ..Default::default()
        })
    }
    scene_info.chhmmbdhjpg.push(player_group);

    if _save {
        session
            .send(
                CMD_ENTER_SCENE_BY_SERVER_SC_NOTIFY,
                EnterSceneByServerScNotify {
                    scene: Some(scene_info.clone()),
                    lineup: Some(lineup_info),
                    ..Default::default()
                },
            )
            .await?;

        session
            .send(
                CMD_SCENE_ENTITY_MOVE_SC_NOTIFY,
                SceneEntityMoveScNotify {
                    entity_id: 0,
                    motion: Some(player_pos),
                    entry_id,
                    ..Default::default()
                },
            )
            .await?;

        player_info.position.entry_id = entry_id;
        player_info.position.floor_id = enterance.floor_id as u32;
        player_info.position.plane_id = enterance.plane_id as u32;
        player_info.position.x = position.x;
        player_info.position.y = position.y;
        player_info.position.z = position.z;
        player_info.position.rot_y = position.rot_y;
        // player_info.save().await
    }

    return Ok(scene_info);
}