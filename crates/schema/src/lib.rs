pub mod attestations;
pub mod dependencies;
pub mod licenses;
pub mod recipe;
pub mod rotgut;
pub mod speakeasy;
pub mod still;
pub mod config;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use clap::ValueEnum;

#[derive(Debug, Clone, Eq, PartialEq, ValueEnum, JsonSchema, Serialize, Deserialize)]
pub enum SchemaType {
    Config,
    Recipe,
    RotGut,
    Still,
    SpeakEasy,
    All,
}
