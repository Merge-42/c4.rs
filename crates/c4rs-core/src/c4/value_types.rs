use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

/// Type-safe wrapper for element identifiers.
///
/// Uses UUID internally for uniqueness.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ElementIdentifier(uuid::Uuid);

impl ElementIdentifier {
    /// Creates a new random element identifier.
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    /// Returns the underlying UUID.
    pub fn inner(&self) -> uuid::Uuid {
        self.0
    }
}

impl Default for ElementIdentifier {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ElementIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Serialize for ElementIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ElementIdentifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let uuid = uuid::Uuid::parse_str(&s).map_err(serde::de::Error::custom)?;
        Ok(Self(uuid))
    }
}
