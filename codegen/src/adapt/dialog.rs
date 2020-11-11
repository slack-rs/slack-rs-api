#![allow(unused_imports)]
#![allow(clippy::single_match)]
use crate::adapt::utils::*;
use crate::rust::{Method, Module};

pub fn correct(module: &mut Module) {
    for mut method in &mut module.methods {
        match method.name.as_str() {
            "open" => correct_open(&mut method),
            _ => {}
        }
    }
}

fn correct_open(_method: &mut Method) {}
