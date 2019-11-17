use slack_api as slack;

use std::default::Default;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // You can generate a legacy token to quickly test these apis
    // https://api.slack.com/custom-integrations/legacy-tokens
    let token = env::var("SLACK_API_TOKEN").map_err(|_| "SLACK_API_TOKEN env var must be set")?;
    let client =
        slack::default_client().map_err(|e| format!("Could not get default_client, {:?}", e))?;

    {
        let request = slack::rtm::StartRequest::default();
        let response = slack::rtm::start(&client, &token, &request);

        if let Ok(response) = response {
            if let Some(channels) = response.channels {
                let channel_names = channels
                    .iter()
                    .filter_map(|c| c.name.as_ref())
                    .collect::<Vec<_>>();

                println!("Got channels: {:?}", channel_names);
            }

            if let Some(users) = response.users {
                let user_names = users
                    .iter()
                    .filter_map(|u| u.name.as_ref())
                    .collect::<Vec<_>>();

                println!("Got users: {:?}", user_names);
            }
        } else {
            println!("{:?}", response);
        }
    }
    Ok(())
}
