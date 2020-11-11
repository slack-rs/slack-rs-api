#![allow(unused_imports)]
#![allow(clippy::single_match)]
use crate::adapt::utils::*;
use crate::rust::{Method, Module};

pub fn correct(module: &mut Module) {
    for mut method in &mut module.methods {
        match method.name.as_str() {
            "stepCompleted" => correct_step_completed(&mut method),
            "stepFailed" => correct_step_failed(&mut method),
            "updateStep" => correct_update_step(&mut method),
            _ => {}
        }
    }
}

fn correct_step_completed(_method: &mut Method) {}

fn correct_step_failed(_method: &mut Method) {}

fn correct_update_step(_method: &mut Method) {}
