use super::*;

fn get_archive_data() -> ArchiveData {
    ArchiveData {
        agdfngfgagl: vec![2, 1, 3, 7],
        kcjiblndbbg: vec![1, 3, 3, 7],
        lplhpdmoapn: vec![40349, 59504, 443],
        ..Default::default()
    }
}

pub async fn on_get_archive_data_cs_req(
    session: &PlayerSession,
    _body: &GetArchiveDataCsReq,
) -> Result<()> {
    session
        .send(
            CMD_GET_ARCHIVE_DATA_SC_RSP,
            GetArchiveDataScRsp {
                retcode: 0,
                archive_data: Some(get_archive_data())
            }
        ).await
}

pub async fn on_get_updated_archive_data_cs_req(
    session: &PlayerSession,
    _body: &GetUpdatedArchiveDataCsReq,
) -> Result<()> {
    session
        .send(
            CMD_GET_ARCHIVE_DATA_SC_RSP,
            GetUpdatedArchiveDataScRsp {
                retcode: 0,
                archive_data: Some(get_archive_data())
            }
        ).await
}

pub async fn on_get_jukebox_data_cs_req(
    session: &PlayerSession,
    _body: &GetJukeboxDataCsReq,
) -> Result<()> {
    session.send(
        CMD_GET_JUKEBOX_DATA_SC_RSP,
        GetJukeboxDataScRsp {
            retcode: 0,
            pldiejopafk: 210000,
            efnoehpidpi: session.player_info().excel_manager.background_music_config.iter().map(|(_, v)| 
                Hehaonbniao {
                    id: v.id,
                    group_id: v.group_id,
                    bbhkfblnbln: true 
                }
            ).collect()
        } 
    ).await
}