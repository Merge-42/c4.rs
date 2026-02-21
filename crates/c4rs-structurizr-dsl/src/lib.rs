//! Serialization of C4 models to Structurizr DSL format.

pub mod error;
pub mod identifier_generator;
pub mod structurizr_dsl;
pub mod styles_serializer;
pub mod templates;
pub mod traits;
pub mod views_serializer;
pub mod workspace_serializer;
pub mod writer;

pub use error::StructurizrDslError;
pub use identifier_generator::IdentifierGenerator;
pub use structurizr_dsl::StructurizrDslSerializer;
pub use styles_serializer::{ElementStyle, RelationshipStyle, StylesSerializer};
pub use traits::escape_dsl_string;
pub use views_serializer::{ViewConfiguration, ViewType, ViewsSerializer};
pub use workspace_serializer::WorkspaceSerializer;
