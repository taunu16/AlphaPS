use crate::{excel::ExcelManager, net::PlayerSession};

use super::{commands::CommandSystem, globals, inventory::Inventory};
use anyhow::Result;
use proto::*;
use serde::{Deserialize, Serialize};

pub struct PlayerInfo {
    pub lineup: LineupInfo,
    pub inventory: Inventory,
    pub excel_manager: ExcelManager,
    pub command_system: CommandSystem,
    pub position: PlayerPosition
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub rot_y: i32,
    pub entry_id: u32,
    pub floor_id: u32,
    pub plane_id: u32,
}

impl Default for PlayerPosition {
    fn default() -> Self {
        Self { 
            x: 4480, 
            y: 19364, 
            z: -550, 
            rot_y: 0,

            entry_id: 2010101,
            plane_id: 20101,
            floor_id: 20101001
        }
    }
}


impl PlayerInfo {
    pub fn new() -> Self {
        Self {
            lineup: default_lineup(),
            inventory: Inventory::new(),
            excel_manager: ExcelManager::new(),
            command_system: Default::default(),
            position: Default::default()
        }
    }

    pub async fn sync_lineup(&self, session: &PlayerSession) -> Result<()> {
        session
            .send(
                CMD_SYNC_LINEUP_NOTIFY,
                SyncLineupNotify {
                    lineup: Some(self.lineup.clone()),
                    ..Default::default()
                },
            )
            .await
    }

    pub async fn sync_lineup_reason(&self, session: &PlayerSession, reasons: Vec<SyncLineupReason>) -> Result<()> {
        session
            .send(
                CMD_SYNC_LINEUP_NOTIFY,
                SyncLineupNotify {
                    lineup: Some(self.lineup.clone()),
                    reason_list: reasons.iter().map(|a| (*a).into()).collect(),
                    ..Default::default()
                },
            )
            .await
    }
}

fn default_lineup() -> LineupInfo {
    LineupInfo {
        plane_id: 10001,
        name: String::from("Lineup 1"),
        index: 0,
        leader_slot: 0,
        mp: 5,
        mp_max: 5,
        avatar_list: globals
            .lineup
            .iter()
            .enumerate()
            .map(|(idx, id)| LineupAvatar {
                id: *id,
                slot: idx as u32,
                hp: 10000,
                sp: Some(AmountInfo {
                    cur_amount: 10000,
                    max_amount: 10000,
                }),
                satiety: 100,
                avatar_type: 3,
            })
            .collect(),
        ..Default::default()
    }
}