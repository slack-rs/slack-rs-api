#![allow(unused_imports)]
#![allow(clippy::single_match)]
use crate::adapt::utils::*;
use crate::rust::{Method, Module};

mod approved;
mod denied;

pub fn correct(module: &mut Module) {
    for mut module in &mut module.submodules {
        match module.name.as_str() {
            "approved" => approved::correct(&mut module),
            "denied" => denied::correct(&mut module),
            _ => {}
        }
    }

    for mut method in &mut module.methods {
        match method.name.as_str() {
            "approve" => correct_approve(&mut method),
            "deny" => correct_deny(&mut method),
            "list" => correct_list(&mut method),
            _ => {}
        }
    }
}

fn correct_approve(_method: &mut Method) {}

fn correct_deny(_method: &mut Method) {}

fn correct_list(_method: &mut Method) {}
