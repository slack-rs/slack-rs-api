#![allow(unused_imports)]
#![allow(clippy::single_match)]
use crate::adapt::utils::*;
use crate::rust::{Method, Module};

mod resources;
mod scopes;
mod users;

pub fn correct(module: &mut Module) {
    for mut module in &mut module.submodules {
        match module.name.as_str() {
            "resources" => resources::correct(&mut module),
            "scopes" => scopes::correct(&mut module),
            "users" => users::correct(&mut module),
            _ => {}
        }
    }

    for mut method in &mut module.methods {
        match method.name.as_str() {
            "info" => correct_info(&mut method),
            "request" => correct_request(&mut method),
            _ => {}
        }
    }
}

fn correct_info(_method: &mut Method) {}

fn correct_request(_method: &mut Method) {}
