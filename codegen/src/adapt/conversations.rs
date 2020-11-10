#![allow(unused_imports)]
#![allow(clippy::single_match)]
use crate::adapt::utils::*;
use crate::rust::{Method, Module};

pub fn correct(module: &mut Module) {
    for mut method in &mut module.methods {
        match method.name.as_str() {
            "archive" => correct_archive(&mut method),
            "close" => correct_close(&mut method),
            "create" => correct_create(&mut method),
            "history" => correct_history(&mut method),
            "info" => correct_info(&mut method),
            "invite" => correct_invite(&mut method),
            "join" => correct_join(&mut method),
            "kick" => correct_kick(&mut method),
            "leave" => correct_leave(&mut method),
            "list" => correct_list(&mut method),
            "mark" => correct_mark(&mut method),
            "members" => correct_members(&mut method),
            "open" => correct_open(&mut method),
            "rename" => correct_rename(&mut method),
            "replies" => correct_replies(&mut method),
            "setPurpose" => correct_set_purpose(&mut method),
            "setTopic" => correct_set_topic(&mut method),
            "unarchive" => correct_unarchive(&mut method),
            _ => {}
        }
    }
}

fn correct_archive(_method: &mut Method) {}

fn correct_close(_method: &mut Method) {}

fn correct_create(_method: &mut Method) {}

fn correct_history(method: &mut Method) {
    set_parameters_required(method, &["channel", "token"]);
    let mut root = ResponseTypeModifier::from(method);

    // messages.attachments.id is not required
    root.split()
        .member_type("messages")
        .vec_type()
        .member_type("attachments")
        .vec_type()
        .member("id")
        .required(false);

    // messages.bot_id is defined as Vec<_> but should return a single _
    root.split()
        .member_type("messages")
        .vec_type()
        .member("bot_id")
        .required(false)
        .r#type()
        .set_to_inner(|inner| inner.vec_type());

    // messages can be null
    root.split().member("messages").required(false);

    // channel_actions_ts can be null
    root.split().member("channel_actions_ts").required(false);
}

fn correct_info(method: &mut Method) {
    set_parameters_required(method, &["channel", "token"]);
    let mut root = ResponseTypeModifier::from(method);

    // channel is defined as Vec<_> but should return a single _
    root.split()
        .member_type("channel")
        .set_to_inner(|inner| inner.vec_type());
}

fn correct_invite(_method: &mut Method) {}

fn correct_join(_method: &mut Method) {}

fn correct_kick(_method: &mut Method) {}

fn correct_leave(_method: &mut Method) {}

fn correct_list(method: &mut Method) {
    set_parameters_required(method, &["token"]);
    let mut root = ResponseTypeModifier::from(method);

    // channels is defined as Vec<Vec<_>> but should return Vec<_>
    root.split()
        .member_type("channels")
        .vec_type()
        .set_to_inner(|inner| inner.vec_type());
}

fn correct_mark(_method: &mut Method) {}

fn correct_members(_method: &mut Method) {}

fn correct_open(_method: &mut Method) {}

fn correct_rename(_method: &mut Method) {}

fn correct_replies(_method: &mut Method) {}

fn correct_set_purpose(_method: &mut Method) {}

fn correct_set_topic(_method: &mut Method) {}

fn correct_unarchive(_method: &mut Method) {}
