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
    for mut param in &mut method.parameters {
        match param.name.as_str() {
            // The channel parameter is required
            "channel" => param.required = true,
            // The Token parameter is required
            "token" => param.required = true,
            // The ts parameter is required
            "ts" => param.required = true,
            _ => {}
        }
    }
}

fn correct_memessage(method: &mut Method) {
    for mut param in &mut method.parameters {
        match param.name.as_str() {
            // The channel parameter is required
            "channel" => param.required = true,
            // The Token parameter is required
            "token" => param.required = true,
            // The text parameter is required
            "text" => param.required = true,
            _ => {}
        }
    }
}

fn correct_postmessage(method: &mut Method) {
    for mut param in &mut method.parameters {
        match param.name.as_str() {
            // The text parameter is required
            "text" => param.required = true,
            _ => {}
        }
    }
}

fn correct_unfurl(method: &mut Method) {
    for mut param in &mut method.parameters {
        match param.name.as_str() {
            // The unfurls parameter is required
            "unfurls" => param.required = true,
            _ => {}
        }
    }
}
