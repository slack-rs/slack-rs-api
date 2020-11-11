#![allow(unused_imports)]
#![allow(clippy::single_match)]
use crate::adapt::utils::*;
use crate::rust::{Method, Module};

pub fn correct(module: &mut Module) {
    for mut method in &mut module.methods {
        match method.name.as_str() {
            "add" => correct_add(&mut method),
            "complete" => correct_complete(&mut method),
            "delete" => correct_delete(&mut method),
            "info" => correct_info(&mut method),
            "list" => correct_list(&mut method),
            _ => {}
        }
    }
}

fn correct_add(_method: &mut Method) {}

fn correct_complete(_method: &mut Method) {}

fn correct_delete(_method: &mut Method) {}

fn correct_info(_method: &mut Method) {}

fn correct_list(_method: &mut Method) {}
