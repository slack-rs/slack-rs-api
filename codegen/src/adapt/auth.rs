#![allow(unused_imports)]
#![allow(clippy::single_match)]
use crate::adapt::utils::*;
use crate::rust::{Method, Module};

pub fn correct(module: &mut Module) {
    for mut method in &mut module.methods {
        match method.name.as_str() {
            "revoke" => correct_revoke(&mut method),
            "test" => correct_test(&mut method),
            _ => {}
        }
    }
}

fn correct_revoke(_method: &mut Method) {}

fn correct_test(_method: &mut Method) {}
