use super::{set_parameters_required, ResponseTypeModifier};
use crate::rust::{Method, Module};

pub fn correct(module: &mut Module) {
    for mut method in &mut module.methods {
        match method.name.as_str() {
            "history" => correct_history(&mut method),
            "info" => correct_info(&mut method),
            "list" => correct_list(&mut method),
            _ => {}
        }
    }
}

fn correct_info(method: &mut Method) {
    set_parameters_required(method, &["channel", "token"]);
    let mut root = ResponseTypeModifier::from(method);

    // channel is defined as Vec<_> but should return a single _
    root.split()
        .member_type("channel")
        .set_to_inner(|inner| inner.vec_type());
}

fn correct_list(method: &mut Method) {
    set_parameters_required(method, &["token"]);
    let mut root = ResponseTypeModifier::from(method);

    // channels is defined as Vec<Vec<_>> but should return Vec<_>
    root.split()
        .member_type("channels")
        .vec_type()
        .set_to_inner(|inner| inner.vec_type());
}

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
