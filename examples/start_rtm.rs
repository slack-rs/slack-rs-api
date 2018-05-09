extern crate slack_api as slack;

use std::default::Default;
use std::env;

fn main() {
    let token = env::var("SLACK_API_TOKEN").expect("SLACK_API_TOKEN not set.");
    let client = slack::default_client().unwrap();

    {
        let request = slack::rtm::StartRequest::default();
        let response = slack::rtm::start(&client, &token, &request);

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
