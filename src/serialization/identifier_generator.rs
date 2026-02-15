//! Identifier generation for Structurizr DSL elements.
//!
//! This module provides automatic generation of short identifiers for C4 model elements.
//! Identifiers are generated from element names by taking the first letter of each word
//! and converting to lowercase.
//!
//! # Examples
//!
//! ```
//! use c4rs::serialization::identifier_generator::IdentifierGenerator;
//!
//! assert_eq!(IdentifierGenerator::generate("User"), "u");
//! assert_eq!(IdentifierGenerator::generate("Software System"), "ss");
//! assert_eq!(IdentifierGenerator::generate("Web Application"), "wa");
//! ```

use std::collections::HashSet;

/// Generates short identifiers for Structurizr DSL elements.
///
/// Identifiers are created by taking the first letter of each word in the element name,
/// converting to lowercase. Collisions are resolved by appending numbers.
#[derive(Debug, Default)]
pub struct IdentifierGenerator {}

impl IdentifierGenerator {
    /// Generate an identifier from an element name.
    ///
    /// Takes the first letter of each word, converts to lowercase.
    ///
    /// # Examples
    ///
    /// ```
    /// use c4rs::serialization::identifier_generator::IdentifierGenerator;
    ///
    /// assert_eq!(IdentifierGenerator::generate("User"), "u");
    /// assert_eq!(IdentifierGenerator::generate("Database Schema"), "ds");
    /// ```
    pub fn generate(name: &str) -> String {
        name.split_whitespace()
            .filter(|s| !s.is_empty())
            .take(2) // Max 2 words for compact identifiers
            .map(|s| {
                s.chars()
                    .next()
                    .unwrap_or_default()
                    .to_lowercase()
                    .to_string()
            })
            .collect()
    }

    /// Generate a unique identifier, avoiding collisions with used identifiers.
    ///
    /// If the generated identifier is already in use, appends a numeric suffix.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashSet;
    /// use c4rs::serialization::identifier_generator::IdentifierGenerator;
    ///
    /// let mut used = HashSet::new();
    /// used.insert("ds".to_string()); // "Database Schema" produces "ds"
    ///
    /// assert_eq!(IdentifierGenerator::generate_unique("Database", &used), "d");
    /// assert_eq!(IdentifierGenerator::generate_unique("Database Schema", &used), "ds1");
    /// ```
    pub fn generate_unique(name: &str, used: &HashSet<String>) -> String {
        let mut identifier = Self::generate(name);
        let mut counter = 1;

        while used.contains(&identifier) {
            identifier = format!("{}{}", Self::generate(name), counter);
            counter += 1;
        }

        identifier
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_generate_single_word() {
        assert_eq!(IdentifierGenerator::generate("User"), "u");
        assert_eq!(IdentifierGenerator::generate("System"), "s");
        assert_eq!(IdentifierGenerator::generate("API"), "a");
    }

    #[test]
    fn test_generate_two_words() {
        assert_eq!(IdentifierGenerator::generate("Software System"), "ss");
        assert_eq!(IdentifierGenerator::generate("Web Application"), "wa");
        assert_eq!(IdentifierGenerator::generate("Database Schema"), "ds");
    }

    #[test]
    fn test_generate_empty() {
        assert_eq!(IdentifierGenerator::generate(""), "");
    }

    #[test]
    fn test_generate_unique_no_collision() {
        let used = HashSet::new();
        assert_eq!(IdentifierGenerator::generate_unique("User", &used), "u");
    }

    #[test]
    fn test_generate_unique_with_collision() {
        let mut used = HashSet::new();
        used.insert("u".to_string());

        assert_eq!(IdentifierGenerator::generate_unique("User", &used), "u1");
    }

    #[test]
    fn test_generate_unique_multiple_collisions() {
        let mut used = HashSet::new();
        used.insert("u".to_string());
        used.insert("u1".to_string());

        assert_eq!(IdentifierGenerator::generate_unique("User", &used), "u2");
    }
}
