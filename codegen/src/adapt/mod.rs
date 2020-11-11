#![allow(unused_imports)]
#![allow(clippy::single_match)]
use crate::adapt::utils::*;
use crate::rust::{Method, Module};

mod admin;
mod api;
mod apps;
mod auth;
mod bots;
mod calls;
mod chat;
mod conversations;
mod dialog;
mod dnd;
mod emoji;
mod files;
mod migration;
mod oauth;
mod pins;
mod reactions;
mod reminders;
mod rtm;
mod search;
mod stars;
mod team;
mod usergroups;
mod users;
mod utils;
mod views;
mod workflows;

pub fn correct(modules: &mut Vec<Module>) {
    for mut module in modules {
        match module.name.as_str() {
            "bots" => bots::correct(&mut module),
            "migration" => migration::correct(&mut module),
            "emoji" => emoji::correct(&mut module),
            "files" => files::correct(&mut module),
            "pins" => pins::correct(&mut module),
            "dnd" => dnd::correct(&mut module),
            "oauth" => oauth::correct(&mut module),
            "reactions" => reactions::correct(&mut module),
            "usergroups" => usergroups::correct(&mut module),
            "dialog" => dialog::correct(&mut module),
            "admin" => admin::correct(&mut module),
            "workflows" => workflows::correct(&mut module),
            "chat" => chat::correct(&mut module),
            "search" => search::correct(&mut module),
            "views" => views::correct(&mut module),
            "apps" => apps::correct(&mut module),
            "rtm" => rtm::correct(&mut module),
            "stars" => stars::correct(&mut module),
            "team" => team::correct(&mut module),
            "conversations" => conversations::correct(&mut module),
            "reminders" => reminders::correct(&mut module),
            "auth" => auth::correct(&mut module),
            "api" => api::correct(&mut module),
            "calls" => calls::correct(&mut module),
            "users" => users::correct(&mut module),
            _ => {}
        }
    }
}
