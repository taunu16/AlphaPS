use crate::{game::other::tools_importer, handle_error, parse_u32, parse_u32_def, safe_unwrap_result};
use anyhow::Ok;
use atomic_refcell::AtomicRefMut;
use net::handlers::load_scene;
use proto::*;
use crate::net::*;
use crate::*;

use super::{inventory, PlayerInfo};

#[derive(Default, Clone)]
pub struct CommandSystem {
    state: CMDSystemState
}

#[derive(Clone)]
pub enum CMDSystemState {
    Normal,
    RelicBuilder(Vec<inventory::RelicAffix>, bool)
}

impl Default for CMDSystemState {
    fn default() -> Self { Self::Normal }
}

pub async fn invoke(
    session: &PlayerSession,
    body: &SendMsgCsReq,
) -> Result<()> {
    macro_rules! send_message {
        ($($arg:tt)*) => {{
            let res = std::fmt::format(std::format_args!($($arg)*));
            session.send(
                CMD_REVC_MSG_SC_NOTIFY, 
                Kifdjbodlcc {
                    chat_type: body.chat_type,
                    msg_type: body.msg_type,
                    text: res.to_owned(),
                    aljhmlmnmhp: 0,
                    djefnoaonkc: 2137,
                    emote: 0,
                    ..Default::default()
                }
            ).await
        }};
        ($t:expr) => {
            session.send(
                CMD_REVC_MSG_SC_NOTIFY, 
                Kifdjbodlcc {
                    chat_type: body.chat_type,
                    msg_type: body.msg_type,
                    text: "".to_owned(),
                    aljhmlmnmhp: 0,
                    djefnoaonkc: 2137,
                    emote: $t,
                    ..Default::default()
                }
            ).await
        };
    }
    safe_unwrap_result!(
        session.send(CMD_SEND_MSG_SC_RSP, GetBagCsReq{}).await
    );
    safe_unwrap_result!(session.send(
        CMD_REVC_MSG_SC_NOTIFY, 
        Kifdjbodlcc {
            chat_type: body.chat_type,
            msg_type: body.msg_type,
            text: body.text.clone(),
            aljhmlmnmhp: 2137,
            djefnoaonkc: 0,
            emote: body.emote,
            ..Default::default()
        }
    ).await);

    println!("{:?}", body);

    let args = body.text.strip_prefix("/").unwrap_or(&body.text).split(" ").collect::<Vec<_>>();
    let player_info = &mut session.player_info_mut();

    match &player_info.command_system.clone().state {
        CMDSystemState::Normal => {
            if !body.text.starts_with("/") {
                return session.send(
                    CMD_REVC_MSG_SC_NOTIFY, 
                    Kifdjbodlcc {
                        chat_type: body.chat_type,
                        msg_type: body.msg_type,
                        text: body.text.clone(),
                        aljhmlmnmhp: 0,
                        djefnoaonkc: 2137,
                        emote: body.emote,
                        ..Default::default()
                    }
                ).await;
            }

            process(args, session, player_info).await
        },
        CMDSystemState::RelicBuilder(affix, last_phase) => {
            if *last_phase {
                safe_unwrap_result!(player_info.inventory.add_relic(session, parse_u32!(args[0]), parse_u32_def!(args.get(1), "15"), parse_u32_def!(args.get(2), "1"), affix.clone()).await);
                player_info.command_system.state = CMDSystemState::Normal;
                return send_message!("Relic saved");
            }

            if args[0] == "end" {
                player_info.command_system.state = CMDSystemState::RelicBuilder(affix.clone(), true);
                return send_message!("OK, now input: [relic id] <level:15> <main affix id:1>");
            }

            let mut affix = affix.clone();
            affix.push(inventory::RelicAffix { id: parse_u32!(args[0]), step: parse_u32!(args[0])*2, cnt: parse_u32!(args[0]) });
            player_info.command_system.state = CMDSystemState::RelicBuilder(affix.clone(), false);
            Ok(())
        }
    }
}

