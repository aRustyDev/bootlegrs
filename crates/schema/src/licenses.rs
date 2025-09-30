use schemars::JsonSchema;

#[derive(JsonSchema)]
pub enum Licenses {
    Apache(String),
    Bsd(String),
    Unlicense(String),
    Agpl(String),
    Mit(String),
    Custom(String),
}
