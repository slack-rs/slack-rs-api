# slack-rs-api

[Slack][slack] Web API interface.

[![Build Status][ci-img]][ci-url] [![Crates.io][crates-img]][crates-url] [![License][license-img]][license-url]

[Documentation][docs]

## Usage

Add this to your `Cargo.toml`:
```toml
[dependencies]
slack_api = "0.23.0"
```

### Async
`default-features` include an async functions and client using [reqwest][reqwest]  
See [async channel history example](examples/channel_history.rs)

### Sync
The `"sync"` feature provides sync functions and the `"reqwest_blocking"` feature provides a sync client using reqwest  
See [sync channel history example](examples/channel_history_sync.rs)

## Slack docs
Slack's api is large and changes often. Their docs are high quality and no attempt to replicate them is made in this crate's docs. Please refer to their docs as your primary resource of how slack's api works.

## Providing own client
You can provide your own client by implementing the async or sync versions of `SlackWebRequestSender`.   
Which should would allow avoiding `reqwest` and thus `tokio`.

## Something I need is missing
Not every method is available in this crate but if something is missing you would like then please log an issue. Bear in mind this is maintained in contributor's spare time and contributions are welcome.

## License
`slack-api` is distributed under the [Apache-2.0 License](./LICENSE).

[docs]: https://docs.rs/slack_api
[ci-img]: https://travis-ci.org/slack-rs/slack-rs-api.svg?branch=master
[ci-url]: https://travis-ci.org/slack-rs/slack-rs-api
[crates-img]: https://img.shields.io/crates/v/slack_api.svg
[crates-url]: https://crates.io/crates/slack_api
[license-img]: https://img.shields.io/github/license/mthjones/slack-rs-api.svg
[license-url]: https://raw.githubusercontent.com/mthjones/slack-rs-api/master/LICENSE
[slack]: https://api.slack.com/
[slack_web]: https://api.slack.com/web
[reqwest]: https://crates.io/crates/reqwest
