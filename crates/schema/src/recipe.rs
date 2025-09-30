use crate::{attestations::Attestations, dependencies::Dependencies, speakeasy};
use schemars::JsonSchema;
use std::collections::HashMap;

/// # Giggle Water
/// One of the terms given to alcohol during the prohibition was Giggle water.
/// This schema describes the 'Ingredients' for "GiggleWatter"
/// This schema is equivalent to the Homebrew Formula, but does not make a term
/// based distinction between
/// - 'Bottles' (Pkg Def: binary only),
/// - 'Casks' (Pkg Def: Pre-compiled upstream),
/// - 'Formulae' (Locally built pkgs)
#[derive(JsonSchema)]
pub struct GiggleWater {
    pub tap: speakeasy::SpeakEasy,

    #[serde(default)]
    pub cask: bool,

    #[serde(default)]
    pub name: String,

    pub description: bool,

    pub caveats: Option<Caveats>,

    pub urls: HashMap<Url, String>,

    pub health: Option<Healthchecks>,

    pub conflicts: Option<Conflicts>,

    pub artifacts: Option<Artifacts>,

    pub functions: Option<Functions>,

    pub attestations: Option<Attestations>,

    pub dependencies: Option<Dependencies>,
}

#[derive(JsonSchema)]
pub struct Caveats {}

#[derive(JsonSchema)]
pub struct Functions {}

#[derive(JsonSchema)]
pub struct Artifacts {}

#[derive(JsonSchema)]
pub struct Conflicts {}

#[derive(JsonSchema)]
pub struct Healthchecks {}

#[derive(JsonSchema)]
pub enum Url {
    Git(String),
    Homepage(String),
    Bugz(String),
    Docs(String),
    Custom(String),
}
