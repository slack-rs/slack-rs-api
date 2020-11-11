#![allow(unused_imports)]
#![allow(clippy::single_match)]
use crate::adapt::utils::*;
use crate::rust::{Method, Module};

mod session;

pub fn correct(module: &mut Module) {
    for mut module in &mut module.submodules {
        match module.name.as_str() {
            "session" => session::correct(&mut module),
            _ => {}
        }
    }

    for mut method in &mut module.methods {
        match method.name.as_str() {
            "assign" => correct_assign(&mut method),
            "invite" => correct_invite(&mut method),
            "list" => correct_list(&mut method),
            "remove" => correct_remove(&mut method),
            "setAdmin" => correct_set_admin(&mut method),
            "setExpiration" => correct_set_expiration(&mut method),
            "setOwner" => correct_set_owner(&mut method),
            "setRegular" => correct_set_regular(&mut method),
            _ => {}
        }
    }
}

fn correct_assign(_method: &mut Method) {}

fn correct_invite(_method: &mut Method) {}

fn correct_list(_method: &mut Method) {}

fn correct_remove(_method: &mut Method) {}

fn correct_set_admin(_method: &mut Method) {}

fn correct_set_expiration(_method: &mut Method) {}

fn correct_set_owner(_method: &mut Method) {}

fn correct_set_regular(_method: &mut Method) {}
