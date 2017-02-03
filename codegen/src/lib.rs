#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate inflector;

use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

use inflector::Inflector;

mod json_schema;
use json_schema::{JsonSchema, PropType};

mod generator;
use generator::*;

const SCHEMA_DIR: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/slack-api-schemas");

pub fn generate_types(output_path: &Path) -> io::Result<()> {
    let codegen_filepath = output_path.join("types.rs");

    let mut types_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&codegen_filepath)?;

    types_file.write_all(b"use std::collections::HashMap;\n\n")?;

    let schema_path = Path::new(SCHEMA_DIR);

    for entry in fs::read_dir(schema_path.join("objects"))? {
        if let Ok(e) = entry {
            let path = e.path();
            if path.is_file() {
                let mut schema_file = File::open(&path)?;
                let mut schema_contents = String::new();
                schema_file.read_to_string(&mut schema_contents)?;

                let schema = serde_json::from_str::<JsonSchema>(&schema_contents)
                    .expect(&format!("Could not parse object schema for {}", path.display()));

                let ty_name = path.file_stem().unwrap().to_str().unwrap().to_pascal_case();

                let ty = match PropType::from_schema(&schema, &ty_name) {
                    PropType::Obj(ref o) => o.to_code(),
                    PropType::Enum(ref e) => e.to_code(),
                    _ => panic!("Object schema is not an object."),
                };

                types_file.write_all(ty.as_bytes())?;
            }
        }
    }

    Ok(())
}

pub fn generate_modules(output_path: &Path) -> io::Result<()> {
    let mut mods = vec![];

    let schema_path = Path::new(SCHEMA_DIR);

    for entry in fs::read_dir(schema_path.join("web"))? {
        if let Ok(e) = entry {
            let path = e.path();
            if path.is_file() {
                let mut schema_file = File::open(&path)?;
                let mut schema_contents = String::new();
                schema_file.read_to_string(&mut schema_contents)?;

                let module = serde_json::from_str::<Module>(&schema_contents)
                    .expect(&format!("Could not parse module schema for {}", path.display()));
                mods.push(module.get_safe_name());

                let out_filepath = output_path.join(format!("{}.rs", module.get_safe_name()));

                let mut out_file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(&out_filepath)?;

                out_file.write_all(module.generate().as_bytes())?;
            }
        }
    }

    Ok(())
}
