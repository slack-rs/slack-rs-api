#![allow(unused_imports)]
#![allow(clippy::single_match)]
use crate::adapt::utils::*;
use crate::rust::{Method, Module};

mod ekm;
mod restrict_access;

pub fn correct(module: &mut Module) {
    for mut module in &mut module.submodules {
        match module.name.as_str() {
            "ekm" => ekm::correct(&mut module),
            "restrictAccess" => restrict_access::correct(&mut module),
            _ => {}
        }
    }

    for mut method in &mut module.methods {
        match method.name.as_str() {
            "archive" => correct_archive(&mut method),
            "convertToPrivate" => correct_convert_to_private(&mut method),
            "create" => correct_create(&mut method),
            "delete" => correct_delete(&mut method),
            "disconnectShared" => correct_disconnect_shared(&mut method),
            "getConversationPrefs" => correct_get_conversation_prefs(&mut method),
            "getTeams" => correct_get_teams(&mut method),
            "invite" => correct_invite(&mut method),
            "rename" => correct_rename(&mut method),
            "search" => correct_search(&mut method),
            "setConversationPrefs" => correct_set_conversation_prefs(&mut method),
            "setTeams" => correct_set_teams(&mut method),
            "unarchive" => correct_unarchive(&mut method),
            _ => {}
        }
    }
}

fn correct_archive(_method: &mut Method) {}

fn correct_convert_to_private(_method: &mut Method) {}

fn correct_create(_method: &mut Method) {}

fn correct_delete(_method: &mut Method) {}

fn correct_disconnect_shared(_method: &mut Method) {}

fn correct_get_conversation_prefs(_method: &mut Method) {}

fn correct_get_teams(_method: &mut Method) {}

fn correct_invite(_method: &mut Method) {}

fn correct_rename(_method: &mut Method) {}

fn correct_search(_method: &mut Method) {}

fn correct_set_conversation_prefs(_method: &mut Method) {}

fn correct_set_teams(_method: &mut Method) {}

fn correct_unarchive(_method: &mut Method) {}
