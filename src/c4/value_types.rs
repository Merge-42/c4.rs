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

/// Newtype wrapper for non-empty strings with length validation.
///
/// Ensures strings are between 1-255 characters.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct NonEmptyString(String);

impl NonEmptyString {
    /// Minimum allowed length.
    pub const MIN_LENGTH: usize = 1;

    /// Maximum allowed length.
    pub const MAX_LENGTH: usize = 255;

    /// Creates a new NonEmptyString from a validated string.
    ///
    /// Returns an error if the string is empty or too long.
    pub fn new(s: impl Into<String>) -> Result<Self, NonEmptyStringError> {
        let s = s.into();
        if s.is_empty() {
            Err(NonEmptyStringError::Empty)
        } else if s.len() > Self::MAX_LENGTH {
            Err(NonEmptyStringError::TooLong {
                max: Self::MAX_LENGTH,
                actual: s.len(),
            })
        } else {
            Ok(Self(s))
        }
    }

    /// Creates a NonEmptyString without validation.
    ///
    /// # Safety
    ///
    /// The caller must ensure the string is non-empty and within length limits.
    pub unsafe fn new_unchecked(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    /// Returns the underlying string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the string's length.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the string is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl AsRef<str> for NonEmptyString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::ops::Deref for NonEmptyString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for NonEmptyString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for NonEmptyString {
    type Error = NonEmptyStringError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

impl From<NonEmptyString> for String {
    fn from(val: NonEmptyString) -> Self {
        val.0
    }
}

impl From<&str> for NonEmptyString {
    fn from(s: &str) -> Self {
        NonEmptyString::new(s).unwrap()
    }
}

/// Error type for NonEmptyString validation.
#[derive(Debug, thiserror::Error)]
pub enum NonEmptyStringError {
    #[error("string cannot be empty")]
    Empty,

    #[error("string exceeds maximum length of {max} characters (actual: {actual})")]
    TooLong { max: usize, actual: usize },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_empty_string_valid() {
        let s = NonEmptyString::new("hello").unwrap();
        assert_eq!(s.as_str(), "hello");
        assert_eq!(s.len(), 5);
    }

    #[test]
    fn test_non_empty_string_empty() {
        let result = NonEmptyString::new("");
        assert!(result.is_err());
    }

    #[test]
    fn test_non_empty_string_too_long() {
        let long = "x".repeat(256);
        let result = NonEmptyString::new(long);
        assert!(result.is_err());
    }
}
