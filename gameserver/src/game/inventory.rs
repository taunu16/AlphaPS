use std::{collections::HashMap, fs::{self, File}, path::PathBuf};

use anyhow::Ok;
use proto::{AmountInfo, AvatarSync, CMD_PLAYER_SYNC_SC_NOTIFY};
use serde::{Deserialize, Serialize};

use crate::{find_by_id, find_by_uid, find_i_by_id, find_i_by_uid, handle_error, handle_errorv, net::PlayerSession, safe_unwrap_option};

use super::{constants, PlayerInfo};

const UNLOCKED_AVATARS: [u32; 49] = [
    8001, 1001, 1002, 1003, 1004, 1005, 1006, 1008, 1009, 1013, 1101, 1102, 1103, 1104, 1105, 1106,
    1107, 1108, 1109, 1110, 1111, 1112, 1201, 1202, 1203, 1204, 1205, 1206, 1207, 1208, 1209, 1210,
    1211, 1212, 1213, 1214, 1215, 1217, 1301, 1302, 1303, 1304, 1305, 1306, 1307, 1308, 1309, 1312,
    1315,
];

macro_rules! sync_avatar_info {
    ($s:expr, $v:expr) => {
        return $s.send(CMD_PLAYER_SYNC_SC_NOTIFY, proto::PlayerSyncScNotify {
            avatar_sync: Some(AvatarSync { avatar_list: vec![$v]}),
            ..Default::default()
        }).await
    };
}

