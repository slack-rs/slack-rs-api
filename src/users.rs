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

//! Get info on members of your Slack team.
//!
//! For more information, see [Slack's API
//! documentation](https://api.slack.com/methods).

use std::collections::HashMap;

use super::{ApiResult, SlackWebRequestSender, parse_slack_response};

/// Gets user presence information.
///
/// Wraps https://api.slack.com/methods/users.getPresence
pub fn get_presence<R: SlackWebRequestSender>(client: &R, token: &str, user: &str) -> ApiResult<GetPresenceResponse> {
    let mut params = HashMap::new();
    params.insert("user", user);
    let response = try!(client.send_authed( "users.getPresence", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct GetPresenceResponse {
    pub presence: String,
    pub online: Option<bool>,
    pub auto_away: Option<bool>,
    pub manual_away: Option<bool>,
    pub connection_count: Option<u32>,
    pub last_activity: Option<u32>,
}

/// Gets information about a user.
///
/// Wraps https://api.slack.com/methods/users.info
pub fn info<R: SlackWebRequestSender>(client: &R, token: &str, user: &str) -> ApiResult<InfoResponse> {
    let mut params = HashMap::new();
    params.insert("user", user);
    let response = try!(client.send_authed( "users.info", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct InfoResponse {
    pub user: super::User,
}

/// Lists all users in a Slack team.
///
/// Wraps https://api.slack.com/methods/users.list
pub fn list<R: SlackWebRequestSender>(client: &R, token: &str, presence: Option<bool>) -> ApiResult<ListResponse> {
    let mut params = HashMap::new();
    if let Some(presence) = presence {
        params.insert("presence",
                      if presence {
                          "1"
                      } else {
                          "0"
                      });
    }
    let response = try!(client.send_authed( "users.list", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct ListResponse {
    pub members: Vec<super::User>,
}

/// Marks a user as active.
///
/// Wraps https://api.slack.com/methods/users.setActive
pub fn set_active<R: SlackWebRequestSender>(client: &R, token: &str) -> ApiResult<SetActiveResponse> {
    let response = try!(client.send_authed( "users.setActive", token, HashMap::new()));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct SetActiveResponse;

/// Manually sets user presence.
///
/// Wraps https://api.slack.com/methods/users.setPresence
pub fn set_presence<R: SlackWebRequestSender>(client: &R, token: &str, presence: &str) -> ApiResult<SetPresenceResponse> {
    let mut params = HashMap::new();
    params.insert("presence", presence);
    let response = try!(client.send_authed( "users.setPresence", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct SetPresenceResponse;

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::test_helpers::*;

    #[test]
    fn general_api_error_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{"ok": false, "err": "some_error"}"#);
        let result = get_presence(&client, "TEST_TOKEN", "U12345678");
        assert!(result.is_err());
    }

    #[test]
    fn get_presence_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"
            {
                "ok": true,
                "presence": "active"
            }
        "#);
        let result = get_presence(&client, "TEST_TOKEN", "U12345678");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert_eq!(result.unwrap().presence, "active");
    }

    #[test]
    fn get_presence_authed_user_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"
            {
                "ok": true,
                "presence": "active",
                "online": true,
                "auto_away": false,
                "manual_away": false,
                "connection_count": 1,
                "last_activity": 1419027078
            }
        "#);
        let result = get_presence(&client, "TEST_TOKEN", "U12345678");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        let result = result.unwrap();
        assert_eq!(result.presence, "active");
        assert_eq!(result.last_activity.unwrap(), 1419027078);
    }

    #[test]
    fn info_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"
            {
                "ok": true,
                "user": {
                    "id": "U023BECGF",
                    "name": "bobby",
                    "deleted": false,
                    "color": "9f69e7",
                    "profile": {
                        "first_name": "Bobby",
                        "last_name": "Tables",
                        "real_name": "Bobby Tables",
                        "email": "bobby@slack.com",
                        "skype": "my-skype-name",
                        "phone": "+1 (123) 456 7890",
                        "image_24": "https:\/\/...",
                        "image_32": "https:\/\/...",
                        "image_48": "https:\/\/...",
                        "image_72": "https:\/\/...",
                        "image_192": "https:\/\/..."
                    },
                    "is_admin": true,
                    "is_owner": true,
                    "has_2fa": true,
                    "has_files": true
                }
            }
        "#);
        let result = info(&client, "TEST_TOKEN", "U12345678");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        let result = result.unwrap();
        assert_eq!(result.user.id, "U023BECGF");
        assert_eq!(result.user.profile.email.as_ref().unwrap(),
                   "bobby@slack.com");
    }

    #[test]
    fn list_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"
            {
                "ok": true,
                "members": [
                    {
                        "id": "U023BECGF",
                        "name": "bobby",
                        "deleted": false,
                        "color": "9f69e7",
                        "profile": {
                            "first_name": "Bobby",
                            "last_name": "Tables",
                            "real_name": "Bobby Tables",
                            "email": "bobby@slack.com",
                            "skype": "my-skype-name",
                            "phone": "+1 (123) 456 7890",
                            "image_24": "https:\/\/...",
                            "image_32": "https:\/\/...",
                            "image_48": "https:\/\/...",
                            "image_72": "https:\/\/...",
                            "image_192": "https:\/\/..."
                        },
                        "is_admin": true,
                        "is_owner": true,
                        "has_2fa": true,
                        "has_files": true
                    },
                    {
                        "id": "U12345678",
                        "name": "alice",
                        "deleted": false,
                        "color": "9f69e7",
                        "profile": {
                            "first_name": "Alice",
                            "last_name": "Aardvark",
                            "real_name": "Alice Aardvark",
                            "email": "alice@slack.com",
                            "skype": "my-skype-name",
                            "phone": "+1 (123) 456 7890",
                            "image_24": "https:\/\/...",
                            "image_32": "https:\/\/...",
                            "image_48": "https:\/\/...",
                            "image_72": "https:\/\/...",
                            "image_192": "https:\/\/..."
                        },
                        "is_admin": true,
                        "is_owner": true,
                        "has_2fa": true,
                        "has_files": true
                    }
                ]
            }
        "#);
        let result = list(&client, "TEST_TOKEN", None);
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        let result = result.unwrap();
        assert_eq!(result.members[1].id, "U12345678");
        assert_eq!(result.members[0].profile.email.as_ref().unwrap(),
                   "bobby@slack.com");
    }

    #[test]
    fn set_active_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{"ok": true}"#);
        let result = set_active(&client, "TEST_TOKEN");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
    }

    #[test]
    fn set_presence_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{"ok": true}"#);
        let result = set_presence(&client, "TEST_TOKEN", "active");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
    }
}
