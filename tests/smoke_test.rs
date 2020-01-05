use slack_api as slack;

use std::env;

fn get_setup(
) -> Result<(String, impl slack::requests::SlackWebRequestSender), Box<dyn std::error::Error>> {
    // You can generate a legacy token to quickly test these apis
    // https://api.slack.com/custom-integrations/legacy-tokens
    let token = env::var("SLACK_API_TOKEN").map_err(|_| "SLACK_API_TOKEN env var must be set")?;
    let client = slack::default_client().map_err(|_| "Could not get default_client")?;
    Ok((token, client))
}

#[test]
#[ignore]
fn smoke_test() -> Result<(), Box<dyn std::error::Error>> {
    let (_, client) = get_setup()?;

    let resp = slack::api::test(
        &client,
        &slack::api::TestRequest {
            error: Some("my_error"),
            foo: Some("it's me, foo"),
        },
    )
    .err()
    .ok_or("Expected error")?;

    assert_eq!(format!("{:?}", resp), "Unknown(\"my_error\")");
    Ok(())
}

#[test]
#[ignore]
fn smoke_channels() -> Result<(), Box<dyn std::error::Error>> {
    let (token, client) = get_setup()?;

    let all_channels = slack::channels::list(
        &client,
        &token,
        &slack::channels::ListRequest {
            ..slack::channels::ListRequest::default()
        },
    )?
    .channels
    .ok_or("Expected some channels")?;

    assert!(all_channels.len() > 0);

    for channel in &all_channels[..10] {
        let channel_id = channel.id.as_ref().ok_or("expected channel id")?;

        let channel_info = slack::channels::info(
            &client,
            &token,
            &slack::channels::InfoRequest {
                channel: &channel_id,
                ..Default::default()
            },
        )?
        .channel
        .ok_or("Expected some channel")?;

        dbg!(channel_info.name);

        let _channel_history = slack::channels::history(
            &client,
            &token,
            &slack::channels::HistoryRequest {
                channel: &channel_id,
                oldest: Some(1234567890.1234.into()),
                count: Some(1),
                ..Default::default()
            },
        )?
        .messages
        .ok_or("Expected some messages")?;
    }

    Ok(())
}
