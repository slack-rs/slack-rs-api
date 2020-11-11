#![allow(unused_imports)]
#![allow(clippy::single_match)]
use crate::adapt::utils::*;
use crate::rust::{Method, Module};

mod scheduled_messages;

pub fn correct(module: &mut Module) {
    for mut module in &mut module.submodules {
        match module.name.as_str() {
            "scheduledMessages" => scheduled_messages::correct(&mut module),
            _ => {}
        }
    }

    for mut method in &mut module.methods {
        match method.name.as_str() {
            "delete" => correct_delete(&mut method),
            "deleteScheduledMessage" => correct_delete_scheduled_message(&mut method),
            "getPermalink" => correct_get_permalink(&mut method),
            "meMessage" => correct_me_message(&mut method),
            "postEphemeral" => correct_post_ephemeral(&mut method),
            "postMessage" => correct_post_message(&mut method),
            "scheduleMessage" => correct_schedule_message(&mut method),
            "unfurl" => correct_unfurl(&mut method),
            "update" => correct_update(&mut method),
            _ => {}
        }
    }
}

fn correct_delete(method: &mut Method) {
    set_parameters_required(method, &["channel", "token", "ts"]);
}

fn correct_delete_scheduled_message(_method: &mut Method) {}

fn correct_get_permalink(_method: &mut Method) {}

fn correct_me_message(method: &mut Method) {
    set_parameters_required(method, &["channel", "token", "text"]);
}

fn correct_post_ephemeral(_method: &mut Method) {}

fn correct_post_message(method: &mut Method) {
    set_parameters_required(method, &["text"]);
}

fn correct_schedule_message(_method: &mut Method) {}

fn correct_unfurl(method: &mut Method) {
    set_parameters_required(method, &["unfurls"]);
}

fn correct_update(_method: &mut Method) {}
