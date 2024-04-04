use lazy_static::lazy_static;
use serde::Deserialize;
use serde_json::from_str;

const DEFAULT_GLOBALS: &str = include_str!("../../globals.json");

lazy_static! {
    pub static ref INSTANCE: Globals = {
        let local_config = std::path::Path::new("globals.json");
        let data = if local_config.exists() {
            std::fs::read_to_string("globals.json").unwrap()
        } else {
            let config = dirs::config_dir()
                .expect("No config directory found")
                .join("hkrpg-gameserver");

            std::fs::create_dir_all(&config).unwrap();

            let env = config.join("globals.json");

            if !env.exists() {
                std::fs::write(&env, DEFAULT_GLOBALS).unwrap();
            }

            std::fs::read_to_string(&env).unwrap()
        };

        from_str(&data).unwrap()
    };
}

#[derive(Deserialize)]
pub struct Globals {
    pub lineup: Vec<u32>,
    pub monster_wave_list: Vec<Vec<u32>>,
}