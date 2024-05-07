use const_concat::const_concat;
use std::env::current_dir;

// pub const IS_DEV: bool = cfg!(debug_assertions);// cfg!(debug_assertions);
// pub const PUBLIC_PATH: &str = if IS_DEV {
//     "D:/Rust/jarvis-app/public"
// } else {
//     "./public"
// };

pub const COMMANDS_PATH: &str = "commands/";

pub const DB_FILE_NAME: &str = "app.db";
pub const APP_VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
pub const AUTHOR_NAME: Option<&str> = option_env!("CARGO_PKG_AUTHORS");
pub const REPOSITORY_LINK: Option<&str> = option_env!("CARGO_PKG_REPOSITORY");

// pub const VOSK_MODEL_PATH: &str = const_concat!(PUBLIC_PATH, "/vosk/model_small");
pub const VOSK_MODEL_PATH: &str = "vosk/model_small";

pub const CMD_RATIO_THRESHOLD: f64 = 60f64;
pub const CMS_WAIT_DELAY: std::time::Duration = std::time::Duration::from_secs(10);

pub const ASSISTANT_GREET_PHRASES: [&str; 3] = ["greet1", "greet2", "greet3"];
pub const ASSISTANT_PHRASES_TBR: [&str; 16] = [
    "сэр",
    "слушаю сэр",
    "всегда к услугам",
    "произнеси",
    "ответь",
    "покажи",
    "скажи",
    "давай",
    "да сэр",
    "к вашим услугам сэр",
    "всегда к вашим услугам сэр",
    "запрос выполнен сэр",
    "выполнен сэр",
    "есть",
    "загружаю сэр",
    "очень тонкое замечание сэр",
];
