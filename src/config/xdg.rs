use figment::{
    Figment,
    providers::{Env, Format, Json, Toml},
};
use serde::Deserialize;

const XDG_CFG: &str = env!("XDG_CONFIG_HOME");
const XDG_BIN: &str = env!("XDG_BIN_HOME");
const XDG_DATA: &str = env!("XDG_DATA_HOME");
const XDG_STATE: &str = env!("XDG_STATE_HOME");
const XDG_CACHE: &str = env!("XDG_CACHE_HOME");

const XDG_DATA_PATH: &str = env!("XDG_DATA_DIRS");
const XDG_RUNTIME_PATH: &str = env!("XDG_RUNTIME_DIRS");

enum Dir {
    Config(String),
    Bin(String),
    State(String),
    Cache(String),
    Data(String),
}

enum Path {
    Data,
    Runtime,
}
