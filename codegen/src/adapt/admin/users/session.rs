#![allow(unused_imports)]
#![allow(clippy::single_match)]
use crate::adapt::utils::*;
use crate::rust::{Method, Module};

pub fn correct(module: &mut Module) {
    for mut method in &mut module.methods {
        match method.name.as_str() {
            "invalidate" => correct_invalidate(&mut method),
            "reset" => correct_reset(&mut method),
            _ => {}
        }
    }
}

fn correct_invalidate(_method: &mut Method) {}

fn correct_reset(_method: &mut Method) {}
