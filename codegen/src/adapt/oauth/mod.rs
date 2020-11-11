#![allow(unused_imports)]
#![allow(clippy::single_match)]
use crate::adapt::utils::*;
use crate::rust::{Method, Module};

mod v_2;

pub fn correct(module: &mut Module) {
    for mut module in &mut module.submodules {
        match module.name.as_str() {
            "v2" => v_2::correct(&mut module),
            _ => {}
        }
    }

    for mut method in &mut module.methods {
        match method.name.as_str() {
            "access" => correct_access(&mut method),
            "token" => correct_token(&mut method),
            _ => {}
        }
    }
}

fn correct_access(_method: &mut Method) {}

fn correct_token(_method: &mut Method) {}
