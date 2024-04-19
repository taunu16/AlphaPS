use std::collections::HashMap;

use crate::util;

use super::*;

pub async fn on_get_basic_info_cs_req(
    session: &PlayerSession,
    _body: &GetBasicInfoCsReq,
) -> Result<()> {
    session
        .send(
            CMD_GET_BASIC_INFO_SC_RSP,
            GetBasicInfoScRsp {
                retcode: 0,
                player_setting_info: Some(PlayerSettingInfo::default()),
                ..Default::default()
            },
        )
        .await
}

pub async fn on_get_hero_basic_type_info_cs_req(
    session: &PlayerSession,
    _body: &GetHeroBasicTypeInfoCsReq,
) -> Result<()> {
    session
        .send(
            CMD_GET_HERO_BASIC_TYPE_INFO_SC_RSP,
            GetHeroBasicTypeInfoScRsp {
                retcode: 0,
                gender: Gender::Man.into(),
                cur_basic_type: HeroBasicType::BoyShaman.into(),
                basic_type_info_list: vec![HeroBasicTypeInfo {
                    basic_type: HeroBasicType::BoyShaman.into(),
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
        .await
}

pub async fn on_player_heart_beat_cs_req(
    session: &PlayerSession,
    body: &PlayerHeartBeatCsReq,
) -> Result<()> {
    session
        .send(
            CMD_PLAYER_HEART_BEAT_SC_RSP,
            PlayerHeartBeatScRsp {
                retcode: 0,
                client_time_ms: body.client_time_ms,
                server_time_ms: util::cur_timestamp_ms(),
                download_data: Some(ClientDownloadData {
                    version: 51,
                    time: util::cur_timestamp_ms() as i64,
                    data: rbase64::decode("G0x1YVMBGZMNChoKBAQICHhWAAAAAAAAAAAAAAAod0AB+kNTLlVuaXR5RW5naW5lLkdhbWVPYmplY3QuRmluZCgiVUlSb290L0Fib3ZlRGlhbG9nL0JldGFIaW50RGlhbG9nKENsb25lKSIpOkdldENvbXBvbmVudEluQ2hpbGRyZW4odHlwZW9mKENTLlJQRy5DbGllbnQuTG9jYWxpemVkVGV4dCkpLnRleHQgPSAiPHNpemU9MjA+QWxwaGFQUyAoQmFzZWQgb24gUm9iaW5TUiBbZGlzY29yZC5nZy9yZXZlcnNlZHJvb21zXSkgaXMgYSBmcmVlIGFuZCBvcGVuIHNvdXJjZSBzb2Z0d2FyZS48L3NpemU+IgAAAAAAAAAAAAEEEAAAACQAQAApQEAAKYBAACnAQABWAAEALIAAAR1AQQCkgEEA5ABAAOnAwQHpAMIB6UDCAawAAAEsgAAAH8BChRkAgAAMAAAABANDUwQMVW5pdHlFbmdpbmUEC0dhbWVPYmplY3QEBUZpbmQEKVVJUm9vdC9BYm92ZURpYWxvZy9CZXRhSGludERpYWxvZyhDbG9uZSkEF0dldENvbXBvbmVudEluQ2hpbGRyZW4EB3R5cGVvZgQEUlBHBAdDbGllbnQEDkxvY2FsaXplZFRleHQEBXRleHQUajxzaXplPTIwPkFscGhhUFMgKEJhc2VkIG9uIFJvYmluU1IgW2Rpc2NvcmQuZ2cvcmV2ZXJzZWRyb29tc10pIGlzIGEgZnJlZSBhbmQgb3BlbiBzb3VyY2Ugc29mdHdhcmUuPC9zaXplPgEAAAABAAAAAAAQAAAAAQAAAAEAAAABAAAAAQAAAAEAAAABAAAAAQAAAAEAAAABAAAAAQAAAAEAAAABAAAAAQAAAAEAAAABAAAAAQAAAAAAAAABAAAABV9FTlY=").unwrap()
                }),
            },
        )
        .await
}

//travel broshure
pub async fn on_phbnokkhgkd(
    session: &PlayerSession,
    _body: &Phbnokkhgkd,
) -> Result<()> {
    let ids = HashMap::from([
        (12, 1300),
        (1, 101),
        (2, 200),
        (3, 300),
        (4, 402),
        (101, 0),
        (5, 502),
        (102, 0),
        (6, 600),
        (7, 702),
        (8, 800),
        (9, 1001),
        (10, 1101),
        (11, 1202),

        (13, 1401),
        (14, 1501),
    ]);
    let mut elements = HashMap::new();

    for i in vec![101u32, 102, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14] {
        elements.insert(i, Bbjejalfhen {
            gfehjonbcmn: Nhnehbekhhj::PageInteracted as i32,
            hajfkgdcglb: Pefolbeomfh::PageDescCollapse as i32,
            oockdmpidlg: vec![
                Gbofekdfncl {
                    item_id: 223000,
                    unique_id: 223000,
                    ohchcpejldh: 10,
                    pnaebbacmlm: 10,
                    pddanmnadaf: 10,
                    lblemehjiek: 10,
                    ..Default::default()
                }
            ],
            egigfgpjddg: *ids.get(&i).unwrap_or(&0),
            idgckihophm: i
        });
    }

    session.send(
        CMD_TRAVEL_BROCHURE_GET_DATA_SC_RSP,
        Ipogaccpagm {
            retcode: 0,
            impheidipgc: 2,
            biaghopnodp: elements,
            moippddaohm: HashMap::from([
                (223001u32, 1u32),
                (223005u32, 1u32),
                (223200u32, 1u32),
                (223714u32, 1u32),
                (223332u32, 1u32),
                (223274u32, 1u32),
                (223023u32, 1u32),
                (223152u32, 1u32),
                (223409u32, 1u32),
                (223731u32, 1u32),
                (223155u32, 1u32),
                (223732u32, 1u32),
                (223733u32, 1u32),
                (223734u32, 1u32),
                (223350u32, 1u32),
                (223735u32, 1u32),
                (223352u32, 1u32),
                (223353u32, 1u32),
                (223356u32, 1u32),
                (223106u32, 1u32),
                (223431u32, 1u32),
                (223432u32, 1u32),
                (223370u32, 1u32),
                (223434u32, 1u32),
                (223184u32, 1u32),
                (223504u32, 1u32),
                (223254u32, 1u32),
                (223255u32, 1u32),
            ])
        }
    ).await
}

pub async fn on_get_phone_data_cs_req(
    session: &PlayerSession,
    _body: &GetPhoneDataCsReq,
) -> Result<()> {
    let mut data = vec![];
    let mut data2 = vec![];
    for i in 0..6u32 {
        data.push(221000 + i);
        data2.push(220000 + i);
    }
    session.send(
        CMD_GET_PHONE_DATA_SC_RSP,
        GetPhoneDataScRsp {
            epoicbnlbgn: data2.clone(),
            paciklpabdl: data,
            ccmpnpbeipj: 221000,
            eacdbinnkjg: 220000,
            retcode: 0
        }
    ).await
}

pub async fn on_select_phone_theme_cs_req(
    session: &PlayerSession,
    body: &SelectPhoneThemeCsReq,
) -> Result<()> {
    session.send(
        CMD_SELECT_PHONE_THEME_SC_RSP,
        SelectPhoneThemeScRsp {
            ccmpnpbeipj: body.bganhiddedd,
            inhdnclkmjg: body.bganhiddedd,
            retcode: 0
        }
    ).await
}

pub async fn on_select_chat_bubble_cs_req(
    session: &PlayerSession,
    body: &SelectChatBubbleCsReq,
) -> Result<()> {
    session.send(
        CMD_SELECT_CHAT_BUBBLE_SC_RSP,
        SelectChatBubbleScRsp {
            eacdbinnkjg: body.minhcfcflbd,
            gcknobhoooa: body.minhcfcflbd,
            retcode: 0
        }
    ).await
}