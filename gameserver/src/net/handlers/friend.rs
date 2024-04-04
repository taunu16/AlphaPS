use crate::{game::commands, handle_error, parse_u32, parse_u32_def, safe_unwrap_result};

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
                            // plmbeaaegak: vec![],
                            is_banned: false,
                            ailinangjne: "test1".to_owned(),
                            ldfiofjhjja: "test2".to_owned(),
                            signature: "POLSKA GUROM 🇵🇱".to_owned(),
                            platform_type: PlatformType::Pc.into(),
                            online_status: FriendOnlineStatus::FriendOnlineStatusOnline.into(),
                            gjlfhjlijon: 200101,
                            jpajpffgnbi: 0,
                            oopogjgllpg: 0,
                            ..Default::default()
                        }),
                        ..Default::default()
                    }
                ],
                domnlbiijfb: vec![],
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
        Ooibcglpnac {
            retcode: 0,
            pgofeopnpbm: vec![
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
                Bpifmdladdn {
                    emote: 0,
                    fbelgjfhbkh: "
<ųũźť=20>Server Console</ųũźť>
                    ".trim().to_owned(),
                    msg_type: MsgType::CustomText.into(),
                    nokipdbhglc: 0,
                    phhhfhobhmk: 0
                },
            ],
            fjbkleaflam: 0,
            oligkfnjkma: 0,
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