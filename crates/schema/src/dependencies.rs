// use crate::{attestations, recipe, rotgut, speakeasy, still};
use schemars::JsonSchema;

#[derive(JsonSchema)]
pub struct Dependencies {
    pub platform: Vec<Dependency>,
    pub runtime: Vec<Dependency>,
    pub build: Vec<Dependency>,
    pub arch: Vec<Dependency>,
    pub os: Vec<Dependency>,
}

#[derive(JsonSchema)]
pub enum Dependency {
    Build(String),
    Platform(String),
    Arch(String),
    Os(String),
    Runtime(String),
}

#[derive(JsonSchema)]
pub struct BuildDependency {}

#[derive(JsonSchema)]
pub struct PlatformDependency {}

#[derive(JsonSchema)]
pub struct ArchDependency {}

#[derive(JsonSchema)]
pub struct OsDependency {}

#[derive(JsonSchema)]
pub struct RuntimeDependency {}
