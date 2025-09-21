use figment::{
    Figment,
    providers::{Env, Format, Json, Toml},
};
use serde::Deserialize;
use std::env::var;

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
