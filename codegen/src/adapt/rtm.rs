#![allow(unused_imports)]
#![allow(clippy::single_match)]
use crate::adapt::utils::*;
use crate::rust::{Method, Module};

pub fn correct(module: &mut Module) {
    for mut method in &mut module.methods {
        match method.name.as_str() {
            "connect" => correct_connect(&mut method),
            _ => {}
        }
    }
}

fn correct_connect(_method: &mut Method) {}
