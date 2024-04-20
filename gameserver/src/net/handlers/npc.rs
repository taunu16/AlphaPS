use crate::{excel::tools_res::PropState, safe_unwrap_result};

use super::*;

pub async fn on_bciendaonnn(
    session: &PlayerSession,
    body: &Bciendaonnn,
) -> Result<()> {
    println!("{:?}", body);

    session
        .send(
            CMD_GET_FIRST_TALK_BY_PERFORMANCE_NPC_SC_RSP,
            Bemeopenkpd {
                retcode: 0,
                apiaadfldbe: body.khjfgfhhchj.iter().map(|id| 
                    Hndfedalldc {
                        jmnkdpdjilg: *id,
                        apcaodelfcp: false
                    }
                ).collect()
            },
        )
        .await
}

//GetNpcTakenRewardCsReq
pub async fn on_get_npc_taken_reward_cs_req(
    session: &PlayerSession,
    body: &GetNpcTakenRewardCsReq,
) -> Result<()> {
    println!("{:?}", body);

    session
        .send(
            CMD_GET_NPC_TAKEN_REWARD_SC_RSP,
            GetNpcTakenRewardScRsp {
                retcode: 0,
                egeneneoadj: body.egeneneoadj,
                bekdcnobfeo: vec![]
            }
        ).await
}

//FinishFirstTalkByPerformanceNpcCsReq


//SubmitOrigamiItemCsReq
pub async fn on_submit_origami_item_cs_req(
    session: &PlayerSession,
    body: &SubmitOrigamiItemCsReq,
) -> Result<()> {
    println!("{:?}", body);
    session
        .send(
            CMD_SUBMIT_ORIGAMI_ITEM_SC_RSP,
            SubmitOrigamiItemScRsp {
                retcode: 0,
                gbjdobijaoi: body.gbjdobijaoi
            }
        ).await
}

//
pub async fn on_interact_prop_cs_req(
    session: &PlayerSession,
    body: &InteractPropCsReq
) -> Result<()> {
    let player_info = session.player_info();
    let mut state = PropState::Open;

    safe_unwrap_result!(session.send(
        CMD_SCENE_PLANE_EVENT_SC_NOTIFY, 
        Flcldcecndc {
            reward: Some(ItemList {
                item_list: vec![
                    Item {
                        item_id: 102,
                        num: 2137,
                        ..Default::default()
                    }
                ]
            })
        }
    ).await);

    if let Some(prop) = player_info.scene_prop_cache.get(&body.prop_entity_id) {
        state = match &prop.prop_state_list {
            x if x.contains(&PropState::Destructed) => PropState::Destructed,
            x if x.contains(&PropState::ChestUsed) => PropState::ChestUsed,
            x if x.contains(&PropState::EventOpen) => PropState::EventOpen,
            _ => PropState::Open
        };
        println!("KYYYS {:?}", state);

        safe_unwrap_result!(session.send(
            CMD_GROUP_STATE_CHANGE_SC_NOTIFY, 
            GroupStateChangeScNotify {
                group_state_info: Some(GroupStateInfo {
                    entry_id: player_info.position.entry_id,
                    group_id: prop.group_id,
                    emfppbjclgp: state.clone() as u32,
                    oekbjnjeedf: 2137,
                })
            }
        ).await);
    }

    session
        .send(
            CMD_INTERACT_PROP_SC_RSP,
            InteractPropScRsp {
                retcode: 0,
                prop_entity_id: body.prop_entity_id,
                prop_state: state as u32
            }
        ).await
}

//GetMainMissionCustomValueCsReq
pub async fn on_get_main_mission_custom_value_cs_req(
    session: &PlayerSession,
    body: &GetMainMissionCustomValueCsReq
) -> Result<()> {//println!("{:?}", body);
    session.send(
        CMD_GET_MAIN_MISSION_CUSTOM_VALUE_SC_RSP,
        GetMainMissionCustomValueScRsp {
            retcode: 0,
            mmmedgnoljo: body.sub_mission_id_list.iter().map(|a| Ebeeijpilmi {
                id: *a,
                status: 2,
                miadakiaoln:vec![]
            }).collect()
        }
    ).await
}

pub async fn on_get_first_talk_npc_cs_req(
    session: &PlayerSession,
    body: &GetFirstTalkNpcCsReq
) -> Result<()> {
    session.send(
        CMD_GET_FIRST_TALK_NPC_SC_RSP,
        Abojckcendm {
            retcode: 0,
            apiaadfldbe: body.aammpfgpknj.iter().map(|q| Oijcllopbih {
                ihbalhicnej: *q,
                ..Default::default()
            }).collect()
        }
    ).await
}