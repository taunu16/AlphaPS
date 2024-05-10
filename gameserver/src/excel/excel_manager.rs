use lazy_static::lazy_static;
use paste::paste;
use super::{types::*, Item};

macro_rules! excel_parse {
    ($($name:ident),* $(,)*) => {
        paste! {
        $(
            const [<$name:snake:upper _JSON>]: &str = include_str!(concat!("./data/ExcelOutput/", stringify!($name), ".json"));
        )*

        pub struct ExcelManager {
            pub item: Item,
            $(
                 pub [<$name:snake>]: $name,
            )*
        }

        impl ExcelManager {
            pub fn new() -> Self {
                Self {
                    item: Item::new(),
                    $(
                        [<$name:snake>]: serde_json::from_str([<$name:snake:upper _JSON>]).expect((&(stringify!($name).to_owned() + " parse failed"))),
                    )*
                }
            }
        }
    }
    };
}

excel_parse! {
    ActivityPanel,
    BackgroundMusicConfig,
    Interact,
    StageConfig,
    ChallengeMazeConfig,
    MainMissionConfig,
}

lazy_static! {
    pub static ref EXCEL: ExcelManager = ExcelManager::new();
}