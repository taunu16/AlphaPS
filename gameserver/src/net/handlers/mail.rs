use std::time::{SystemTime, UNIX_EPOCH};

use crate::{excel::EXCEL, safe_unwrap_result};

use super::*;

pub async fn on_get_mail_cs_req(
    session: &PlayerSession,
    _body: &GetMailCsReq,
) -> Result<()> {
    // safe_unwrap!(session.send(CMD_PLAYER_SYNC_SC_NOTIFY, Eckkajafean {
    //     relic_list: vec![
    //         Relic {
    //             tid: 63065,
    //             level: 15,
    //             main_affix_id: 1,
    //             unique_id: 12,
    //             sub_affix_list: vec![
    //                 RelicAffix {
    //                     affix_id: 2,
    //                     step: 4,
    //                     ..Default::default()
    //                 }
    //             ],
    //             ..Default::default()
    //         }
    //     ],
    //     // cngnejklkif: Some(ItemList {
    //     //     item_list: vec![
    //     //         Item {
    //     //             item_id: 300031,
    //     //             num: 2137,
    //     //             ..Default::default()
    //     //         }
    //     //     ]
    //     // }),
    //     // dmkpcemhcga: vec![
    //     //     Fhhbjljchea {
    //     //         djhoddohmnb: 300031,
    //     //         num: 2137 
    //     //     }
    //     // ],
    //     jiljjaiepgc: vec![
    //         Afpohdndlni {
    //             tid: 300031,
    //             num: 2137,
    //             ..Default::default()
    //         },
    //         Afpohdndlni {
    //             tid: 300021,
    //             num: 10,
    //             ..Default::default()
    //         }
    //     ],
    //     ..Default::default()
    // }).await);

    session
        .send(
            CMD_GET_MAIL_SC_RSP,
            GetMailScRsp {
                retcode: 0,
                is_end: true,
                start: 0,
                total_num: 1,
                mail_list: vec![
                    ClientMail {
                        title: "test".to_string(),
                        content: "<color=red><size=120>car</size></color>".to_string(),
                        sender: "miheayo".to_owned(),
                        expire_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64 + 1000i64,
                        is_read: false,
                        id: 1,
                        template_id: 0,
                        mail_type: 0,
                        para_list: vec![],
                        time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
                        attachment: Some(ItemList { item_list: vec![
                            Item {
                                item_id: 300031,
                                num: 2137,
                                ..Default::default()
                            }
                        ]})
                    }
                ],
                notice_mail_list: vec![]
            },
        )
        .await
}

pub async fn on_get_bag_cs_req(
    session: &PlayerSession,
    _body: &GetBagCsReq,
) -> Result<()> {
    let inventory = &session.player_info().inventory;
    session
        .send(
            CMD_GET_BAG_SC_RSP,
            GetBagScRsp {
                retcode: 0,
                relic_list: inventory.relic_to_proto(),
                material_list: inventory.items_to_proto(),
                equipment_list: inventory.lightcones_to_proto(),
                ..Default::default()
            },
        )
        .await
}

pub async fn on_use_item_cs_req(
    session: &PlayerSession,
    body: &UseItemCsReq,
) -> Result<()> {
    println!("{:?}", body);
     
    let mut player_info = session.player_info_mut();
    let buff_data = EXCEL.item.buff_data.iter().find(|iubd| iubd.use_data_id == body.use_item_id);
    if buff_data.is_none() {
        return session
        .send(
            CMD_USE_ITEM_SC_RSP,
            UseItemScRsp {
                retcode: 0,
                onnkbfefeok: body.onnkbfefeok,
                fghjlkadkpp: Option::None,
                use_item_id: body.use_item_id,
                ..Default::default()
            }
        ).await;
    }
    let buff_data = buff_data.unwrap();

    let mut sync_reasons: Vec<SyncLineupReason> = vec![];

    if buff_data.preview_skill_point > 0.0 {
        player_info.lineup.mp = (player_info.lineup.mp + buff_data.preview_skill_point as u32).clamp(0, player_info.lineup.mp_max);
        sync_reasons.push(SyncLineupReason::SyncReasonMpAdd);
    }

    safe_unwrap_result!(player_info.inventory.take_item(session, body.use_item_id, 1).await);
    if sync_reasons.is_empty() {
        safe_unwrap_result!(player_info.sync_lineup(session, player_info.lineup.clone()).await);
    } else {
        safe_unwrap_result!(player_info.sync_lineup_reason(session, sync_reasons).await);
    }

    session
        .send(
            CMD_USE_ITEM_SC_RSP,
            UseItemScRsp {
                retcode: 0,
                use_item_id: body.use_item_id,
                onnkbfefeok: body.onnkbfefeok,
                ..Default::default()
            }
        ).await
}