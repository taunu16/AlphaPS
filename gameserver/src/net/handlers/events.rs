use super::*;

const ALLOWED_EVENT_TYPES: [i64; 8] = [
    20, //expeditions
    2, //7 day checkins (they dont work but they show up)
    221, //clock park (something new; clockie event)
    220, //evolve build panel (legends of galactic baseballer)
    //older ones
    210, //fever time challenge (tides of war)
    43, //drink maker
    47, //cosmodysey (wont start without server assistance)
    37, //dreamchaser bulletin
];

const EVENT_BLACKLIST: [u32; 2] = [
    10056, //newest 7 day check in (only one not working at all)
    30007, //medicine assignments
];

pub async fn on_get_activity_schedule_config_cs_req(
    session: &PlayerSession,
    _body: &GetActivityScheduleConfigCsReq
) -> Result<()> {
    let player_info = session.player_info();
    // let mut already_added_types = vec![/*6, 2*/];
    session.send(
        CMD_GET_ACTIVITY_SCHEDULE_CONFIG_SC_RSP, 
        GetActivityScheduleConfigScRsp {
            retcode: 0,
            activity_schedule_list: player_info.excel_manager.activity_panel
                .iter()
                .rev()
                .filter(|(_, v)| ALLOWED_EVENT_TYPES.contains(&v.activity_panel_type) && !EVENT_BLACKLIST.contains(&v.panel_id))
                /*.filter(|(_, v)| ![17/*some siluette*/, 27 /*aetherum wars*/].contains(&v.activity_panel_type) && !v.is_resident_panel)*/ 
                // .filter(|(_, v)| {
                //     if !already_added_types.contains(&v.activity_panel_type) {
                //         already_added_types.push(v.activity_panel_type);
                //         return true;
                //     }
                //     false
                // })
                .map(|(_, v)| ActivityScheduleInfo {
                    begin_time: 0,
                    end_time: i64::MAX,
                    bcnbcoijiao: v.panel_id,
                    nknoilmdemg: v.activity_module_id
                }
            ).collect()
            // activity_schedule_list: vec![
            //     ActivityScheduleInfo {
            //         begin_time: 0,
            //         end_time: i64::MAX,
            //         bcnbcoijiao: 50013,
            //         nknoilmdemg: 5001301
            //     }
            // ]
        }
    ).await
}