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

/// Creates a reminder
///
/// Wraps https://api.slack.com/methods/reminders.add
pub fn add<R: SlackWebRequestSender>(client: &R,
           token: &str,
           text: &str,
           time: &str,
           user: Option<&str>)
           -> ApiResult<AddResponse> {
    let mut params = HashMap::new();
    params.insert("text", text);
    params.insert("time", time);
    if let Some(user) = user {
        params.insert("user", user);
    }
    let response = try!(client.send_authed("reminders.add", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct AddResponse {
    pub reminder: super::Reminder,
}

/// Marks a reminder as complete
///
/// Wraps https://api.slack.com/methods/reminders.complete
pub fn complete<R: SlackWebRequestSender>(client: &R, token: &str, reminder: &str) -> ApiResult<CompleteResponse> {
    let mut params = HashMap::new();
    params.insert("reminder", reminder);
    let response = try!(client.send_authed("reminders.complete", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct CompleteResponse;


/// Deletes a reminder
///
/// Wraps https://api.slack.com/methods/reminders.delete
pub fn delete<R: SlackWebRequestSender>(client: &R, token: &str, reminder: &str) -> ApiResult<DeleteResponse> {
    let mut params = HashMap::new();
    params.insert("reminder", reminder);
    let response = try!(client.send_authed("reminders.delete", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct DeleteResponse;

/// Gets info for a reminder
///
/// Wraps https://api.slack.com/methods/reminders.info
pub fn info<R: SlackWebRequestSender>(client: &R, token: &str, reminder: &str) -> ApiResult<InfoResponse> {
    let mut params = HashMap::new();
    params.insert("reminder", reminder);
    let response = try!(client.send_authed("reminders.info", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct InfoResponse {
    reminder: super::Reminder,
}

/// Lists reminders by or for the auth'd user.
///
/// Wraps https://api.slack.com/methods/reminders.list
pub fn list<R: SlackWebRequestSender>(client: &R, token: &str) -> ApiResult<ListResponse> {
    let response = try!(client.send_authed("reminders.list", token, HashMap::new()));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct ListResponse {
    pub reminders: Vec<super::Reminder>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::test_helpers::*;

    #[test]
    fn general_api_error_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{"ok": true}"#);
        let result = add(&client,
                         "TEST_TOKEN",
                         "thumbsup",
                         "in 10 minutes",
                         None);
        assert!(result.is_err());
    }

    #[test]
    fn add_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{
            "ok": true,
            "reminder": {
                "id": "Rm12345678",
                "creator": "U18888888",
                "user": "U18888888",
                "text": "eat a banana",
                "recurring": false,
                "time": 1602288000,
                "complete_ts": 0
            }
        }"#);
        let result = add(&client,
                         "TEST_TOKEN",
                         "thumbsup",
                         "in 10 minutes",
                         None);
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
    }

    #[test]
    fn info_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{
                "ok": true,
                "reminder": {
                    "id": "Rm12345678",
                    "creator": "U18888888",
                    "user": "U18888888",
                    "text": "eat a banana",
                    "recurring": false,
                    "time": 1458678068,
                    "complete_ts": 1458678200
                }
            }"#);
        let result = info(&client,
                         "TEST_TOKEN",
                         "Rm12345678");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        let reminder = result.unwrap().reminder;
        assert_eq!(reminder.id, "Rm12345678");
        assert_eq!(reminder.time.unwrap(), 1458678068);
        assert_eq!(reminder.recurring, false);
        assert_eq!(reminder.user,"U18888888");
        assert_eq!(reminder.creator,"U18888888");
    }

    #[test]
    fn list_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{
                "ok": true,
                "reminders": [
                    {
                        "id": "Rm12345678",
                        "creator": "U18888888",
                        "user": "U18888888",
                        "text": "eat a banana",
                        "recurring": false,
                        "time": 1458678068,
                        "complete_ts": 0
                    },
                    {
                        "id": "Rm23456789",
                        "creator": "U18888888",
                        "user": "U18888888",
                        "text": "drink water",
                        "recurring": true
                    }
                ]
        }"#);
        let result = list(&client, "TEST_TOKEN");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        let reminders = result.unwrap().reminders;
        assert_eq!(reminders[0].recurring, false);
        assert_eq!(reminders[1].recurring, true);
        assert_eq!(reminders[0].time, Some(1458678068));
        assert_eq!(reminders[1].time, None);
    }

    #[test]
    fn delete_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{"ok": true}"#);
        let result = delete(&client,
                            "TEST_TOKEN",
                            "Rm23456789");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
    }
}
