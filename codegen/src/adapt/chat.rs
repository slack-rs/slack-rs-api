use super::set_parameters_required;
use crate::rust::{Method, Module};

pub fn correct(module: &mut Module) {
    for mut method in &mut module.methods {
        match method.name.as_str() {
            "delete" => correct_delete(&mut method),
            "meMessage" => correct_memessage(&mut method),
            "postMessage" => correct_postmessage(&mut method),
            "unfurl" => correct_unfurl(&mut method),
            _ => {}
        }
    }
}

fn correct_delete(method: &mut Method) {
    set_parameters_required(method, &["channel", "token", "ts"]);
}

fn correct_memessage(method: &mut Method) {
    set_parameters_required(method, &["channel", "token", "text"]);
}

fn correct_postmessage(method: &mut Method) {
    set_parameters_required(method, &["text"]);
}

fn correct_unfurl(method: &mut Method) {
    set_parameters_required(method, &["unfurls"]);
}
