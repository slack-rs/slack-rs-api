#![allow(unused_imports)]
#![allow(clippy::single_match)]
use crate::adapt::utils::*;
use crate::rust::{Method, Module};

pub fn correct(module: &mut Module) {
    for mut method in &mut module.methods {
        match method.name.as_str() {
            "endDnd" => correct_end_dnd(&mut method),
            "endSnooze" => correct_end_snooze(&mut method),
            "info" => correct_info(&mut method),
            "setSnooze" => correct_set_snooze(&mut method),
            "teamInfo" => correct_team_info(&mut method),
            _ => {}
        }
    }
}

fn correct_end_dnd(_method: &mut Method) {}

fn correct_end_snooze(_method: &mut Method) {}

fn correct_info(_method: &mut Method) {}

fn correct_set_snooze(_method: &mut Method) {}

fn correct_team_info(_method: &mut Method) {}
