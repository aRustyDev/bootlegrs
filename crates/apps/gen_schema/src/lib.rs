use schema::{config::Config, recipe, rotgut, speakeasy, still, SchemaType};
use schemars::generate::SchemaSettings;

#[allow(unreachable_patterns)]
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
            let schema = generator.clone().into_root_schema_for::<Config>();
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
