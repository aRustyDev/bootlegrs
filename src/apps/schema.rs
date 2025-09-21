use crate::cli::args;
use crate::config::lib;
use crate::schema::{recipe, rotgut, speakeasy, still};
use clap::ValueEnum;
use schemars::JsonSchema;
use schemars::generate::SchemaSettings;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, ValueEnum, JsonSchema, Serialize, Deserialize)]
pub enum SchemaType {
    Config,
    Recipe,
    RotGut,
    Still,
    SpeakEasy,
    All,
}

pub fn generate_schema(target: &SchemaType) {
    let settings = SchemaSettings::default().for_serialize();
    let generator = settings.into_generator();
    match target {
        SchemaType::Recipe => {
            let schema = generator.into_root_schema_for::<recipe::GiggleWater>();
            println!("{}", serde_json::to_string_pretty(&schema).unwrap());
        }
        SchemaType::RotGut => {
            let schema = generator.into_root_schema_for::<rotgut::RotGut>();
            println!("{}", serde_json::to_string_pretty(&schema).unwrap());
        }
        SchemaType::SpeakEasy => {
            let schema = generator.into_root_schema_for::<speakeasy::SpeakEasy>();
            println!("{}", serde_json::to_string_pretty(&schema).unwrap());
        }
        SchemaType::Still => {
            let schema = generator.into_root_schema_for::<still::Still>();
            println!("{}", serde_json::to_string_pretty(&schema).unwrap());
        }
        SchemaType::Config => {
            let schema = generator.clone().into_root_schema_for::<args::Args>();
            println!("{}", serde_json::to_string_pretty(&schema).unwrap());
            let schema = generator.clone().into_root_schema_for::<lib::Config>();
            println!("{}", serde_json::to_string_pretty(&schema).unwrap());
        }
        SchemaType::All => {
            println!("{:?}", generate_schema(&SchemaType::Recipe));
            println!("{:?}", generate_schema(&SchemaType::RotGut));
            println!("{:?}", generate_schema(&SchemaType::SpeakEasy));
            println!("{:?}", generate_schema(&SchemaType::Still));
            println!("{:?}", generate_schema(&SchemaType::Config));
        }
        _ => {
            println!("{:?}", "{'error':'no schema selected'}");
        }
    }
}
