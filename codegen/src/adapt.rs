#![allow(clippy::single_match)]

use crate::rust::{Method, Module, ResponseType};

pub fn correct(modules: &mut [Module]) {
    for module in modules {
        match module.name.as_str() {
            "conversations" => correct_conversations(module),
            _ => {}
        }
    }
}

fn correct_conversations(module: &mut Module) {
    for mut method in &mut module.methods {
        match method.name.as_str() {
            "history" => correct_conversations_history(&mut method),
            "info" => correct_conversations_info(&mut method),
            "list" => correct_conversations_list(&mut method),
            _ => {}
        }
    }
}

fn correct_conversations_info(method: &mut Method) {
    for mut param in &mut method.parameters {
        match param.name.as_str() {
            // The channel parameter is required
            "channel" => param.required = true,
            // The Token parameter is required
            "token" => param.required = true,
            _ => {}
        }
    }
    if let ResponseType::Object(o) = &mut method.response.r#type {
        for t in o {
            match t.name.as_str() {
                // channel is defined as Vec<_> but returns a single _
                "channel" => {
                    if let ResponseType::Vec(v) = &mut t.r#type.r#type {
                        t.r#type = (**v).clone();
                    }
                }
                _ => {}
            }
        }
    }
}

fn correct_conversations_list(method: &mut Method) {
    for mut param in &mut method.parameters {
        match param.name.as_str() {
            // The Token parameter is required
            "token" => param.required = true,
            _ => {}
        }
    }
    if let ResponseType::Object(o) = &mut method.response.r#type {
        for t in o {
            match t.name.as_str() {
                // channels is defined as Vec<Vec<_>> but the endpoint returns Vec<_>
                "channels" => dedup_vec(&mut t.r#type.r#type),
                _ => {}
            }
        }
    }
}

fn correct_conversations_history(method: &mut Method) {
    for mut param in &mut method.parameters {
        match param.name.as_str() {
            // The channel parameter is required
            "channel" => param.required = true,
            // The Token parameter is required
            "token" => param.required = true,
            _ => {}
        }
    }
    if let ResponseType::Object(o) = &mut method.response.r#type {
        for t in o {
            match t.name.as_str() {
                // messages can be null
                "messages" => {
                    t.r#type.required = false;
                    if let ResponseType::Vec(v) = &mut t.r#type.r#type {
                        if let ResponseType::Object(o) = &mut v.r#type {
                            for t in o {
                                match t.name.as_str() {
                                    // attachments is not a vec
                                    "attachments" => {
                                        if let ResponseType::Vec(v) = &mut t.r#type.r#type {
                                            if let ResponseType::Object(o) = &mut v.r#type {
                                                for t in o {
                                                    match t.name.as_str() {
                                                        // id is not required
                                                        "id" => t.r#type.required = false,
                                                        _ => {}
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    // bot_id is not a vec
                                    "bot_id" => {
                                        if let ResponseType::Vec(v) = &t.r#type.r#type {
                                            t.r#type = (**v).clone();
                                            t.r#type.required = false;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
                // channel_actions_ts can be null
                "channel_actions_ts" => t.r#type.required = false,
                _ => {}
            }
        }
    }
}

fn dedup_vec(r#type: &mut ResponseType) {
    if let ResponseType::Vec(v) = r#type {
        if let ResponseType::Vec(vi) = &v.r#type {
            *v = vi.clone();
        }
    }
}
