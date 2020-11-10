#![allow(unused_imports)]
#![allow(clippy::single_match)]
use crate::adapt::utils::*;
use crate::rust::{Method, Module};

pub fn correct(module: &mut Module) {
    for mut method in &mut module.methods {
        match method.name.as_str() {
            "messages" => correct_messages(&mut method),
            _ => {}
        }
    }
}

fn correct_messages(_method: &mut Method) {}