async fn process(args: Vec<&str>, session: &PlayerSession, player_info: &mut AtomicRefMut<'_, PlayerInfo>) -> Result<()> {
    macro_rules! send_message {
        ($($arg:tt)*) => {{
            let res = std::fmt::format(std::format_args!($($arg)*));
            session.send(
                CMD_REVC_MSG_SC_NOTIFY, 
                Kifdjbodlcc {
                    chat_type: ChatType::Private.into(),
                    msg_type: MsgType::CustomText.into(),
                    text: res.to_owned(),
                    aljhmlmnmhp: 0,
                    djefnoaonkc: 2137,
                    emote: 0,
                    ..Default::default()
                }
            ).await
        }};
        ($t:expr) => {
            session.send(
                CMD_REVC_MSG_SC_NOTIFY, 
                Kifdjbodlcc {
                    chat_type: ChatType::Private.into(),
                    msg_type: MsgType::Emote.into(),
                    text: "".to_owned(),
                    aljhmlmnmhp: 0,
                    djefnoaonkc: 2137,
                    emote: $t,
                    ..Default::default()
                }
            ).await
        };
    }
    let inventory = &mut player_info.inventory;
    match args[0] {
        "give" => {
            safe_unwrap_result!(inventory.add_item(session, parse_u32!(args[1]), parse_u32_def!(args.get(2), "1")).await);
            return send_message!("Gave {} of {}", parse_u32_def!(args.get(2), "1"), parse_u32!(args[1]));
        },
        "take" => {
            safe_unwrap_result!(inventory.take_item(session, parse_u32!(args[1]), parse_u32_def!(args.get(2), "1")).await);
            return send_message!("Taken {} of {}", parse_u32_def!(args.get(2), "1"), parse_u32!(args[1]));
        },
        "givel" => {
            return inventory.add_lightcone(session, parse_u32!(args[1]), parse_u32_def!(args.get(2), "1"), parse_u32_def!(args.get(3), "1"), parse_u32_def!(args.get(3), "0")).await;
        },
        "givea" => {
            return inventory.add_avatar(session, parse_u32!(args[1]), parse_u32_def!(args.get(2), "6"), parse_u32_def!(args.get(3), "0"), parse_u32_def!(args.get(3), "80")).await;
        },
        "giver" => {
            return inventory.add_relic(session, parse_u32!(args[1]), parse_u32_def!(args.get(2), "15"), parse_u32_def!(args.get(3), "1"), vec![]).await;
        },
        "skilltree" => {
            if args.len() < 4 || !(args.len() >= 3 && args[2] == "all") {
                return send_message!("Usage: /skilltree [avatar id] ([short tree id] [value] | none | max)");
            }
            if args.len() >= 3 {
                match args[2] {
                    "all" => {
                        safe_unwrap_result!(inventory.set_avatar_skilltree_to_map(session, parse_u32!(args[1]), vec![
                            (1, 6),
                            (2, 10),
                            (3, 10),
                            (4, 10),
                            (7, 1),
                            (101, 1),
                            (102, 1),
                            (103, 1),
                            (201, 1),
                            (202, 1),
                            (203, 1),
                            (204, 1),
                            (205, 1),
                            (206, 1),
                            (207, 1),
                            (208, 1),
                            (209, 1),
                            (210, 1),
                        ]).await);
                        return send_message!("Upgraded skilltree for {} to max", args[2]);
                    },
                    "none" => {
                        safe_unwrap_result!(inventory.set_avatar_skilltree_to_map(session, parse_u32!(args[1]), vec![]).await);
                        return send_message!("Upgraded skilltree for {} to none", args[2]);
                    },
                    _ => {}
                }
            }
            safe_unwrap_result!(inventory.set_avatar_skilltree(session, parse_u32!(args[1]), parse_u32!(args[2]), parse_u32!(args[3])).await);
            return send_message!("Skilltree {} set to {} for {}", args[2], args[3], args[1]);
        },
        "relicbuilder" => {
            player_info.command_system.state = CMDSystemState::RelicBuilder(vec![], false);
            return send_message!("OK, now type minor affixes in format: \"[affix id] [steps]\"; to end type \"end\"");
        },
        "importtools" => {
            safe_unwrap_result!(tools_importer::import(session, inventory).await);
            return send_message!("Import successful");
        },
        "giveall" => {
            safe_unwrap_result!(inventory.give_all_items(session, parse_u32_def!(args.get(1), "1")).await);
            return send_message!("Gave everything");
        },
        "lineup" => {
            player_info.lineup.avatar_list = vec![];
            let mut i = 0;
            for chr in args {
                if chr.starts_with("/") || chr.parse::<u32>().is_err() {
                    continue;
                }

                player_info.lineup.avatar_list.push(proto::LineupAvatar {
                    id: chr.parse().unwrap(),
                    slot: i,
                    hp: 10000,
                    sp: Some(AmountInfo {
                        cur_amount: 10000,
                        max_amount: 10000,
                    }),
                    satiety: 100,
                    avatar_type: 3,
                });

                i += 1;
            }

            safe_unwrap_result!(player_info.sync_lineup(session, player_info.lineup.clone()).await);
            return send_message!("Lineup set");
        },
        "scene" => {
            let scene = parse_u32!(args[1]);

            safe_unwrap_result!(load_scene(session, scene, true, args.get(2).map(|s| s.parse().unwrap()), player_info).await);

            // safe_unwrap_result!(session
            //     .send(
            //         CMD_ENTER_SCENE_BY_SERVER_SC_NOTIFY,
            //         EnterSceneByServerScNotify {
            //             lineup: Some(player_info.lineup.clone()),
            //             scene: Some(SceneInfo {
            //                 game_mode_type: 2,
            //                 entry_id: scene,
            //                 plane_id: (scene as f32 / 100f32).floor() as u32,
            //                 floor_id: ((scene as f32 / 100f32).floor() * 1000f32).floor() as u32 + 1u32,
            //                 ..Default::default()
            //             }),
            //             bpodijpdnnk: Ffnhcbelgpd::EnterSceneReasonNone.into()
            //         }
            //     )
            //     .await);

            return send_message!("Teleported to {} (floor {})", scene, 1);
        },
        "tpx" => {
            safe_unwrap_result!(tp(session, player_info, Some(parse_i32!(args[1])), None, None).await);

            return send_message!("Teleported");
        },
        "tpy" => {
            safe_unwrap_result!(tp(session, player_info, None, Some(parse_i32!(args[1])), None).await);

            return send_message!("Teleported");
        },
        "tpz" => {
            safe_unwrap_result!(tp(session, player_info, None, None, Some(parse_i32!(args[1]))).await);

            return send_message!("Teleported");
        },
        "pos" => send_message!("x: {} \r\ny: {} \r\n z: {} \r\nyrot: {} \r\nentryid: {}", player_info.position.x, player_info.position.y, player_info.position.z, player_info.position.rot_y, player_info.position.entry_id),
        _ => Ok(())
    }
}

async fn tp(session: &PlayerSession, player_info: &mut PlayerInfo, x: Option<i32>, y: Option<i32>, z: Option<i32>) -> Result<()> {
    if let Some(x) = x {
        player_info.position.x = x;
    }
    if let Some(y) = y {
        player_info.position.y = y;
    }
    if let Some(z) = z {
        player_info.position.z = z;
    }

    session.send(
        CMD_SCENE_ENTITY_MOVE_SC_NOTIFY, 
        SceneEntityMoveScNotify {
            entity_id: 0,
            motion: Some(MotionInfo { 
                // rot
                rot: Some(Vector {
                    x:0,
                    y: player_info.position.rot_y,
                    z: 0,
                }),
                // pos
                pos: Some(Vector {
                    x: player_info.position.x,
                    y: player_info.position.y,
                    z: player_info.position.z,
                }),

            }),
            entry_id: player_info.position.entry_id,
            ..Default::default()
        }
    ).await
}