# slack-rs

[Slack][slack] Web API interface.

[![Build Status][ci-img]][ci-url] [![Crates.io][crates-img]][crates-url] [![License][license-img]][license-url]

[Documentation][docs]

# Usage

Add this to your `Cargo.toml`:
```toml
[dependencies]
slack_api = "0.15.0"
```

and this to your crate root:

```rust
extern crate slack_api;
```

# License
`slack-api` is distributed under the [Apache-2.0 License](./LICENSE).

[docs]: https://docs.rs/slack_api
[ci-img]: https://travis-ci.org/slack-rs/slack-rs-api.svg?branch=master
[ci-url]: https://travis-ci.org/slack-rs/slack-rs-api
[crates-img]: https://img.shields.io/crates/v/slack_api.svg
[crates-url]: https://crates.io/crates/slack_api
[license-img]: https://img.shields.io/github/license/mthjones/slack-rs-api.svg
[license-url]: https://raw.githubusercontent.com/mthjones/slack-rs-api/master/LICENSE
[slack]: https://api.slack.com/
