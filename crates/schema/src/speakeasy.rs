// Equivalent to Taps
use schemars::JsonSchema;

#[derive(JsonSchema)]
pub struct SpeakEasy {
    domain: String,
    owner: String,
    repo: String,
    protocol: String,
    branches: Vec<String>,
    bottles: bool,
    games: bool,
}
