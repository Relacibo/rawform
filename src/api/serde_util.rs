use serde::{Deserialize, Deserializer};

/// Deserialize a field as `Option<Option<T>>`:
/// - key absent  → `None`           (don't change)
/// - key = null  → `Some(None)`     (clear the value)
/// - key = value → `Some(Some(v))`  (set the value)
///
/// Usage: `#[serde(default, deserialize_with = "deserialize_maybe")]`
pub fn deserialize_maybe<'de, D, T>(de: D) -> Result<Option<Option<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Ok(Some(Option::deserialize(de)?))
}
