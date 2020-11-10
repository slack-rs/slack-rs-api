#![allow(unused_imports)]
#![allow(clippy::single_match)]
use crate::adapt::utils::*;
use crate::rust::{Method, Module};

pub fn correct(module: &mut Module) {
    for mut method in &mut module.methods {
        match method.name.as_str() {
            "open" => correct_open(&mut method),
            "publish" => correct_publish(&mut method),
            "push" => correct_push(&mut method),
            "update" => correct_update(&mut method),
            _ => {}
        }
    }
}

fn correct_open(_method: &mut Method) {}

fn correct_publish(_method: &mut Method) {}

fn correct_push(_method: &mut Method) {}

fn correct_update(_method: &mut Method) {}
