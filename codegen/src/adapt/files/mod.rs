#![allow(unused_imports)]
#![allow(clippy::single_match)]
use crate::adapt::utils::*;
use crate::rust::{Method, Module};

mod comments;
mod remote;

pub fn correct(module: &mut Module) {
    for mut module in &mut module.submodules {
        match module.name.as_str() {
            "comments" => comments::correct(&mut module),
            "remote" => remote::correct(&mut module),
            _ => {}
        }
    }

    for mut method in &mut module.methods {
        match method.name.as_str() {
            "delete" => correct_delete(&mut method),
            "info" => correct_info(&mut method),
            "list" => correct_list(&mut method),
            "revokePublicURL" => correct_revoke_public_url(&mut method),
            "sharedPublicURL" => correct_shared_public_url(&mut method),
            "upload" => correct_upload(&mut method),
            _ => {}
        }
    }
}

fn correct_delete(_method: &mut Method) {}

fn correct_info(_method: &mut Method) {}

fn correct_list(_method: &mut Method) {}

fn correct_revoke_public_url(_method: &mut Method) {}

fn correct_shared_public_url(_method: &mut Method) {}

fn correct_upload(_method: &mut Method) {}
