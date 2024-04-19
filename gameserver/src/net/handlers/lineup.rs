use super::*;
use crate::game::globals;

pub async fn on_get_all_lineup_data_cs_req(
    session: &PlayerSession,
    _body: &GetAllLineupDataCsReq,
) -> Result<()> {
    session
        .send(
            CMD_GET_ALL_LINEUP_DATA_SC_RSP,
            GetAllLineupDataScRsp {
                retcode: 0,
                cur_index: 0,
                lineup_list: vec![session.player_info().lineup.clone()],
            },
        )
        .await
}

pub async fn on_get_cur_lineup_data_cs_req(
    session: &PlayerSession,
    _body: &GetCurLineupDataCsReq,
) -> Result<()> {
    session
        .send(
            CMD_GET_CUR_LINEUP_DATA_SC_RSP,
            GetCurLineupDataScRsp {
                retcode: 0,
                lineup: Some(session.player_info().lineup.clone()),
            },
        )
        .await
}

pub async fn on_change_lineup_leader_cs_req(
    session: &PlayerSession,
    body: &ChangeLineupLeaderCsReq,
) -> Result<()> {
    session.player_info_mut().lineup.index = body.slot;
    session
        .send(
            CMD_CHANGE_LINEUP_LEADER_SC_RSP,
            ChangeLineupLeaderScRsp {
                slot: body.slot,
                retcode: 0,
            },
        )
        .await
}

pub async fn on_join_lineup_cs_req(session: &PlayerSession, body: &JoinLineupCsReq) -> Result<()> {
    let mut player_info = session.player_info_mut();

    if !(0..4).contains(&body.slot) {
        return session
            .send(
                CMD_JOIN_LINEUP_SC_RSP,
                JoinLineupScRsp {
                    retcode: Retcode::RetLineupInvalidMemberPos as u32,
                },
            )
            .await;
    }

    if player_info
        .lineup
        .avatar_list
        .iter()
        .any(|avatar| avatar.slot == body.slot)
    {
        return session
            .send(
                CMD_JOIN_LINEUP_SC_RSP,
                JoinLineupScRsp {
                    retcode: Retcode::RetLineupAvatarAlreadyInit as u32,
                },
            )
            .await;
    }

    player_info
        .lineup
        .avatar_list
        .push(lineup_avatar(body.base_avatar_id, body.slot));

    player_info.fix_trailblazer();
    player_info.sync_lineup(session, player_info.lineup.clone()).await?;
    session
        .send(CMD_JOIN_LINEUP_SC_RSP, JoinLineupScRsp::default())
        .await
}

pub async fn on_replace_lineup_cs_req(
    session: &PlayerSession,
    body: &ReplaceLineupCsReq,
) -> Result<()> {
    let mut player_info = session.player_info_mut();

    player_info.lineup.avatar_list.clear();
    for slot_info in &body.lineup_slots {
        player_info
            .lineup
            .avatar_list
            .push(lineup_avatar(slot_info.id, slot_info.slot));
    }
    player_info.lineup.leader_slot = body.leader_slot;

    player_info.fix_trailblazer();
    player_info.sync_lineup(session, player_info.lineup.clone()).await?;
    session
        .send(CMD_REPLACE_LINEUP_SC_RSP, ReplaceLineupScRsp::default())
        .await
}

pub async fn on_quit_lineup_cs_req(session: &PlayerSession, body: &QuitLineupCsReq) -> Result<()> {
    let mut player_info = session.player_info_mut();
    player_info
        .lineup
        .avatar_list
        .retain(|avatar| avatar.id != body.base_avatar_id);

    player_info.fix_trailblazer();
    player_info.sync_lineup(session, player_info.lineup.clone()).await?;
    session
        .send(
            CMD_QUIT_LINEUP_SC_RSP,
            QuitLineupScRsp {
                plane_id: body.plane_id,
                is_mainline: !body.is_virtual,
                is_virtual: body.is_virtual,
                base_avatar_id: body.base_avatar_id,
                retcode: 0,
            },
        )
        .await
}

fn lineup_avatar(id: u32, slot: u32) -> LineupAvatar {
    LineupAvatar {
        id,
        slot,
        hp: 10000,
        sp: Some(AmountInfo {
            cur_amount: 10000,
            max_amount: 10000,
        }),
        satiety: 100,
        avatar_type: 3,
    }
}

pub async fn on_get_assist_list_cs_req(
    session: &PlayerSession,
    _body: &GetAssistListCsReq,
) -> Result<()> {
    session.send(
        CMD_GET_ASSIST_LIST_SC_RSP, 
        Dmhpmbogjco {
            retcode: 0,
            bimojojomfc: vec![
                Gimicolccfe {
                    lehcnanhjdp: Some(DisplayAvatarDetailInfo {
                        level: 80,
                        avatar_id: 1308,
                        relic_list: vec![],
                        skilltree_list: vec![],
                        promotion: 6,
                        rank: 6,
                        ..Default::default()
                    }),
                    simple_info: Some(SimpleInfo {
                        nickname: "Acheron".to_owned(),
                        uid: 4001,
                        level: 69,
                        is_banned: false,
                        signature: "<b>nice</b>".to_owned(),
                        platform_type: PlatformType::Pc.into(),
                        online_status: FriendOnlineStatus::FriendOnlineStatusOnline.into(),
                        gjlfhjlijon: 200103,
                        ..Default::default()
                    })
                }
            ]
        }
    ).await
}

pub async fn on_set_assist_cs_req(
    session: &PlayerSession,
    body: &SetAssistCsReq,
) -> Result<()> {
    let player_info = &mut session.player_info_mut();
    
    if let Some(slot) = player_info.lineup.avatar_list.iter_mut().find(|a| a.slot == 3) {
        slot.avatar_type = AvatarType::AvatarAssistType as i32;
        slot.id = body.ckondfhadld;
        //great way of storing this
        slot.sp = Some(AmountInfo {
            cur_amount: body.uid,
            max_amount: body.uid
        })
    } else {
        let slot = player_info.lineup.avatar_list.len() as u32;
        player_info.lineup.avatar_list.push(LineupAvatar {
            avatar_type: AvatarType::AvatarAssistType as i32,
            hp: 10000,
            sp: Some(AmountInfo {
                cur_amount: body.uid,
                max_amount: body.uid
            }),
            id: body.ckondfhadld,
            satiety: 100,
            slot
        })
    }

    player_info.sync_lineup(session, player_info.lineup.clone()).await;
    session.send(
        CMD_SET_ASSIST_SC_RSP,
        SetAssistScRsp {
            ckondfhadld: body.ckondfhadld,
            uid: body.uid,
            retcode: 0
        } 
    ).await
}