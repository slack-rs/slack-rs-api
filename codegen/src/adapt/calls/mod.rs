#![allow(unused_imports)]
#![allow(clippy::single_match)]
use crate::adapt::utils::*;
use crate::rust::{Method, Module};

mod participants;

pub fn correct(module: &mut Module) {
    for mut module in &mut module.submodules {
        match module.name.as_str() {
            "participants" => participants::correct(&mut module),
            _ => {}
        }
    }

    for mut method in &mut module.methods {
        match method.name.as_str() {
            "add" => correct_add(&mut method),
            "end" => correct_end(&mut method),
            "info" => correct_info(&mut method),
            "update" => correct_update(&mut method),
            _ => {}
        }
    }
}

fn correct_add(_method: &mut Method) {}

fn correct_end(_method: &mut Method) {}

fn correct_info(_method: &mut Method) {}

fn correct_update(_method: &mut Method) {}
