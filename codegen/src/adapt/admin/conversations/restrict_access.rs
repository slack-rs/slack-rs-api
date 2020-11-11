#![allow(unused_imports)]
#![allow(clippy::single_match)]
use crate::adapt::utils::*;
use crate::rust::{Method, Module};

pub fn correct(module: &mut Module) {
    for mut method in &mut module.methods {
        match method.name.as_str() {
            "addGroup" => correct_add_group(&mut method),
            "listGroups" => correct_list_groups(&mut method),
            "removeGroup" => correct_remove_group(&mut method),
            _ => {}
        }
    }
}

fn correct_add_group(_method: &mut Method) {}

fn correct_list_groups(_method: &mut Method) {}

fn correct_remove_group(_method: &mut Method) {}
