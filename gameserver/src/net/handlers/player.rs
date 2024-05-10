use std::collections::HashMap;

use crate::{excel::EXCEL, util};

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
pub async fn on_travel_brochure_get_data_cs_req(
    session: &PlayerSession,
    _body: &TravelBrochureGetDataCsReq,
) -> Result<()> {
    let mut elements = HashMap::new();

    for i in vec![101u32, 102, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16] {
        elements.insert(i, Gnapkglaenf {
            mafmebchomj: TravelBrochurePageStatus::PageInteracted as i32,
            noicpkddpee: Cmdahlbmklc::PageDescCollapse as i32,
            pfoknkknace: vec![],
            mnboaolhcof: format!("{i}01").parse().unwrap(), //*ids.get(&i).unwrap_or(&0),
            inmjgoeaakb: i
        });
    }

    session.send(
        CMD_TRAVEL_BROCHURE_GET_DATA_SC_RSP,
        TravelBrochureGetDataScRsp {
            retcode: 0,
            lmfadlckloe: 2,
            nibokkaieid: elements,
            akphbfphkgc: HashMap::from_iter(EXCEL.item.paster_config.iter().map(|v| (v.id, 1u32)))
        }
    ).await
}

pub async fn on_get_phone_data_cs_req(
    session: &PlayerSession,
    _body: &GetPhoneDataCsReq,
) -> Result<()> {
    let mut data = vec![];
    let mut data2 = vec![];
    for i in 0..7u32 {
        data.push(221000 + i);
        data2.push(220000 + i);
    }
    session.send(
        CMD_GET_PHONE_DATA_SC_RSP,
        GetPhoneDataScRsp {
            mnclhkknffn: data,
            mpgkclipima: data2,
            danocgnclik: 220000,
            fipccimjcgb: 221000,
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
            fipccimjcgb: body.ihefcgoomem,
            njieglbjpmn: body.ihefcgoomem,
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
            cjcfcdccane: body.bfeleinblee,
            danocgnclik: body.bfeleinblee,
            retcode: 0
        }
    ).await
}