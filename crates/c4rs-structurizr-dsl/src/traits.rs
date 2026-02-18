//! Trait for serializing C4 elements to Structurizr DSL format.

use crate::error::StructurizrDslError;

/// Trait for serializing C4 elements to Structurizr DSL format.
///
/// Implement this trait for each element type (Person, SoftwareSystem,
/// Container, Component) to provide custom DSL serialization.
pub trait ElementSerializer {
    /// Serialize the element to Structurizr DSL format.
    ///
    /// Returns a string containing the DSL representation of the element.
    ///
    /// # Errors
    ///
    /// Returns a `StructurizrDslError` if serialization fails.
    fn serialize_structurizr_dsl(&self) -> Result<String, StructurizrDslError>;
}

/// Helper function to escape special characters in DSL strings.
///
/// DSL strings must have internal quotes escaped with backslash.
pub fn escape_dsl_string(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

/// Format element identifier for DSL output.
///
/// Element identifiers must be valid identifiers (alphanumeric + underscore,
/// starting with letter or underscore).
pub fn format_identifier(name: &str) -> String {
    let normalized = name.replace(|c: char| !c.is_alphanumeric() && c != '_', "_");
    normalized
        .chars()
        .next()
        .map(|first| {
            if first.is_ascii_alphabetic() || first == '_' {
                format!("{}{}", first, &normalized[1..])
            } else {
                format!("_{}", normalized)
            }
        })
        .unwrap_or_else(|| "element".to_string())
}
