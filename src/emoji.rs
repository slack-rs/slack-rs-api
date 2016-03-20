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

/// Lists custom emoji for a team.
///
/// Wraps https://api.slack.com/methods/emoji.list
pub fn list<R: SlackWebRequestSender>(client: &R, token: &str) -> ApiResult<ListResponse> {
    let response = try!(client.send_authed("emoji.list", token, HashMap::new()));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct ListResponse {
    pub emoji: HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::test_helpers::*;

    #[test]
    fn general_api_error_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{"ok": false, "err": "some_error"}"#);
        let result = list(&client, "TEST_TOKEN");
        assert!(result.is_err());
    }

    #[test]
    fn test_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{
            "ok": true,
            "emoji": {
                "bowtie": "https:\/\/my.slack.com\/emoji\/bowtie\/46ec6f2bb0.png",
                "squirrel": "https:\/\/my.slack.com\/emoji\/squirrel\/f35f40c0e0.png",
                "shipit": "alias:squirrel"
            }
        }"#);
        let result = list(&client, "TEST_TOKEN");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert_eq!(result.unwrap().emoji.get("shipit").unwrap(),
                   "alias:squirrel");
    }
}
