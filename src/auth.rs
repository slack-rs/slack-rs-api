// Copyright 2015-2016 the slack-rs authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Checks authentication & identity.
//!
//! For more information, see [Slack's API
//! documentation](https://api.slack.com/methods).

use std::collections::HashMap;

use super::{ApiResult, SlackWebRequestSender, parse_slack_response};

/// Checks authentication & identity.
///
/// Wraps https://api.slack.com/methods/auth.test
pub fn test<R: SlackWebRequestSender>(client: &R, token: &str) -> ApiResult<TestResponse> {
    let response = try!(client.send_authed("auth.test", token, HashMap::new()));
    parse_slack_response(response, true)
}

#[derive(RustcDecodable)]
pub struct TestResponse {
    pub url: String,
    pub team: String,
    pub user: String,
    pub team_id: String,
    pub user_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::test_helpers::*;

    #[test]
    fn general_api_error_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{"ok": false, "err": "some_error"}"#);
        let result = test(&client, "TEST_TOKEN");
        assert!(result.is_err());
    }

    #[test]
    fn test_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{
            "ok": true,
            "url": "https:\/\/example-team.slack.com\/",
            "team": "example team",
            "user": "testuser",
            "team_id": "T12345678",
            "user_id": "U12345678"
        }"#);
        
        let result = test(&client, "TEST_TOKEN");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert_eq!(result.unwrap().user, "testuser");
    }
}
