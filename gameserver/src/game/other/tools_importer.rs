use proto::CMD_PLAYER_SYNC_SC_NOTIFY;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::{game::{globals, inventory}, net::PlayerSession, util::load_json};

#[derive(Serialize, Deserialize)]
pub struct ToolsJson {
    avatars: HashMap<String, Avatar>,
    lightcones: Vec<Lightcone>,
    relics: Vec<Relic>,
    battle_config: BattleConfig,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Avatar {
    avatar_id: u32,
    internal_uid: u32,
    level: u32,
    promotion: u32,
    owner_uid: u32,
    data: Data,
    sp_max: u32,
    use_technique: Vec<u32>,
    sp_value: Option<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    rank: u32,
    skills: HashMap<u32, u32>,
}

#[derive(Serialize, Deserialize)]
pub struct BattleConfig {
    battle_type: String,
    blessings: Vec<Option<serde_json::Value>>,
    cycle_count: u32,
    monsters: Vec<Vec<Monster>>,
    path_resonance_id: u32,
    stage_id: u32,
    custom_stats: Vec<Option<serde_json::Value>>,
}

#[derive(Serialize, Deserialize)]
pub struct Monster {
    amount: u32,
    level: u32,
    #[serde(rename = "monsterId")]
    monster_id: u32,
    max_hp: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lightcone {
    equip_avatar: u32,
    item_id: u32,
    internal_uid: u32,
    level: u32,
    promotion: u32,
    rank: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Relic {
    level: u32,
    main_affix_id: u32,
    relic_id: u32,
    relic_set_id: u32,
    sub_affixes: Vec<SubAffix>,
    equip_avatar: u32,
    internal_uid: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAffix {
    sub_affix_id: u32,
    count: u32,
    step: u32,
}

pub async fn import(session: &PlayerSession, inventory: &mut inventory::Inventory) -> crate::Result<()> {
    let data: ToolsJson = load_json(dirs::config_dir().unwrap().join("hkrpg-gameserver").join("freesr-data.json"));

    inventory.lightcones = data.lightcones.iter().map(|lc| inventory::Lightcone {
        avatar: lc.equip_avatar,
        id: lc.item_id,
        level: lc.level,
        promotion: lc.promotion,
        rank: lc.rank,
        unique_id: lc.internal_uid 
    }).collect();

    inventory.relic = data.relics.iter().map(|relic| inventory::Relic {
        avatar: relic.equip_avatar,
        id: relic.relic_id,
        level: relic.level,
        main_affix_id: relic.main_affix_id,
        unique_id: relic.internal_uid,
        minor_affixes: relic.sub_affixes.iter().map(|aff| inventory::RelicAffix {
            id: aff.sub_affix_id,
            step: aff.step,
            cnt: aff.count
        }).collect()
    }).collect();

    inventory.avatars = data.avatars.values().map(|av| inventory::Avatar {
        id: av.avatar_id,
        level: av.level,
        promotion: av.promotion,
        eidolon: av.data.rank,
        lightcone_id: inventory.lightcones.iter().find(|lc| lc.avatar == av.avatar_id).map(|lc| lc.unique_id).unwrap_or(0),
        relics: relic_finder(&inventory.relic, av.avatar_id),
        skill_tree: av.data.skills.clone()
    }).collect();

    inventory.save_data();

    //monsters

    let globalsm = &mut globals.borrow_mut();
    globalsm.monster_wave_list = data.battle_config.monsters.iter().map(|a|a.iter().map(|mon| mon.monster_id).collect()).collect();
    globalsm.monster_levels = data.battle_config.monsters.iter().map(|a|a.first().map(|mon| mon.level).unwrap_or(80)).collect();
    globalsm.stage_id = data.battle_config.stage_id;
    globalsm.save();

    return session.send(CMD_PLAYER_SYNC_SC_NOTIFY, proto::PlayerSyncScNotify {
        equipment_list: inventory.lightcones_to_proto(),
        relic_list: inventory.relic_to_proto(),
        avatar_sync: Some(proto::AvatarSync {
            avatar_list: inventory.avatars_to_proto()
        }),
        ..Default::default()
    }).await
}

fn relic_finder(relics: &Vec<inventory::Relic>, id: u32) -> HashMap<u32, u32> {
    let mut ret: HashMap<u32, u32> = HashMap::new();

    for relic in relics {
        if relic.avatar != id {
            continue;
        }

        let slotid: u32 = relic.id.to_string().chars().last().unwrap().to_digit(10).unwrap();

        ret.insert(slotid, relic.unique_id);
    }
    
    ret
}