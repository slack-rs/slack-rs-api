#![allow(unused_imports)]
#![allow(clippy::single_match)]
use crate::adapt::utils::*;
use crate::rust::{Method, Module};

mod event;
mod permissions;

pub fn correct(module: &mut Module) {
    for mut module in &mut module.submodules {
        match module.name.as_str() {
            "event" => event::correct(&mut module),
            "permissions" => permissions::correct(&mut module),
            _ => {}
        }
    }

    for mut method in &mut module.methods {
        match method.name.as_str() {
            "uninstall" => correct_uninstall(&mut method),
            _ => {}
        }
    }
}

fn correct_uninstall(_method: &mut Method) {}
