use thiserror::Error;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum StructurizrDslError {
    #[error("element not found: {0}")]
    ElementNotFound(String),

    #[error("circular relationship detected: {0}")]
    CircularRelationship(String),

    #[error("invalid parent type for {child}: expected {expected}, got {actual}")]
    InvalidParentType {
        child: String,
        expected: String,
        actual: String,
    },

    #[error("serialization error: {0}")]
    SerializationError(String),

    #[error("missing required property: {0}")]
    MissingProperty(String),

    #[error("duplicate identifier: {0}")]
    DuplicateIdentifier(String),

    #[error("unsupported element type: {0}")]
    UnsupportedElementType(String),

    #[error("template error: {0}")]
    TemplateError(String),
}

impl From<askama::Error> for StructurizrDslError {
    fn from(err: askama::Error) -> Self {
        StructurizrDslError::TemplateError(err.to_string())
    }
}
