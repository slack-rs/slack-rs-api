#![cfg(feature = "reqwest")]

use slack_api as slack;

use std::env;

fn get_setup() -> Result<(String, impl slack::SlackWebRequestSender), Box<dyn std::error::Error>> {
    // You can generate a legacy token to quickly test these apis
    // https://api.slack.com/custom-integrations/legacy-tokens
    let token = env::var("SLACK_API_TOKEN").map_err(|_| "SLACK_API_TOKEN env var must be set")?;
    let client = slack::default_client().map_err(|_| "Could not get default_client")?;
    Ok((token, client))
}

#[tokio::test]
#[ignore]
async fn smoke_test() -> Result<(), Box<dyn std::error::Error>> {
    let (_, client) = get_setup()?;

    let resp = slack::api::test(
        &client,
        &slack::api::TestRequest {
            error: Some("my_error".into()),
            foo: Some("it's me, foo".into()),
        },
    )
    .await
    .err()
    .ok_or("Expected error")?;

    assert_eq!(format!("{:?}", resp), "Unknown(\"my_error\")");
    Ok(())
}

#[tokio::test]
#[ignore]
async fn smoke_channels() -> Result<(), Box<dyn std::error::Error>> {
    let (token, client) = get_setup()?;

    let all_channels = slack::conversations::list(
        &client,
        &token,
        &slack::conversations::ListRequest {
            ..slack::conversations::ListRequest::default()
        },
    )
    .await?
    .channels;

    assert!(!all_channels.is_empty());

    for channel in &all_channels[..10] {
        let channel_id = &channel.id;

        let channel_info = slack::conversations::info(
            &client,
            &token,
            &slack::conversations::InfoRequest {
                channel: channel_id.into(),
                ..Default::default()
            },
        )
        .await?
        .channel;

        dbg!(channel_info.name);

        let _channel_history = slack::conversations::history(
            &client,
            &token,
            &slack::conversations::HistoryRequest {
                channel: channel_id.into(),
                oldest: Some(1234567890.1234),
                ..Default::default()
            },
        )
        .await
        .map_err(|e| {
            if let slack::conversations::HistoryError::MalformedResponse(r, inner_e) = e {
                println!("{}", r);
                Box::new(inner_e) as Box<dyn std::error::Error>
            } else {
                Box::new(e)
            }
        })?
        .messages;
    }

    Ok(())
}
