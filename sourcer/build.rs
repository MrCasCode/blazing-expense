// This file is executed before building the package with the goal of performing
// any pre-required steps just before compilation
// In this build file, we generate the models to make sure those are up to date
use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;
use std::fs::ReadDir;
use std::process::Command;
use std::{env, fs};


#[derive(Debug, Deserialize)]
struct DataSchema {
    input_dir: String,
    output_dir: String,
}

#[derive(Debug, Deserialize)]
struct BuildSettings{
    data_schema: DataSchema
}

impl BuildSettings {
    pub fn new() -> Result<Self, ConfigError> {
        let env = env::var("ENV").unwrap_or_else(|_| "local".into());

        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("env.default.toml"))
            // Add in the current environment file for any env overrides
            .add_source(File::with_name(&format!("env.{}.toml", env)).required(false))
            // Apply any user overrides through environment
            .add_source(Environment::default())
            .build()?;

        s.try_deserialize()
    }
}

fn get_schemas(directory: ReadDir) -> Vec<String> {
    let filenames: Vec<String> = directory
        .map(|entry| entry.unwrap().file_name()) // get file name
        .filter(|path| path.to_str().unwrap().ends_with(".json")) // filter files that end with .json
        .filter_map(|json_paths| json_paths.to_str().map(|json_str| json_str.to_string())) // convert to string
        .collect(); // collect the iterator into a collection, in this case a vector

    filenames
}

fn main() {
    let settings = BuildSettings::new().unwrap();

    let current_dir = env::current_dir().unwrap();
    let schemas_dir = fs::read_dir(current_dir.join(&settings.data_schema.input_dir)).unwrap();

    let schemas = get_schemas(schemas_dir);

    println!("schemas = {:?}", schemas);

    for schema in schemas {
        let schema_name = schema.split(".").next().unwrap().replace("_schema", "");
        let schema_output = settings.data_schema.output_dir.to_string() + "/" + schema_name.as_str() + ".rs";

        let output = Command::new("quicktype")
            .arg("-s")
            .arg("schema")
            .arg("schemas/".to_string() + schema.as_str())
            .arg("-o")
            .arg(&schema_output)
            .arg("--derive-debug")
            .output();

        match output {
            Ok(out) => println!("Successfully generated {}: {:?}", schema_output, out),
            Err(err) => println!("Failed to generate {}: {:?}", schema_output, err),
        }
    }

    println!("cargo:rerun-if-changed={}", settings.data_schema.input_dir);
    println!("cargo:rerun-if-changed={}", settings.data_schema.output_dir);
}
