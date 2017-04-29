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

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod mods;
pub use mods::*;

mod types;
pub use types::*;

pub mod requests;

fn get_slack_url_for_method(method: &str) -> String {
    format!("https://slack.com/api/{}", method)
}

fn optional_struct_or_empty_array<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where T: serde::Deserialize + Default,
          D: serde::Deserializer
{
    use std::marker::PhantomData;
    use serde::de;

    struct StructOrEmptyArray<T>(PhantomData<T>);

    impl<'de, T> de::Visitor for StructOrEmptyArray<T>
        where T: de::Deserialize + Default
    {
        type Value = Option<T>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("struct or empty array")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Option<T>, A::Error>
            where A: de::SeqVisitor
        {
            match seq.visit::<T>()? {
                Some(_) => Err(de::Error::custom("non-empty array is not valid")),
                None => Ok(Some(T::default())),
            }
        }

        fn visit_unit<E>(self) -> Result<Option<T>, E>
            where E: de::Error
        {
            Ok(None)
        }

        fn visit_none<E>(self) -> Result<Option<T>, E>
            where E: de::Error
        {
            Ok(None)
        }

        fn visit_map<M>(self, visitor: M) -> Result<Option<T>, M::Error>
            where M: de::MapVisitor
        {
            de::Deserialize::deserialize(de::value::MapVisitorDeserializer::new(visitor)).map(Some)
        }
    }

    deserializer.deserialize(StructOrEmptyArray(PhantomData))
}