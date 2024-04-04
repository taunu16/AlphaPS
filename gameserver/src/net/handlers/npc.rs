use crate::excel::tools_res::PropState;

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
                        ..Default::default()
                    }
                ).collect()
            },
        )
        .await
}

//GetNpcTakenRewardCsReq
pub async fn on_ofepjkfdlaj(
    session: &PlayerSession,
    body: &Ofepjkfdlaj,
) -> Result<()> {
    println!("{:?}", body);

    session
        .send(
            CMD_GET_NPC_TAKEN_REWARD_SC_RSP,
            Ifemohljpaj {
                retcode: 0,
                egeneneoadj: body.egeneneoadj,
                bekdcnobfeo: vec![]
            }
        ).await
}

//FinishFirstTalkByPerformanceNpcCsReq


//SubmitOrigamiItemCsReq
pub async fn on_igocjbeekjp(
    session: &PlayerSession,
    body: &Igocjbeekjp,
) -> Result<()> {
    println!("{:?}", body);
    session
        .send(
            CMD_SUBMIT_ORIGAMI_ITEM_SC_RSP,
            Ddlcmokabnh {
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
    session
        .send(
            CMD_INTERACT_PROP_CS_REQ,
            InteractPropScRsp {
                retcode: 0,
                prop_entity_id: body.prop_entity_id,
                prop_state: PropState::Open as u32
            }
        ).await
}

//GetMainMissionCustomValueCsReq