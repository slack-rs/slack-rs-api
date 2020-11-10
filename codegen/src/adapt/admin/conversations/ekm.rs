#![allow(unused_imports)]
#![allow(clippy::single_match)]
use crate::adapt::utils::*;
use crate::rust::{Method, Module};

pub fn correct(module: &mut Module) {
    for mut method in &mut module.methods {
        match method.name.as_str() {
            "listOriginalConnectedChannelInfo" => {
                correct_list_original_connected_channel_info(&mut method)
            }
            _ => {}
        }
    }
}

fn correct_list_original_connected_channel_info(_method: &mut Method) {}
