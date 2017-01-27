extern crate slack_api as slack;
extern crate reqwest;

use std::default::Default;
use std::env;

fn main() {
    let token = env::var("SLACK_API_TOKEN").expect("SLACK_API_TOKEN not set.");
    let client = reqwest::Client::new().unwrap();

    let request = slack::users::ListRequest { token: token, ..Default::default() };
    let response = slack::users::list(&client, &request);

    if let Ok(response) = response {
        if let Some(users) = response.members {
            let users = users.iter()
                .filter_map(|u| u.name.clone())
                .collect::<Vec<_>>();
            
            println!("Got users: {}", users.join(", "));
        }
    } else {
        println!("{:?}", response);
    }
}