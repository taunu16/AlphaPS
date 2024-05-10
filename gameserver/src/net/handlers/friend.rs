use crate::game::commands;

use super::*;

pub async fn on_get_friend_list_info_cs_req(
    session: &PlayerSession,
    _body: &GetFriendListInfoCsReq,
) -> Result<()> {
    session
        .send(
            CMD_GET_FRIEND_LIST_INFO_SC_RSP,
            GetFriendListInfoScRsp {
                friend_list: vec![
                    FriendListInfo {
                        simple_info: Some(SimpleInfo {
                            nickname: "AlphaServer".to_owned(),
                            uid: 0,
                            level: 69,
                            oemkmkkhkde: vec![],
                            lnaaoaojpff: false,
                            ejknhjgaibm: "test1".to_owned(),
                            aionaipakjc: "test2".to_owned(),
                            ldcholahmei: "POLSKA GUROM 🇵🇱".to_owned(),
                            platform: PlatformType::Pc.into(),
                            mjbmlkimpnn: Iciddlkpjpe::FriendOnlineStatusOnline.into(),
                            kpiphlhfiib: 0,
                            dgojjmfnomj: 200101,
                            jnlnifcohpc: 0,
                            ..Default::default()
                        }),
                        ..Default::default()
                    }
                ],
                hgloaipjiob: vec![],
                retcode: 0,
                ..Default::default()
            }
        )
        .await
}

pub async fn on_get_private_chat_history_cs_req(
    session: &PlayerSession,
    body: &GetPrivateChatHistoryCsReq,
) -> Result<()> {
    println!("{:?}", body);
    session.send(
        CMD_GET_PRIVATE_CHAT_HISTORY_SC_RSP,
        GetPrivateChatHistoryScRsp {
            retcode: 0,
            fepangjbmpa: vec![
//                 Bpifmdladdn {
//                     emote: 0,
//                     fbelgjfhbkh: "
// <ųũźť=20>Konsola serwera AlphaPS</ųũźť>\n
// <ųũźť=15>Wpisz <Ţ>/help</Ţ> na listę komend</ųũźť>\n
// <ųũźť=11>Napisz niekomendę dla efektu echo</ųũźť>
//                     ".trim().to_owned(),
//                     msg_type: MsgType::CustomText.into(),
//                     nokipdbhglc: 0,
//                     phhhfhobhmk: 0
//                 },
//                 Bpifmdladdn {
//                     emote: 0,
//                     fbelgjfhbkh: "
// <ųũźť=20>AlphaPS Console</ųũźť>\n
// <ųũźť=15>Type <Ţ>/help</Ţ> for help</ųũźť>
//                     ".trim().to_owned(),
//                     msg_type: MsgType::CustomText.into(),
//                     nokipdbhglc: 0,
//                     phhhfhobhmk: 1
//                 },
// there is no help command
                Gnbekokccfc {
                    emote: 0,
                    anjfbjikchf: "Server Console".trim().to_owned(),
                    msg_type: MsgType::CustomText.into(),
                    hkkkjdbgiao: 0, //time
                    befmkobhohp: 0,
                    ggadmjhlomj: Option::None
                },
            ],
            cibgdjekbja: 0,
            ienomggikon: 0,
            ..Default::default()
        }
    ).await
}

pub async fn on_send_msg_cs_req(
    session: &PlayerSession,
    body: &SendMsgCsReq,
) -> Result<()> {
    commands::invoke(session, body).await
}

pub async fn on_get_player_detail_info_cs_req(
    session: &PlayerSession,
    body: &GetPlayerDetailInfoCsReq,
) -> Result<()> {
    session.send(
        CMD_GET_PLAYER_DETAIL_INFO_SC_RSP,
        match body.uid {
            0 => GetPlayerDetailInfoScRsp {
                retcode: 0,
                knmbdmpogkc: Some(Pplihjclmmk {
                    uid: body.uid,
                    ldcholahmei: "POLSKA GUROM 🇵🇱".to_owned(),
                    level: 69,
                    hkbddialajk: false,
                    nickname: "AlphaServer".to_owned(),
                    platform: PlatformType::Pc.into(),
                    dgojjmfnomj: 200101,
                    world_level: 6,
                    ejknhjgaibm: "test1".to_owned(),
                    aionaipakjc: "test2".to_owned(),
                    ..Default::default()
                })
            },
            2137 => GetPlayerDetailInfoScRsp {
                retcode: 0,
                knmbdmpogkc: Some(Pplihjclmmk {
                    uid: body.uid,
                    ldcholahmei: "polando".to_owned(),
                    level: 70,
                    hkbddialajk: false,
                    nickname: "Alpha".to_owned(),
                    platform: PlatformType::Pc.into(),
                    // gjlfhjlijon: 200101,
                    world_level: 6,
                    ejknhjgaibm: "test1".to_owned(),
                    aionaipakjc: "test2".to_owned(),
                    ..Default::default()
                })
            },
            _ => GetPlayerDetailInfoScRsp {
                retcode: Retcode::RetServerInternalError as u32,
                knmbdmpogkc: Option::None
            }
        }
    ).await
}