use serde::{
    de::{DeserializeOwned, Deserializer},
    ser::{SerializeSeq, Serializer},
    Deserialize, Serialize,
};
use std::{fmt, marker::PhantomData};

#[derive(Debug, Clone, PartialEq)]
#[allow(missing_docs)]
pub struct VecOrSingle<T>(pub Vec<T>);

impl<T> std::ops::Deref for VecOrSingle<T> {
    type Target = Vec<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for VecOrSingle<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Serialize> Serialize for VecOrSingle<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self.split_first() {
            Some((first, rest)) if rest.is_empty() => first.serialize(serializer),
            _ => {
                let mut seq = serializer.serialize_seq(Some(self.len()))?;
                for e in &self.0 {
                    seq.serialize_element(e)?;
                }
                seq.end()
            }
        }
    }
}

impl<'de, T: DeserializeOwned> Deserialize<'de> for VecOrSingle<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<VecOrSingle<T>, D::Error> {
        struct VecOrSingleVisit<T>(PhantomData<T>);

        impl<T: DeserializeOwned> VecOrSingleVisit<T> {
            fn visit_any<'de, D, E>(self, v: D) -> Result<Vec<T>, E>
            where
                D: ::serde::de::IntoDeserializer<'de, E>,
                E: ::serde::de::Error,
            {
                let deserializer = ::serde::de::IntoDeserializer::into_deserializer(v);
                ::serde::Deserialize::deserialize(deserializer).map(|v| vec![v])
            }
        }

        impl<'de, T: DeserializeOwned> serde::de::Visitor<'de> for VecOrSingleVisit<T> {
            type Value = Vec<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("single type or a list of that type")
            }

            fn visit_bool<E: ::serde::de::Error>(self, v: bool) -> Result<Self::Value, E> {
                self.visit_any(v)
            }

            fn visit_i8<E: ::serde::de::Error>(self, v: i8) -> Result<Self::Value, E> {
                self.visit_any(v)
            }

            fn visit_i16<E: ::serde::de::Error>(self, v: i16) -> Result<Self::Value, E> {
                self.visit_any(v)
            }

            fn visit_i32<E: ::serde::de::Error>(self, v: i32) -> Result<Self::Value, E> {
                self.visit_any(v)
            }

            fn visit_i64<E: ::serde::de::Error>(self, v: i64) -> Result<Self::Value, E> {
                self.visit_any(v)
            }

            fn visit_u8<E: ::serde::de::Error>(self, v: u8) -> Result<Self::Value, E> {
                self.visit_any(v)
            }

            fn visit_u16<E: ::serde::de::Error>(self, v: u16) -> Result<Self::Value, E> {
                self.visit_any(v)
            }

            fn visit_u32<E: ::serde::de::Error>(self, v: u32) -> Result<Self::Value, E> {
                self.visit_any(v)
            }

            fn visit_u64<E: ::serde::de::Error>(self, v: u64) -> Result<Self::Value, E> {
                self.visit_any(v)
            }

            fn visit_f32<E: ::serde::de::Error>(self, v: f32) -> Result<Self::Value, E> {
                self.visit_any(v)
            }

            fn visit_f64<E: ::serde::de::Error>(self, v: f64) -> Result<Self::Value, E> {
                self.visit_any(v)
            }

            fn visit_str<E: ::serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                self.visit_any(v)
            }

            fn visit_string<E: ::serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                self.visit_any(v)
            }

            fn visit_unit<E: ::serde::de::Error>(self) -> Result<Self::Value, E> {
                self.visit_any(())
            }

            fn visit_newtype_struct<D: serde::de::Deserializer<'de>>(
                self,
                deserializer: D,
            ) -> Result<Self::Value, D::Error> {
                ::serde::Deserialize::deserialize(deserializer).map(|v| vec![v])
            }

            fn visit_seq<A: serde::de::SeqAccess<'de>>(
                self,
                seq: A,
            ) -> Result<Self::Value, A::Error> {
                Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(seq))
            }

            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                map: A,
            ) -> Result<Self::Value, A::Error> {
                Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))
                    .map(|v| vec![v])
            }
        }

        deserializer
            .deserialize_any(VecOrSingleVisit(PhantomData {}))
            .map(VecOrSingle)
    }
}
