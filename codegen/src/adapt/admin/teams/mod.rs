#![allow(unused_imports)]
#![allow(clippy::single_match)]
use crate::adapt::utils::*;
use crate::rust::{Method, Module};

mod admins;
mod owners;
mod settings;

pub fn correct(module: &mut Module) {
    for mut module in &mut module.submodules {
        match module.name.as_str() {
            "admins" => admins::correct(&mut module),
            "owners" => owners::correct(&mut module),
            "settings" => settings::correct(&mut module),
            _ => {}
        }
    }

    for mut method in &mut module.methods {
        match method.name.as_str() {
            "create" => correct_create(&mut method),
            "list" => correct_list(&mut method),
            _ => {}
        }
    }
}

fn correct_create(_method: &mut Method) {}

fn correct_list(_method: &mut Method) {}
