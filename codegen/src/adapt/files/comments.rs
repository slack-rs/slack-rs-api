#![allow(unused_imports)]
#![allow(clippy::single_match)]
use crate::adapt::utils::*;
use crate::rust::{Method, Module};

pub fn correct(module: &mut Module) {
    for mut method in &mut module.methods {
        match method.name.as_str() {
            "delete" => correct_delete(&mut method),
            _ => {}
        }
    }
}

fn correct_delete(_method: &mut Method) {}
