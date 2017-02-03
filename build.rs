extern crate slack_api_codegen;

use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR not specified");
    let out_path = Path::new(&out_dir).to_owned();

    slack_api_codegen::generate_types(&out_path).expect("Failed to generate Slack API types");
    slack_api_codegen::run(&out_path);

    let codegen_dir = Path::new("codegen");

    if codegen_dir.exists() {
        println!("cargo:rerun-if-changed=codegen");
    }
}