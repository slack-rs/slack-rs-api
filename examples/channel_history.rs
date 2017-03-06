extern crate slack_api as slack;
extern crate reqwest;

use std::env;

fn main() {
    let token = env::var("SLACK_API_TOKEN").expect("SLACK_API_TOKEN not set.");
    let client = reqwest::Client::new().unwrap();

    let response = slack::channels::history(&client,
                                            &token,
                                            &slack::channels::HistoryRequest {
                                                channel: &env::args().nth(1).unwrap(),
                                                ..slack::channels::HistoryRequest::default()
                                            });

    if let Ok(response) = response {
        if let Some(messages) = response.messages {
            println!("Got {} messages:", messages.len());
            for message in messages {
                println!("{:?}", message);
            }
        }
    } else {
        println!("{:?}", response);
    }
}
