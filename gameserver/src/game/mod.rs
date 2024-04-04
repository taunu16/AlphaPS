mod global_config;
mod player_info;
pub mod constants;
pub mod other;
pub mod inventory;
pub mod commands;

pub use global_config::INSTANCE as globals;
pub use player_info::PlayerInfo;