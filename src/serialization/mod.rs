//! Structurizr DSL Serialization
//!
//! This module provides serialization of C4 models to Structurizr DSL format.

pub mod component_serializer;
pub mod container_serializer;
pub mod error;
pub mod hierarchy_serializer;
pub mod person_serializer;
pub mod relationship_serializer;
pub mod software_system_serializer;
pub mod structurizr_dsl;
pub mod styles_serializer;
pub mod traits;
pub mod validator;
pub mod views_serializer;
pub mod writer;

pub use error::StructurizrDslError;
pub use hierarchy_serializer::HierarchySerializer;
pub use structurizr_dsl::StructurizrDslSerializer;
pub use styles_serializer::{ElementStyle, RelationshipStyle, StylesSerializer};
pub use traits::{ElementSerializer, escape_dsl_string, format_identifier};
pub use validator::{HierarchyValidator, ParentChildRelationship};
pub use views_serializer::{ViewConfiguration, ViewsSerializer};
