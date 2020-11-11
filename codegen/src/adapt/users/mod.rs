#![allow(unused_imports)]
#![allow(clippy::single_match)]
use crate::adapt::utils::*;
use crate::rust::{Method, Module};

mod profile;

pub fn correct(module: &mut Module) {
    for mut module in &mut module.submodules {
        match module.name.as_str() {
            "profile" => profile::correct(&mut module),
            _ => {}
        }
    }

    for mut method in &mut module.methods {
        match method.name.as_str() {
            "conversations" => correct_conversations(&mut method),
            "deletePhoto" => correct_delete_photo(&mut method),
            "getPresence" => correct_get_presence(&mut method),
            "identity" => correct_identity(&mut method),
            "info" => correct_info(&mut method),
            "list" => correct_list(&mut method),
            "lookupByEmail" => correct_lookup_by_email(&mut method),
            "setActive" => correct_set_active(&mut method),
            "setPhoto" => correct_set_photo(&mut method),
            "setPresence" => correct_set_presence(&mut method),
            _ => {}
        }
    }
}

fn correct_conversations(_method: &mut Method) {}

fn correct_delete_photo(_method: &mut Method) {}

fn correct_get_presence(_method: &mut Method) {}

fn correct_identity(_method: &mut Method) {}

fn correct_info(_method: &mut Method) {}

fn correct_list(_method: &mut Method) {}

fn correct_lookup_by_email(_method: &mut Method) {}

fn correct_set_active(_method: &mut Method) {}

fn correct_set_photo(_method: &mut Method) {}

fn correct_set_presence(_method: &mut Method) {}
