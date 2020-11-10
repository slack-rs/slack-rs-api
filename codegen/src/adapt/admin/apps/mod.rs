#![allow(unused_imports)]
#![allow(clippy::single_match)]
use crate::adapt::utils::*;
use crate::rust::{Method, Module};

mod approved;
mod requests;
mod restricted;

pub fn correct(module: &mut Module) {
    for mut module in &mut module.submodules {
        match module.name.as_str() {
            "approved" => approved::correct(&mut module),
            "requests" => requests::correct(&mut module),
            "restricted" => restricted::correct(&mut module),
            _ => {}
        }
    }

    for mut method in &mut module.methods {
        match method.name.as_str() {
            "approve" => correct_approve(&mut method),
            "restrict" => correct_restrict(&mut method),
            _ => {}
        }
    }
}

fn correct_approve(_method: &mut Method) {}

fn correct_restrict(_method: &mut Method) {}
