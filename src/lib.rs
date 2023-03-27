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

//! Low-level, direct interface for the [Slack Web
//! API](https://api.slack.com/methods).

#[macro_use]
extern crate serde_derive;

mod mod_types;

mod timestamp;
pub use crate::timestamp::*;

mod types;
pub use crate::types::*;

#[cfg(feature = "async")]
mod async_impl;

#[cfg(feature = "async")]
pub use async_impl::*;

#[cfg(feature = "sync")]
pub mod sync;

fn get_slack_url_for_method(method: &str) -> String {
    format!("https://slack.com/api/{}", method)
}

fn optional_struct_or_empty_array<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: serde::Deserialize<'de>,
    D: serde::Deserializer<'de>,
{
    use serde::de;
    use std::marker::PhantomData;

    struct StructOrEmptyArray<T>(PhantomData<T>);

    impl<'de, T> de::Visitor<'de> for StructOrEmptyArray<T>
    where
        T: de::Deserialize<'de>,
    {
        type Value = Option<T>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("struct or empty array")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Option<T>, A::Error>
        where
            A: de::SeqAccess<'de>,
        {
            match seq.next_element::<T>()? {
                Some(_) => Err(de::Error::custom("non-empty array is not valid")),
                None => Ok(None),
            }
        }

        fn visit_unit<E>(self) -> Result<Option<T>, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_map<M>(self, access: M) -> Result<Option<T>, M::Error>
        where
            M: de::MapAccess<'de>,
        {
            T::deserialize(de::value::MapAccessDeserializer::new(access)).map(Some)
        }
    }

    deserializer.deserialize_any(StructOrEmptyArray(PhantomData))
}

#[cfg(test)]
mod tests {
    use super::UserProfile;

    #[test]
    fn test_user_profile_fields_empty_array_deserialize() {
        let user_profile: UserProfile = serde_json::from_str(r#"{"fields": []}"#).unwrap();
        assert!(user_profile.fields.is_none());
    }

    #[test]
    fn test_user_profile_fields_empty_map_deserialize() {
        let user_profile: UserProfile = serde_json::from_str(r#"{"fields": {}}"#).unwrap();
        assert_eq!(0, user_profile.fields.unwrap().len());
    }

    #[test]
    fn test_user_profile_fields_nonempty_map_deserialize() {
        let user_profile: UserProfile =
            serde_json::from_str(r#"{"fields": {"some_field": {"alt": "foo", "label": "bar"}}}"#)
                .unwrap();
        assert_eq!(1, user_profile.fields.unwrap().len());
    }

    #[test]
    fn test_user_profile_fields_null_deserialize() {
        let user_profile: UserProfile = serde_json::from_str(r#"{"fields": null}"#).unwrap();
        assert!(user_profile.fields.is_none());
    }

    #[test]
    fn test_user_profile_fields_undefined_deserialize() {
        let user_profile: UserProfile = serde_json::from_str(r#"{}"#).unwrap();
        assert!(user_profile.fields.is_none());
    }

    #[test]
    fn test_timestamp_to_param_value() {
        assert_eq!(
            crate::Timestamp::from(1234567.0).to_param_value(),
            "1234567.000000"
        );
    }

    #[test]
    fn test_timestamp_within_message() {
        let msg = r#"{
            "type": "message",
            "text": "!sqrt 2",
            "user": "U0E000000",
            "channel": "D00000000",
            "event_ts": "1588861564.009805",
            "ts": "1588861564.009805"
        }"#;
        let message: crate::Message = serde_json::from_str(msg).unwrap();
        match message {
            crate::Message::Standard(crate::MessageStandard {
                event_ts: Some(event_ts),
                ts: Some(ts),
                ..
            }) => {
                assert_eq!(event_ts.to_param_value(), "1588861564.009805");
                assert_eq!(ts.to_param_value(), "1588861564.009805");
            }
            m => panic!("expected Message::Standard but got {:?}", m),
        };
    }
}
