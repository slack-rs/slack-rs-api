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

//! For more information, see [Slack's API
//! documentation](https://api.slack.com/methods).

use std::collections::HashMap;

use super::{ApiResult, SlackWebRequestSender, parse_slack_response};

/// Exchanges a temporary OAuth code for an API token.
///
/// Wraps https://api.slack.com/methods/oauth.access
pub fn access<R: SlackWebRequestSender>(client: &R, client_id: &str, client_secret: &str, code: &str, redirect_uri: Option<&str>) -> ApiResult<AccessResponse> {
    let mut params = HashMap::new();
    params.insert("client_id", client_id);
    params.insert("client_secret", client_secret);
    params.insert("code", code);
    if let Some(redirect_uri) = redirect_uri {
        params.insert("redirect_uri", redirect_uri);
    }
    
    let response = try!(client.send("oauth.access", params));
    parse_slack_response(response, false)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct AccessResponse {
    pub access_token: String,
    pub scope: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::test_helpers::*;

    #[test]
    fn general_api_error_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{"ok": false, "err": "some_error"}"#);
        let result = access(&client, "TEST_ID", "TEST_TOKEN", "TEST_CODE", None);
        assert!(result.is_err());
    }

    #[test]
    fn access_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{
            "access_token": "xoxt-23984754863-2348975623103",
            "scope": "read"
        }"#);
        let result = access(&client, "TEST_ID", "TEST_TOKEN", "TEST_CODE", None);
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert_eq!(result.unwrap().access_token,
                   "xoxt-23984754863-2348975623103");
    }
}
