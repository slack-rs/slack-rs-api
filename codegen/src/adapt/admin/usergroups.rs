#![allow(unused_imports)]
#![allow(clippy::single_match)]
use crate::adapt::utils::*;
use crate::rust::{Method, Module};

pub fn correct(module: &mut Module) {
    for mut method in &mut module.methods {
        match method.name.as_str() {
            "addChannels" => correct_add_channels(&mut method),
            "addTeams" => correct_add_teams(&mut method),
            "listChannels" => correct_list_channels(&mut method),
            "removeChannels" => correct_remove_channels(&mut method),
            _ => {}
        }
    }
}

fn correct_add_channels(_method: &mut Method) {}

fn correct_add_teams(_method: &mut Method) {}

fn correct_list_channels(_method: &mut Method) {}

fn correct_remove_channels(_method: &mut Method) {}
