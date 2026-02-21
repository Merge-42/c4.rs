//! Error types for Structurizr DSL serialization.

use thiserror::Error;

/// Errors that can occur during Structurizr DSL serialization.
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum StructurizrDslError {
    /// Referenced element was not found in the model.
    #[error("element not found: {0}")]
    ElementNotFound(String),

    /// Circular relationship detected in parent-child hierarchy.
    #[error("circular relationship detected: {0}")]
    CircularRelationship(String),

    /// Invalid parent-child relationship (wrong element type).
    #[error("invalid parent type for {child}: expected {expected}, got {actual}")]
    InvalidParentType {
        child: String,
        expected: String,
        actual: String,
    },

    /// Failed to serialize element to DSL format.
    #[error("serialization error: {0}")]
    SerializationError(String),

    /// Missing required property on element.
    #[error("missing required property: {0}")]
    MissingProperty(String),

    /// Duplicate element identifier detected.
    #[error("duplicate identifier: {0}")]
    DuplicateIdentifier(String),

    /// Unsupported element type for serialization.
    #[error("unsupported element type: {0}")]
    UnsupportedElementType(String),

    /// Template rendering error.
    #[error("template error: {0}")]
    TemplateError(String),
}

impl From<askama::Error> for StructurizrDslError {
    fn from(err: askama::Error) -> Self {
        StructurizrDslError::TemplateError(err.to_string())
    }
}
