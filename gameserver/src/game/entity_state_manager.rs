use std::{collections::HashMap, fs::{self, File}, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{excel::tools_res::PropState, handle_error, handle_errorv};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct EntityStateManager {
    #[serde(default)]
    states: HashMap<u32, HashMap<u32, PropState>>,
}

impl EntityStateManager {
    pub fn new() -> Self {
        let mut ret: Self = Default::default();
        if !Self::get_save_path().join("..").exists() {
            handle_error!(fs::create_dir_all(Self::get_save_path().join("..")), "Config directory creation failed", ret);
        }

        if Self::get_save_path().exists() {
            let data = handle_error!(File::options().read(true).open(Self::get_save_path()), "Entitystates open failed", ret);
            ret = handle_error!(serde_json::from_reader(data), "Entitystates JSON parse failed", ret);
        }

        ret
    }

    fn get_save_path() -> PathBuf {
        dirs::config_dir().unwrap().join("hkrpg-gameserver").join("entitystates.json")
    }

    fn save_data(&self) {
        let mut data = handle_errorv!(File::options().write(true).create(true).truncate(true).open(Self::get_save_path()), "Entitystates JSON open failed");
        handle_errorv!(serde_json::to_writer(&mut data, self), "Entitystates JSON save failed");
    }

    pub fn clear_data(&mut self) {
        self.states.clear();
        self.save_data();
    }

    pub fn set_entity_state(&mut self, entry_id: u32, entity_id: u32, state: PropState) {
        if let Some(val) = self.states.get_mut(&entry_id) {
            val.insert(entity_id, state);
            self.save_data();
            return;
        }

        self.states.insert(entry_id, HashMap::from([(entity_id, state)]));
        self.save_data()
    }

    pub fn get_entity_state(&self, entry_id: u32, entity_id: u32) -> Option<&PropState> {
        if let Some(ent) = self.states.get(&entry_id) {
            ent.get(&entity_id)
        } else {
            None
        }
    }
}