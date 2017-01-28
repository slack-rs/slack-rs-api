extern crate slack_api as slack;
extern crate reqwest;

use std::default::Default;
use std::env;

fn main() {
    let token = env::var("SLACK_API_TOKEN").expect("SLACK_API_TOKEN not set.");
    let client = reqwest::Client::new().unwrap();

    {
        let request = slack::rtm::StartRequest { token: token.clone(), ..Default::default() };
        let response = slack::rtm::start(&client, &request);

        if let Ok(response) = response {
            if let Some(channels) = response.channels {
                let channels = channels.iter()
                    .filter_map(|c| c.name.clone())
                    .collect::<Vec<_>>();
                
                println!("Got channels: {}", channels.join(", "));
            }

            if let Some(users) = response.users {
                let users = users.iter()
                    .filter_map(|u| u.name.clone())
                    .collect::<Vec<_>>();
                
                println!("Got users: {}", users.join(", "));
            }
        } else {
            println!("{:?}", response);
        }
    }
}