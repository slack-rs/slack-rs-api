#![allow(unused_imports)]
#![allow(clippy::single_match)]
use crate::adapt::utils::*;
use crate::rust::{Method, Module};

mod profile;

pub fn correct(module: &mut Module) {
    for mut module in &mut module.submodules {
        match module.name.as_str() {
            "profile" => profile::correct(&mut module),
            _ => {}
        }
    }

    for mut method in &mut module.methods {
        match method.name.as_str() {
            "accessLogs" => correct_access_logs(&mut method),
            "billableInfo" => correct_billable_info(&mut method),
            "info" => correct_info(&mut method),
            "integrationLogs" => correct_integration_logs(&mut method),
            _ => {}
        }
    }
}

fn correct_access_logs(_method: &mut Method) {}

fn correct_billable_info(_method: &mut Method) {}

fn correct_info(_method: &mut Method) {}

fn correct_integration_logs(_method: &mut Method) {}
