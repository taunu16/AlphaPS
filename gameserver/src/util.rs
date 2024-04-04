use std::{fs::File, path::Path, time::{SystemTime, UNIX_EPOCH}};
use serde::Deserialize;

use crate::handle_error;

#[macro_export]
macro_rules! safe_unwrap_result {
    ($a:expr) => {
        match $a {
            std::result::Result::Ok(_) => {},
            Err(e) => return Err(e)
        }
    };
}

#[macro_export]
macro_rules! safe_unwrap_option {
    ($a:expr) => {
        match $a {
            Some(a) => a,
            None => return Ok(())
        }
    };
}

#[macro_export]
macro_rules! find_by_id {
    ($a:expr, $f:expr) => {
        safe_unwrap_option!($a.iter().find(|a| a.id == $f))
    };
}
#[macro_export]
macro_rules! find_by_uid {
    ($a:expr, $f:expr) => {
        safe_unwrap_option!($a.iter().find(|a| a.unique_id == $f))
    };
}

#[macro_export]
macro_rules! find_i_by_id {
    ($a:expr, $f:expr) => {
        safe_unwrap_option!($a.iter().position(|a| a.id == $f))
    };
}
#[macro_export]
macro_rules! find_i_by_uid {
    ($a:expr, $f:expr) => {
        safe_unwrap_option!($a.iter().position(|a| a.unique_id == $f))
    };
}

#[macro_export]
macro_rules! handle_error {
    ($e:expr, $m:tt, $v:expr) => {
        match ($e) {
            Err(err) => {
                let e = format!("{}: {:?}", $m, ::anyhow::anyhow!(err));
                tracing::error!(error.message = %format!("{}", &e), "{:?}", e);
                return $v;
            },
            std::result::Result::Ok(v) => v
        }
    };
    ($e:expr, $m:tt) => {
        match ($e) {
            Err(err) => {
                let e = format!("{}: {:?}", $m, ::anyhow::anyhow!(err.clone()));
                tracing::error!(error.message = %format!("{}", &e), "{:?}", e);
                return Err(::anyhow::anyhow!(err));
            },
            std::result::Result::Ok(v) => v
        }
    };
}

#[macro_export]
macro_rules! handle_errorv {
    ($e:expr, $m:tt) => {
        match ($e) {
            Err(e) => {
                let e = format!("{}: {:?}", $m, ::anyhow::anyhow!(e));
                tracing::error!(error.message = %format!("{}", &e), "{:?}", e);
                return;
            },
            std::result::Result::Ok(v) => v
        }
    };
}

#[macro_export]
macro_rules! parse_u32 {
    ($a:expr) => {
        handle_error!($a.trim().parse::<u32>(), "u32 parse failed")
    };
}

#[macro_export]
macro_rules! parse_u32_def {
    ($a:expr, $b:tt) => {
        handle_error!($a.unwrap_or(&$b).trim().parse::<u32>(), "u32 parse failed")
    };
}

#[macro_export]
macro_rules! parse_i32 {
    ($a:expr) => {
        handle_error!($a.trim().parse::<i32>(), "u32 parse failed")
    };
}

pub fn load_json<T: for<'a> Deserialize<'a>, P: AsRef<Path>>(path: P) -> T {
    let file = File::options().read(true).open(path).expect("JSON open failed");

    serde_json::from_reader(file).unwrap()
}

pub fn cur_timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}