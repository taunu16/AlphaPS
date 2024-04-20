use crate::{excel::tools_res::*, game::PlayerInfo};

use super::*;

pub async fn on_get_cur_scene_info_cs_req(
    session: &PlayerSession,
    _body: &GetCurSceneInfoCsReq,
) -> Result<()> {
    session
        .send(
            CMD_GET_CUR_SCENE_INFO_SC_RSP,
            GetCurSceneInfoScRsp {
                retcode: 0,
                scene: Some(load_scene(session, 2031301, false, Some(1014), &mut session.player_info_mut()).await.unwrap()),
            },
        )
        .await
}

//stole this from ami https://github.com/amizing25/robinsr

// enterscene
pub async fn on_lckgkdehclb(session: &PlayerSession, request: &Lckgkdehclb) -> Result<()> {
    // send packet first
    session
        .send(CMD_ENTER_SCENE_SC_RSP, GetBagCsReq{})
        .await?;

        println!("{:?}", request);

    let _ = load_scene(session, request.entry_id, true, Some(request.maplanefddc), &mut session.player_info_mut()).await;


    Ok(())
}

// getscenemapinfocsreq
pub async fn on_get_scene_map_info_cs_req(sesison: &PlayerSession, request: &GetSceneMapInfoCsReq) -> Result<()> {
    let mut map_infos = Vec::<MazeMapData>::new();

    for entry_id in &request.dmkkkfnkofh {
        let mut chest_count = 0;
        
        if let Some(entrance) = GAME_RESOURCES.map_entrance.get(&entry_id) && let Some(group) = GAME_RESOURCES.level_group.get(&format!("P{}_F{}", entrance.plane_id, entrance.floor_id)) {
            for (_, v)  in &group.group_items {
                for prop in &v.props {
                    if prop.__test_field.contains("_TreasureBox_") {
                        chest_count += 1;
                    }
                }
            }
        }

        let mut map_info = MazeMapData {
            retcode: 0,
            unlocked_chest_list: vec![
                MazeChest {
                    map_info_chest_type: MapInfoChestType::MapInfoChestTypeNormal.into(),
                    total_amount_list: chest_count,
                    unlocked_amount_list: 2137,
                },
                MazeChest {
                    map_info_chest_type: MapInfoChestType::MapInfoChestTypePuzzle.into(),
                    ..Default::default()
                },
                MazeChest {
                    map_info_chest_type: MapInfoChestType::MapInfoChestTypeChallenge.into(),
                    ..Default::default()
                },
            ],
            aechnhklpkp: vec![
                Kbbeoemcdhi {
                    feiakppdjea: 0, //val
                    iookcfnfjha: 0, //max
                    ipnhjoomhdm: 1
                },
                Kbbeoemcdhi {
                    feiakppdjea: 0,
                    iookcfnfjha: 0,
                    ipnhjoomhdm: 2
                }
            ],
            entry_id: *entry_id,
            ..Default::default()
        };

        for i in 1..100 {
            for j in 0..100 {
                map_info.lighten_section_list.push((i * 10000) + j)
            }
        }

        // map_info.lighten_section_list.append(&mut vec![10000, 20000, 30000, 400002, 50000, 60000, 70000, 80000]);

        for i in 0..100 {
            map_info.lighten_section_list.push(i)
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
                map_info.unlocked_teleport_list.push(*teleport.0)
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
            GetSceneMapInfoScRsp {
                retcode: 0,
                map_list: map_infos,
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
        if entity.entity_id > 4 {
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
    if let Some(emotion) = &request.entity_motion && let Some(motion) = &emotion.motion && let Some(pos) = &motion.pos && let Some(rot) = &motion.rot {
        if emotion.entity_id < 5 {
            let player_info = &mut session.player_info_mut();
            let position = &mut player_info.position;
            position.x = pos.x;
            position.y = pos.y;
            position.z = pos.z;
            position.rot_y = rot.y;
        }
    }
    session
        .send(CMD_SCENE_ENTITY_TELEPORT_SC_RSP, SceneEntityTeleportScRsp{
            entity_motion: request.entity_motion.clone(),
            retcode: 0,
            oeipegeoedk: 0
        })
        .await
}

pub async fn load_scene(
    session: &PlayerSession,
    entry_id: u32,
    _save: bool,
    teleport_id: Option<u32>,
    player_info: &mut PlayerInfo
) -> Result<SceneInfo> {

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

    player_info.scene_prop_cache.clear();

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
            player_info.scene_prop_cache.insert(prop_entity_id, prop.clone());
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

    for i in 1..100 {
        for j in 0..100 {
            scene_info.lighten_section_list.push((i * 10000) + j)
        }
    }

    for i in 0..100 {
        scene_info.lighten_section_list.push(i)
    }

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