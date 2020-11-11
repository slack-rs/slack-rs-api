#![allow(unused_imports)]
#![allow(clippy::single_match)]
use crate::adapt::utils::*;
use crate::rust::{Method, Module, Parameter, ParameterDataType};

mod approved;
mod requests;
mod restricted;

pub fn correct(module: &mut Module) {
    for mut module in &mut module.submodules {
        match module.name.as_str() {
            "approved" => approved::correct(&mut module),
            "requests" => requests::correct(&mut module),
            "restricted" => restricted::correct(&mut module),
            _ => {}
        }
    }

    for mut method in &mut module.methods {
        match method.name.as_str() {
            "approve" => correct_approve(&mut method),
            "restrict" => correct_restrict(&mut method),
            _ => {}
        }
    }
}

fn correct_approve(method: &mut Method) {
    add_parameters(
        method,
        vec![Parameter {
            description: Some("The ID of the enterprise to approve the app on".into()),
            name: "enterprise_id".into(),
            required: false,
            param_type: ParameterDataType::String,
        }],
    );
}

fn correct_restrict(_method: &mut Method) {}
