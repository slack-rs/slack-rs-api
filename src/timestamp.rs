use serde;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Timestamp(u64);

impl From<f64> for Timestamp {
    fn from(t: f64) -> Self {
        let micro_seconds = t * 1_000_000.0;
        Timestamp(micro_seconds as u64)
    }
}

impl Into<f64> for Timestamp {
    fn into(self) -> f64 {
        let seconds = (self.0 as f64) / 1_000_000.0;
        seconds
    }
}
impl Into<f64> for &Timestamp {
    fn into(self) -> f64 {
        let seconds = (self.0 as f64) / 1_000_000.0;
        seconds
    }
}

impl<'de> ::serde::Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        use serde::de::Error as SerdeError;

        let value = ::serde_json::Value::deserialize(deserializer)?;
        if let Some(s) = value.as_str() {
            s.parse::<f64>()
                .map_err(|e| D::Error::custom(e))
                .map(Into::into)
        } else if let Some(f) = value.as_f64() {
            Ok(f.into())
        } else if let Some(u) = value.as_u64() {
            Ok((u as f64).into())
        } else {
            Err(D::Error::custom(format!(
                "expected a timestamp but got: {}",
                value.to_string()
            )))
        }
    }
}

impl Timestamp {
    pub fn to_param_value(&self) -> String {
        let t: f64 = self.into();
        serde_json::to_string(&t).unwrap()
    }
}
