// use tracing::instrument::WithSubscriber;

use crate::safe_unwrap_result;

use super::*;

// const UNLOCKED_AVATARS: [u32; 49] = [
//     8005, 1001, 1002, 1003, 1004, 1005, 1006, 1008, 1009, 1013, 1101, 1102, 1103, 1104, 1105, 1106,
//     1107, 1108, 1109, 1110, 1111, 1112, 1201, 1202, 1203, 1204, 1205, 1206, 1207, 1208, 1209, 1210,
//     1211, 1212, 1213, 1214, 1215, 1217, 1301, 1302, 1303, 1304, 1305, 1306, 1307, 1308, 1309, 1312,
//     1315,
// ];

pub async fn on_get_avatar_data_cs_req(
    session: &PlayerSession,
    body: &GetAvatarDataCsReq,
) -> Result<()> {
    session
        .send(
            CMD_GET_AVATAR_DATA_SC_RSP,
            GetAvatarDataScRsp {
                retcode: 0,
                is_all: body.is_get_all,
                avatar_list: session.player_info().inventory.avatars_to_proto(),
                // avatar_list: UNLOCKED_AVATARS
                //     .iter()
                //     .map(|id| Avatar {
                //         base_avatar_id: *id,
                //         level: 80,
                //         promotion: 6,
                //         rank: 6,
                //         ..Default::default()
                //     })
                //     .collect(),
                ..Default::default()
            },
        )
        .await
}

pub async fn on_dress_avatar_cs_req(
    session: &PlayerSession,
    body: &DressAvatarCsReq,
) -> Result<()> {
    let inventory = &mut session.player_info_mut().inventory;
    safe_unwrap_result!(inventory.equip_lightcone(session, body.imhlbinfhlh, body.equipment_unique_id).await);
    session
        .send(
            CMD_DRESS_AVATAR_SC_RSP,
            DressAvatarScRsp {
                retcode: 0
            }
        ).await
}

pub async fn on_dress_relic_avatar_cs_req(
    session: &PlayerSession,
    body: &DressRelicAvatarCsReq,
) -> Result<()> {
    println!("{:?}", body);
    let inventory = &mut session.player_info_mut().inventory;
    safe_unwrap_result!(inventory.equip_relic(session, body.imhlbinfhlh, body.param_list.clone()).await);
    session
        .send(
            CMD_DRESS_RELIC_AVATAR_SC_RSP,
            DressRelicAvatarScRsp {
                retcode: 0
            }
        ).await
}

pub async fn on_take_off_equipment_cs_req(
    session: &PlayerSession,
    body: &TakeOffEquipmentCsReq,
) -> Result<()> {
    let inventory = &mut session.player_info_mut().inventory;
    safe_unwrap_result!(inventory.equip_lightcone(session, body.imhlbinfhlh, 0).await);
    session
        .send(
            CMD_TAKE_OFF_EQUIPMENT_SC_RSP,
            TakeOffEquipmentScRsp {
                retcode: 0
            }
        ).await
}

pub async fn on_take_off_relic_cs_req(
    session: &PlayerSession,
    body: &TakeOffRelicCsReq,
) -> Result<()> {
    let inventory = &mut session.player_info_mut().inventory;
    safe_unwrap_result!(inventory.equip_relic(session, body.imhlbinfhlh, body.slot_list.iter().map(|sl| RelicParam { slot: *sl, relic_unique_id: 0 }).collect()).await);
    session
        .send(
            CMD_TAKE_OFF_RELIC_SC_RSP,
            TakeOffRelicScRsp {
                retcode: 0
            }
        ).await
}

pub async fn on_scene_cast_skill_cost_mp_cs_req(
    session: &PlayerSession,
    _body: &SceneCastSkillCostMpCsReq,
) -> Result<()> {
    session.player_info_mut().lineup.mp -= 1;
    session
        .send(
            CMD_SCENE_CAST_SKILL_COST_MP_SC_RSP,
            SceneCastSkillCostMpCsReq {}
        ).await

}

pub async fn on_scene_cast_skill_cs_req(
    session: &PlayerSession,
    body: &SceneCastSkillCsReq,
) -> Result<()> {
    session
        .send(
            CMD_SCENE_CAST_SKILL_SC_RSP,
            SceneCastSkillScRsp {
                retcode: 0,
                elgjckaejld: body.elgjckaejld,
                pbgpinglheg: vec![
                    Kaijnnbaieb {
                        ..Default::default()
                    }
                ],
                ..Default::default()
            }
        ).await

}