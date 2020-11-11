#![allow(unused_imports)]
#![allow(clippy::single_match)]
use crate::adapt::utils::*;
use crate::rust::{Method, Module};

pub fn correct(module: &mut Module) {
    for mut method in &mut module.methods {
        match method.name.as_str() {
            "info" => correct_info(&mut method),
            "setDefaultChannels" => correct_set_default_channels(&mut method),
            "setDescription" => correct_set_description(&mut method),
            "setDiscoverability" => correct_set_discoverability(&mut method),
            "setIcon" => correct_set_icon(&mut method),
            "setName" => correct_set_name(&mut method),
            _ => {}
        }
    }
}

fn correct_info(_method: &mut Method) {}

fn correct_set_default_channels(_method: &mut Method) {}

fn correct_set_description(_method: &mut Method) {}

fn correct_set_discoverability(_method: &mut Method) {}

fn correct_set_icon(_method: &mut Method) {}

fn correct_set_name(_method: &mut Method) {}
