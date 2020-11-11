#![allow(unused_imports)]
#![allow(clippy::single_match)]
use crate::adapt::utils::*;
use crate::rust::{Method, Module};

pub fn correct(module: &mut Module) {
    for mut method in &mut module.methods {
        match method.name.as_str() {
            "add" => correct_add(&mut method),
            "addAlias" => correct_add_alias(&mut method),
            "list" => correct_list(&mut method),
            "remove" => correct_remove(&mut method),
            "rename" => correct_rename(&mut method),
            _ => {}
        }
    }
}

fn correct_add(_method: &mut Method) {}

fn correct_add_alias(_method: &mut Method) {}

fn correct_list(_method: &mut Method) {}

fn correct_remove(_method: &mut Method) {}

fn correct_rename(_method: &mut Method) {}
