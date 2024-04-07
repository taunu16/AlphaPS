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

pub async fn on_phbnokkhgkd(
    session: &PlayerSession,
    _body: &Phbnokkhgkd,
) -> Result<()> {
    let mut elements = HashMap::new();

    for i in vec![101u32, 102, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14] {
        elements.insert(i, Bbjejalfhen {
            gfehjonbcmn: Nhnehbekhhj::PageUnlocked as i32,
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
            egigfgpjddg: if i < 9 || i > 100 {i} else {i+1},
            idgckihophm: if i == 101 {1} else if i == 102 {2} else {i+2}
        });
    }

    session.send(
        CMD_TRAVEL_BROCHURE_GET_DATA_SC_RSP,
        Ipogaccpagm {
            retcode: 0,
            impheidipgc: 0,
            biaghopnodp: elements,
            moippddaohm: HashMap::from([
                (101u32, 101u32),
                (1u32, 1u32),
                (2u32, 2u32),
                (3u32, 3u32),
                (4u32, 4u32),
                (5u32, 5u32),
                (6u32, 6u32),
                (7u32, 7u32),
                (8u32, 8u32),
                (9u32, 10u32),
                (10u32, 11u32),
                (11u32, 12u32),
                (12u32, 13u32),
                (13u32, 14u32),
                (14u32, 15u32),
            ])
        }
    ).await
}