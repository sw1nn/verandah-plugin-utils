//! Serde utilities for plugin configuration.

use serde::de::{Deserializer, IgnoredAny};
use serde::Deserialize;

/// A serde-deserializable value that discards whatever it receives.
///
/// Useful with `#[serde(flatten)]` to capture unknown config field *keys*
/// without pulling in a specific value crate like `toml::Value`:
///
/// ```ignore
/// #[derive(Deserialize)]
/// struct Config {
///     known_field: String,
///     #[serde(flatten)]
///     unknown: HashMap<String, IgnoredValue>,
/// }
/// ```
pub struct IgnoredValue;

impl<'de> Deserialize<'de> for IgnoredValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        IgnoredAny::deserialize(deserializer)?;
        Ok(IgnoredValue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::de::IntoDeserializer;

    #[test]
    fn test_ignored_value_deserializes_u64() -> Result<(), serde::de::value::Error> {
        let _: IgnoredValue = IgnoredValue::deserialize(42_u64.into_deserializer())?;
        Ok(())
    }

    #[test]
    fn test_ignored_value_deserializes_string() -> Result<(), serde::de::value::Error> {
        let _: IgnoredValue =
            IgnoredValue::deserialize("hello".to_owned().into_deserializer())?;
        Ok(())
    }

    #[test]
    fn test_ignored_value_deserializes_bool() -> Result<(), serde::de::value::Error> {
        let _: IgnoredValue = IgnoredValue::deserialize(true.into_deserializer())?;
        Ok(())
    }
}
