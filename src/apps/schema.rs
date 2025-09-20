use crate::schema::{recipe, rotgut, speakeasy, still};
use schemars::generate::SchemaSettings;

pub fn generate_schema(target: &String) {
    // By default, generated schemas describe how types are deserialized.
    // So we modify the settings here to instead generate schemas describing how it's serialized:
    let settings = SchemaSettings::default().for_serialize();
    let generator = settings.into_generator();
    match target.to_lowercase().as_str() {
        "recipe" => {
            let schema = generator.into_root_schema_for::<recipe::GiggleWater>();
            println!("{}", serde_json::to_string_pretty(&schema).unwrap());
        }
        "rotgut" => {
            let schema = generator.into_root_schema_for::<rotgut::RotGut>();
            println!("{}", serde_json::to_string_pretty(&schema).unwrap());
        }
        "speakeasy" => {
            let schema = generator.into_root_schema_for::<speakeasy::SpeakEasy>();
            println!("{}", serde_json::to_string_pretty(&schema).unwrap());
        }
        "still" => {
            let schema = generator.into_root_schema_for::<still::Still>();
            println!("{}", serde_json::to_string_pretty(&schema).unwrap());
        }
        "all" => {
            println!("{:?}", generate_schema(&"recipe".to_lowercase()));
            println!("{:?}", generate_schema(&"rotgut".to_lowercase()));
            println!("{:?}", generate_schema(&"speakeasy".to_lowercase()));
            println!("{:?}", generate_schema(&"still".to_lowercase()));
        }
        _ => {
            println!("{:?}", "{'error':'no schema selected'}");
        }
    }
}
