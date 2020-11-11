#![allow(unused_imports)]
#![allow(clippy::single_match)]
use crate::adapt::utils::*;
use crate::rust::{Method, Module};

mod apps;
mod conversations;
mod emoji;
mod invite_requests;
mod teams;
mod usergroups;
mod users;

pub fn correct(module: &mut Module) {
    for mut module in &mut module.submodules {
        match module.name.as_str() {
            "apps" => apps::correct(&mut module),
            "conversations" => conversations::correct(&mut module),
            "emoji" => emoji::correct(&mut module),
            "inviteRequests" => invite_requests::correct(&mut module),
            "teams" => teams::correct(&mut module),
            "usergroups" => usergroups::correct(&mut module),
            "users" => users::correct(&mut module),
            _ => {}
        }
    }
}
