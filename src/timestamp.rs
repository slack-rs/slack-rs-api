#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Timestamp(u64);

impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let u = self.0 / 1_000_000;
        let l = self.0 % 1_000_000;
        write!(f, "{}.{:06}", u, l)
    }
}

impl From<f64> for Timestamp {
    fn from(t: f64) -> Self {
        let micro_seconds = t * 1_000_000.0;
        Timestamp(micro_seconds as u64)
    }
}

impl From<u64> for Timestamp {
    fn from(t: u64) -> Self {
        let micro_seconds = t * 1_000_000;
        Timestamp(micro_seconds)
    }
}

impl From<(u64, f64)> for Timestamp {
    fn from(ts: (u64, f64)) -> Self {
        let (ti, td) = ts;
        let micro_seconds = ti * 1_000_000 + (td * 1_000_000.0) as u64;
        Timestamp(micro_seconds)
    }
}

impl<'de> ::serde::Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        use serde::de::Error as SerdeError;

        // slack seems to use strings sometimes to
        // maintain precision greater than what f64 is happy with
        // so custom parser to get the f128 out of the string

        let value = ::serde_json::Value::deserialize(deserializer)?;

        if let Some(s) = value.as_str() {
            if let Some(dot_index) = s.find('.') {
                // must be f128 in string
                let i = s[..dot_index].parse::<u64>();
                let d = s[dot_index..].parse::<f64>();
                if let (Ok(i), Ok(d)) = (i, d) {
                    return Ok((i, d).into());
                }
            } else {
                // must be u64 in a string
                if let Ok(u) = s.parse::<u64>() {
                    return Ok(u.into());
                }
            }
        }

        if let Some(f) = value.as_f64() {
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
        format!("{}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserve_precision_f64() {
        let ts_str = "1588859442.008705";
        let ts: Timestamp = serde_json::from_str(ts_str).unwrap();
        assert_eq!(ts, Timestamp(1588859442008705));
        assert_eq!(ts.to_param_value(), "1588859442.008705");
    }

    #[test]
    fn preserve_precision_str() {
        let ts_str = "\"1588859442.008705\"";
        let ts: Timestamp = serde_json::from_str(ts_str).unwrap();
        assert_eq!(ts, Timestamp(1588859442008705));
        assert_eq!(ts.to_param_value(), "1588859442.008705");
    }

    #[test]
    fn preserve_precision_str_0_dp() {
        let ts_str = "\"1588859442\"";
        let ts: Timestamp = serde_json::from_str(ts_str).unwrap();
        assert_eq!(ts, Timestamp(1588859442000000));
        assert_eq!(ts.to_param_value(), "1588859442.000000");
    }

    #[test]
    fn preserve_precision_str_1_dp() {
        let ts_str = "\"1588859442.1\"";
        let ts: Timestamp = serde_json::from_str(ts_str).unwrap();
        assert_eq!(ts, Timestamp(1588859442100000));
        assert_eq!(ts.to_param_value(), "1588859442.100000");
    }
}
