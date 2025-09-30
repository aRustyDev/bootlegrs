use clap::Parser;
use schemars::JsonSchema;
use std::env;
use std::path::PathBuf;
// use figment::util::diff_paths;
use figment::{
    Figment,
    providers::{Env, Format, Json, Serialized, Toml, Yaml},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Parser, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Config {
    dot: Vec<String>,

    #[clap(short, long, value_parser, default_value_t = 1)]
    count: u8,
}

impl Config {
    pub fn load() -> Result<Self, figment::Error> {
        Self::figment().extract()
    }

    fn get_xdg_config_home() -> PathBuf {
        env::var("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                // Fallback to ~/.config if XDG_CONFIG_HOME not set
                dirs::home_dir()
                    .unwrap_or_else(|| PathBuf::from("."))
                    .join(".config")
            })
    }

    fn figment() -> Figment {
        let mut figment = Figment::new();
        let xdg_config = Self::get_xdg_config_home();

        // Config file locations in priority order (lowest to highest)
        let locations = [
            PathBuf::from("/etc/bootleg"),     // system
            xdg_config.join("bootleg"),        // user config
            PathBuf::from("./config/bootleg"), // local config dir
            PathBuf::from("./bootleg"),        // local file
        ];

        for location in locations {
            figment = figment
                .merge(Json::file(location.with_extension("json")).nested())
                .merge(Toml::file(location.with_extension("toml")).nested())
                .merge(Yaml::file(location.with_extension("yaml")).nested());
        }

        figment
            .merge(Env::prefixed("BOOTLEG_"))
            .merge(Env::prefixed("XDG_"))
            .merge(Env::raw().only(&[
                "HOME",
                "XDG_CONFIG_HOME",
                "XDG_BIN_HOME",
                "XDG_DATA_HOME",
                "XDG_STATE_HOME",
                "XDG_CACHE_HOME",
                "XDG_RUNTIME_DIRS",
                "XDG_DATA_DIRS",
            ]))
            .merge(Serialized::defaults(Config::parse()))
    }
}
