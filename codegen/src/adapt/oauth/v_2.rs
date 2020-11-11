#![allow(unused_imports)]
#![allow(clippy::single_match)]
use crate::adapt::utils::*;
use crate::rust::{Method, Module};

pub fn correct(module: &mut Module) {
    for mut method in &mut module.methods {
        match method.name.as_str() {
            "access" => correct_access(&mut method),
            _ => {}
        }
    }
}

fn correct_access(_method: &mut Method) {}
