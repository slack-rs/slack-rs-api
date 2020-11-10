#![allow(unused_imports)]
#![allow(clippy::single_match)]
use crate::adapt::utils::*;
use crate::rust::{Method, Module};

mod users;

pub fn correct(module: &mut Module) {
    for mut module in &mut module.submodules {
        match module.name.as_str() {
            "users" => users::correct(&mut module),
            _ => {}
        }
    }

    for mut method in &mut module.methods {
        match method.name.as_str() {
            "create" => correct_create(&mut method),
            "disable" => correct_disable(&mut method),
            "enable" => correct_enable(&mut method),
            "list" => correct_list(&mut method),
            "update" => correct_update(&mut method),
            _ => {}
        }
    }
}

fn correct_create(_method: &mut Method) {}

fn correct_disable(_method: &mut Method) {}

fn correct_enable(_method: &mut Method) {}

fn correct_list(_method: &mut Method) {}

fn correct_update(_method: &mut Method) {}
