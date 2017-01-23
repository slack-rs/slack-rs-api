extern crate slack_api_codegen;

use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR not specified");
    let out_path = Path::new(&out_dir).to_owned();

    let _ = slack_api_codegen::generate_types(&out_path);
    slack_api_codegen::run(&out_path);

    let codegen_dir = Path::new("codegen");

    // avoid unnecessary recompiles when used as a crates.io dependency
    if codegen_dir.exists() {
        println!("cargo:rerun-if-changed=codegen");
    }
}