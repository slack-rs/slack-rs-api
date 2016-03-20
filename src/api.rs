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

//! Checks API calling code.
//!
//! For more information, see [Slack's API
//! documentation](https://api.slack.com/methods).

use std::collections::HashMap;

use super::{ApiResult, SlackWebRequestSender, parse_slack_response};

/// Checks API calling code.
///
/// Wraps https://api.slack.com/methods/api.test
pub fn test<R: SlackWebRequestSender>(client: &R, args: Option<HashMap<&str, &str>>, error: Option<&str>) -> ApiResult<ApiTestResponse> {
    let mut params = HashMap::new();
    if let Some(error) = error {
        params.insert("error", error);
    }
    if let Some(args) = args {
        params.extend(args);
    }
    
    let response = try!(client.send("api.test", params));
    parse_slack_response(response, true)
}

#[derive(RustcDecodable)]
pub struct ApiTestResponse {
    pub error: Option<String>,
    pub args: Option<HashMap<String, String>>,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;
    use super::super::test_helpers::*;

    #[test]
    fn general_api_error_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{"ok": false, "err": "some_error"}"#);
        let result = test(&client, None, Some("some_error"));
        assert!(result.is_err());
    }

    #[test]
    fn test_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{
            "ok": true,
            "args": {
                "arg1": "val1",
                "arg2": "val2"
            }
        }"#);
        let mut args = HashMap::new();
        args.insert("arg1", "val1");
        args.insert("arg2", "val2");
        let result = test(&client, Some(args), None);
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert_eq!(result.unwrap().args.unwrap().get("arg1").unwrap(), "val1");
    }
}
