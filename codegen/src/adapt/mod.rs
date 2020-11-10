#![allow(clippy::single_match)]

mod chat;
mod conversations;

use crate::rust::{Module, ResponseType};

pub fn correct(modules: &mut [Module]) {
    for module in modules {
        match module.name.as_str() {
            "chat" => chat::correct(module),
            "conversations" => conversations::correct(module),
            _ => {}
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