macro_rules! sync_info {
    ($s:expr, $k:ident, $v:expr) => {
        return $s.send(CMD_PLAYER_SYNC_SC_NOTIFY, proto::PlayerSyncScNotify {
            $k: vec![$v],
            ..Default::default()
        }).await
    };
    ($s:expr, $k1:ident, $v1:expr, $k2:ident, $v2:expr) => {
        return $s.send(CMD_PLAYER_SYNC_SC_NOTIFY, proto::PlayerSyncScNotify {
            $k1: vec![$v1],
            $k2: vec![$v2],
            ..Default::default()
        }).await
    };
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Inventory {
    #[serde(default)]
    pub items: Vec<Item>,
    #[serde(default)]
    pub relic: Vec<Relic>,
    #[serde(default)]
    pub lightcones: Vec<Lightcone>,
    #[serde(default = "default_avatars")] //why someone thought it was a good idea (i was debugging it for an hour)
    pub avatars: Vec<Avatar>,
}

fn default_avatars() -> Vec<Avatar> {
    UNLOCKED_AVATARS.map(|av| Avatar {
        id: av,
        promotion: 6,
        level: 80,
        lightcone_id: 0,
        eidolon: 0,
        skill_tree: HashMap::new(),
        relics: HashMap::new(),
    }).to_vec()
}

impl Default for Inventory {
    fn default() -> Self {
        Self { 
            items: Default::default(), 
            relic: Default::default(), 
            lightcones: Default::default(), 
            avatars: default_avatars()
        }
    }
}

impl Inventory {
    pub fn new() -> Self {
        let mut inv: Inventory = Default::default();
        if !Self::get_save_path().join("..").exists() {
            handle_error!(fs::create_dir_all(Self::get_save_path().join("..")), "Config directory creation failed", inv);
        }

        if Self::get_save_path().exists() {
            let data = handle_error!(File::options().read(true).open(Self::get_save_path()), "Inventory open failed", inv);
            inv = handle_error!(serde_json::from_reader(data), "Inventory JSON parse failed", inv);
        }

        inv
    }

    fn get_save_path() -> PathBuf {
        dirs::config_dir().unwrap().join("hkrpg-gameserver").join("inventory.json")
    }

    pub(crate) fn find_free_id(&self) -> u32 {
        // let mut sort = self.relic.clone();
        // sort.sort_by(|a,b| b.unique_id.partial_cmp(&a.unique_id).unwrap());
        // sort[0].unique_id + 1
        let mut i = 1;
        let _binding = vec![self.relic.iter().map(|a| a.unique_id).collect::<Vec<u32>>(), self.lightcones.iter().map(|a| a.unique_id).collect::<Vec<u32>>()];
        let mut list = _binding.iter().flat_map(|f| f.iter()).collect::<Vec<_>>();
        loop {
            if let Some(pos) = list.iter().position(|a| **a == i) {
                list.remove(pos);
            } else {
                return i;
            }
            i += 1;
        }
    }

    pub fn save_data(&self) {
        let mut data = handle_errorv!(File::options().write(true).create(true).truncate(true).open(Self::get_save_path()), "Inventory JSON open failed");
        handle_errorv!(serde_json::to_writer_pretty(&mut data, self), "Inventory JSON save failed");
    }

    pub fn create_battle_lineup(&self, player_info: &PlayerInfo) -> Vec<proto::BattleAvatar> {
        let mut ret: Vec<proto::BattleAvatar> = vec![];

        for av in player_info.lineup.avatar_list.clone() {
            if let Some(av) = self.avatars.iter().find(|a| a.id == av.id) {
                ret.push(av.to_battle_proto(&player_info))
            }
        }
        
        ret
    }

    pub async fn add_relic(&mut self, session: &PlayerSession, id: u32, level: u32, main_affix_id: u32, minor_affixes: Vec<RelicAffix>) -> std::result::Result<(), anyhow::Error> {
        let relic = Relic {
            id, level, main_affix_id, minor_affixes,
            unique_id: self.find_free_id(),
            avatar: 0
        };
        self.relic.push(relic.clone());
        self.save_data();

        sync_info!(session, relic_list, relic.to_proto())
    }

    pub async fn add_lightcone(&mut self, session: &PlayerSession, id: u32, level: u32, rank: u32, promotion: u32) -> std::result::Result<(), anyhow::Error> {
        let lightcone = Lightcone {
            id, level, rank, promotion,
            unique_id: self.find_free_id(),
            avatar: 0
        };
        self.lightcones.push(lightcone.clone());
        self.save_data();

        sync_info!(session, equipment_list, lightcone.to_proto())
    }

    pub async fn add_item(&mut self, session: &PlayerSession, id: u32, count: u32) -> std::result::Result<(), anyhow::Error> {
        let mut item = Item {id, count};
        if let Some(idx) = self.items.iter().position(|a| a.id == id)  {
            self.items[idx].count += count;
            item = self.items[idx].clone();
        } else {
            self.items.push(item.clone());
        }
        self.save_data();

        sync_info!(session, material_list, item.to_proto())
    }

    pub async fn take_item(&mut self, session: &PlayerSession, id: u32, count: u32) -> std::result::Result<(), anyhow::Error> {
        if let Some(idx) = self.items.iter().position(|a| a.id == id)  {
            if self.items[idx].count > count {
                self.items[idx].count -= count; 
            } else {
                self.items[idx].count = 0; 
            }
            
            self.save_data();

            sync_info!(session, material_list, self.items[idx].to_proto())
        }
        Ok(())
    }

    pub async fn add_avatar(&mut self, session: &PlayerSession, id: u32, rank: u32, eidolons: u32, level: u32) -> std::result::Result<(), anyhow::Error> {
        let avatar = Avatar {
            id, eidolon: rank, promotion: eidolons, level,
            lightcone_id: 0,
            skill_tree: HashMap::new(),
            relics: HashMap::new(),
        };
        if let Some(idx) = self.avatars.iter().position(|a| a.id == id) {
            self.avatars[idx] = avatar.clone();
        } else {
            self.avatars.push(avatar.clone());
        }
        
        self.save_data();

        sync_avatar_info!(session, avatar.to_proto())
    }

    pub async fn set_avatar_skilltree(&mut self, session: &PlayerSession, avatar: u32, id: u32, val: u32) -> std::result::Result<(), anyhow::Error> {
        self.set_avatar_skilltree_to_map(session, avatar, vec![(id, val)]).await
    }

    pub async fn set_avatar_skilltree_to_map(&mut self, session: &PlayerSession, avatar: u32, map: Vec<(u32, u32)>) -> std::result::Result<(), anyhow::Error> {
        if let Some(idx) = self.avatars.iter().position(|a| a.id == avatar) {
            self.avatars[idx].skill_tree = HashMap::new();
            for (k, v) in map {
                self.avatars[idx].skill_tree.insert(avatar * 1000 + k, v);
            }
            self.save_data();

            sync_avatar_info!(session, self.avatars[idx].to_proto());
        }
        Ok(())
    }

    /*pub async fn equip_lightcone(&mut self, session: &PlayerSession, unique_id: u32, avatar: u32) -> std::result::Result<(), anyhow::Error> {
        if let Some(i) = self.lightcones.iter().position(|lc| lc.unique_id == unique_id) {
            let mut avatar_list = vec![
                proto::Avatar {
                    equipment_unique_id: unique_id,
                    base_avatar_id: avatar,
                    ..Default::default()
                }
            ];

            let mut lightcone_swap_id = 0;

            if let Some(idx) = self.avatars.iter().position(|a| a.id == avatar) {
                lightcone_swap_id = self.avatars[idx].lightcone_id;
                self.avatars[idx].lightcone_id = unique_id;

                if let Some(lidx) = self.lightcones.iter().position(|a| a.id == lightcone_swap_id) {
                    self.lightcones[lidx].avatar = self.lightcones[i].avatar;
                }
            }

            if self.lightcones[i].avatar != 0 && self.lightcones[i].avatar != avatar {
                if let Some(idx) = self.avatars.iter().position(|a| a.id == self.lightcones[i].avatar) {
                    self.avatars[idx].lightcone_id = lightcone_swap_id;
                }

                avatar_list.push(proto::Avatar {
                    equipment_unique_id: lightcone_swap_id,
                    base_avatar_id: self.lightcones[i].avatar,
                    ..Default::default()
                });
            }

            self.lightcones[i].avatar = avatar;
            self.save_data();

            return session.send(CMD_PLAYER_SYNC_SC_NOTIFY, proto::PlayerSyncScNotify {
                equipment_list: vec![self.lightcones[i].to_proto()],
                avatar_sync: Some(AvatarSync {
                    avatar_list
                }),
                ..Default::default()
            }).await;
        } 
        Ok(())
    }*/

    pub async fn equip_lightcone(&mut self, session: &PlayerSession, avatar: u32, id: u32) -> std::result::Result<(), anyhow::Error> {
        let avatar_idx = find_i_by_id!(self.avatars, avatar);
        let mut update_avatar_idx: Vec<usize> = vec![avatar_idx];
        let mut update_cones_idx: Vec<usize> = vec![];
        
        if id == 0 {
            if self.avatars[avatar_idx].lightcone_id != 0 {
                let idx = find_i_by_uid!(self.lightcones, self.avatars[avatar_idx].lightcone_id);
                self.lightcones[idx].avatar = 0;
                update_cones_idx.push(idx);
            }
            self.avatars[avatar_idx].lightcone_id = 0;
            return Ok(());
        }


        if let Some(subavatar_idx) = self.avatars.iter().position(|a| a.lightcone_id == id) {
            let avatarsc = self.avatars.clone();
            self.avatars[subavatar_idx].lightcone_id = avatarsc[avatar_idx].lightcone_id;
            if !update_avatar_idx.contains(&subavatar_idx) {
                update_avatar_idx.push(subavatar_idx);
            }
        }

        self.avatars[avatar_idx].lightcone_id = id;

        let cone_idx = find_i_by_uid!(self.lightcones, id);

        if self.avatars[avatar_idx].lightcone_id != 0 {
            let cone_self_idx = find_i_by_uid!(self.lightcones, self.avatars[avatar_idx].lightcone_id);

            let swapv = self.lightcones[cone_self_idx].avatar;
            self.lightcones[cone_self_idx].avatar = self.lightcones[cone_idx].avatar;
            self.lightcones[cone_idx].avatar = swapv;

            update_cones_idx.extend(vec![cone_idx, cone_self_idx]);
        } else {
            self.lightcones[cone_idx].avatar = avatar;
            update_cones_idx.push(cone_idx);
        }

        self.avatars[avatar_idx].lightcone_id = id;

        self.save_data();
        return session.send(CMD_PLAYER_SYNC_SC_NOTIFY, proto::PlayerSyncScNotify {
            equipment_list: update_cones_idx.iter().map(|i| self.lightcones[*i].to_proto()).collect(),
            avatar_sync: Some(AvatarSync {
                avatar_list: update_avatar_idx.iter().map(|i| self.avatars[*i].to_proto()).collect()
            }),
            ..Default::default()
        }).await
    }

    pub async fn equip_relic(&mut self, session: &PlayerSession, avatar: u32, params: Vec<proto::RelicParam>) -> std::result::Result<(), anyhow::Error> {
        let avatar_idx = find_i_by_id!(self.avatars, avatar);
        let mut update_avatar_idx: Vec<usize> = vec![avatar_idx];
        let mut update_relic_idx: Vec<usize> = vec![];
        
        for param in params {
            if param.relic_unique_id == 0 {
                if let Some(relic_id) = self.avatars[avatar_idx].relics.get(&param.slot) {
                    let idx = find_i_by_uid!(self.relic, *relic_id);
                    self.relic[idx].avatar = 0;
                    update_relic_idx.push(idx);
                }
                self.avatars[avatar_idx].relics.remove(&param.slot);
                continue;
            }

            if let Some(subavatar_idx) = self.avatars.iter().position(|a| a.relics.get(&param.slot) == Some(&param.relic_unique_id)) {
                let avatarsc = self.avatars.clone();
                self.avatars[subavatar_idx].relics.insert(param.slot, *avatarsc[avatar_idx].relics.get(&param.slot).unwrap_or(&0));
                if !update_avatar_idx.contains(&subavatar_idx) {
                    update_avatar_idx.push(subavatar_idx);
                }
            }

            let relic_idx = find_i_by_uid!(self.relic, param.relic_unique_id);

            if let Some(relic_self_id) = self.avatars[avatar_idx].relics.get(&param.slot) {
                let relic_self_idx = find_i_by_uid!(self.relic, *relic_self_id);

                let swapv = self.relic[relic_self_idx].avatar;
                self.relic[relic_self_idx].avatar = self.relic[relic_idx].avatar;
                self.relic[relic_idx].avatar = swapv;

                update_relic_idx.extend(vec![relic_idx, relic_self_idx]);
            } else {
                self.relic[relic_idx].avatar = avatar;
                update_relic_idx.push(relic_idx);
            }

            self.avatars[avatar_idx].relics.insert(param.slot, param.relic_unique_id);
        }

        self.save_data();
        return session.send(CMD_PLAYER_SYNC_SC_NOTIFY, proto::PlayerSyncScNotify {
            relic_list: update_relic_idx.iter().map(|i| self.relic[*i].to_proto()).collect(),
            avatar_sync: Some(AvatarSync {
                avatar_list: update_avatar_idx.iter().map(|i| self.avatars[*i].to_proto()).collect()
            }),
            ..Default::default()
        }).await
    }

    pub async fn give_all_items(&mut self, session: &PlayerSession, count: u32) -> std::result::Result<(), anyhow::Error> {
        self.items = constants::ALL_ITEMS.map(|id|  Item {
            count, id
        }).to_vec();

        self.save_data();

        return session.send(CMD_PLAYER_SYNC_SC_NOTIFY, proto::PlayerSyncScNotify {
            material_list: self.items_to_proto(),
            ..Default::default()
        }).await
    }

    pub fn relic_to_proto(&self) -> Vec<proto::Relic> {
        self.relic.iter().map(|r| r.to_proto()).collect()
    }

    pub fn items_to_proto(&self) -> Vec<proto::Material> {
        self.items.iter().map(|r| r.to_proto()).collect()
    }

    pub fn lightcones_to_proto(&self) -> Vec<proto::Equipment> {
        self.lightcones.iter().map(|r| r.to_proto()).collect()
    }

    pub fn avatars_to_proto(&self) -> Vec<proto::Avatar> {
        self.avatars.iter().map(|r| r.to_proto()).collect()
    }
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct Avatar {
    pub id: u32,
    pub lightcone_id: u32,
    pub eidolon: u32,
    pub promotion: u32,
    pub level: u32,
    #[serde(default)]
    pub skill_tree: HashMap<u32, u32>,
    #[serde(default)]
    pub relics: HashMap<u32, u32>
}

impl Avatar {
    pub fn to_proto(&self) -> proto::Avatar {
        proto::Avatar {
            base_avatar_id: self.id,
            equipment_unique_id: self.lightcone_id,
            rank: self.eidolon,
            promotion: self.promotion,
            level: self.level,
            skilltree_list: self.skill_tree.iter().map(|(k, v)| proto::AvatarSkillTree {
                point_id: *k,
                level: *v
            }).collect(),
            amafpakcckf: self.relics.iter().map(|(slot, idx)| proto::EquipRelic {
                ipnhjoomhdm: *slot,
                relic_unique_id: *idx
            }).collect(),
            ..Default::default()
        }
    }

    pub fn to_battle_proto(&self, player_info: &PlayerInfo) -> proto::BattleAvatar {
        let mut equipment_list: Vec<proto::BattleEquipment> = vec![];

        if let Some(lc) = player_info.inventory.lightcones.iter().find(|a| a.avatar == self.id) {
            equipment_list.push(lc.to_battle_proto());
        }

        proto::BattleAvatar {
            index: player_info.lineup.avatar_list.iter().position(|a| a.id == self.id).unwrap_or(5) as u32,
            id: self.id,
            equipment_list,
            rank: self.eidolon,
            promotion: self.promotion,
            level: self.level,
            hp: 10000,
            sp: Some(AmountInfo {
                cur_amount: 10000,
                max_amount: 10000
            }),
            skilltree_list: self.skill_tree.iter().map(|(k, v)| proto::AvatarSkillTree {
                point_id: *k,
                level: *v
            }).collect(),
            relic_list: self.relics.iter().map(|(_, v)| player_info.inventory.relic.iter().find(|a| a.unique_id == *v).unwrap_or(&Default::default()).to_battle_proto()).collect(),
            ..Default::default()
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct Lightcone {
    pub unique_id: u32,
    pub id: u32,
    pub rank: u32,
    pub level: u32,
    pub promotion: u32,
    pub avatar: u32,
}

impl Lightcone {
    pub fn to_proto(&self) -> proto::Equipment {
        proto::Equipment {
            tid: self.id,
            imhlbinfhlh: self.avatar,
            level: self.level,
            promotion: self.promotion,
            rank: self.rank,
            unique_id: self.unique_id,
            ..Default::default()
        }
    }

    pub fn to_battle_proto(&self) -> proto::BattleEquipment {
        proto::BattleEquipment {
            id: self.id,
            level: self.level,
            promotion: self.promotion,
            rank: self.rank
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct Item {
    pub id: u32,
    pub count: u32
}

impl Item {
    pub fn to_proto(&self) -> proto::Material {
        proto::Material {
            tid: self.id,
            num: self.count,
            ..Default::default()
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct Relic {
    pub unique_id: u32,
    pub avatar: u32,
    pub id: u32,
    pub level: u32,
    pub main_affix_id: u32,
    pub minor_affixes: Vec<RelicAffix>
}

impl Relic {
    pub fn to_proto(&self) -> proto::Relic {
        proto::Relic { 
            base_avatar_id: self.avatar, 
            level: self.level, 
            is_protected: false, 
            exp: 0, 
            kjgemkmeikm: false, 
            tid: self.id, 
            main_affix_id: self.main_affix_id, 
            unique_id: self.unique_id, 
            imhlbinfhlh: self.avatar,
            sub_affix_list: self.minor_affixes.iter().map(|a| a.to_proto()).collect(),
            ..Default::default() 
        }
    }

    pub fn to_battle_proto(&self) -> proto::BattleRelic {
        proto::BattleRelic { 
            id: self.id,
            level: self.level,
            main_affix_id: self.main_affix_id,
            unique_id: self.unique_id,
            sub_affix_list: self.minor_affixes.iter().map(|a| a.to_proto()).collect(),
            ..Default::default() 
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct RelicAffix {
    pub id: u32,
    pub step: u32,
    pub cnt: u32
}

impl RelicAffix {
    pub fn to_proto(&self) -> proto::RelicAffix {
        proto::RelicAffix {
            affix_id: self.id, 
            step: self.step, 
            cnt: self.cnt, 
            ..Default::default()
        }
    }
}