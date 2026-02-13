//! Structurizr DSL Serialization
//!
//! This module provides serialization of C4 models to Structurizr DSL format.

pub mod component_serializer;
pub mod container_serializer;
pub mod error;
pub mod person_serializer;
pub mod relationship_serializer;
pub mod software_system_serializer;
pub mod structurizr_dsl;
pub mod traits;
pub mod writer;

pub use error::StructurizrDslError;
pub use structurizr_dsl::StructurizrDslSerializer;
pub use traits::{ElementSerializer, escape_dsl_string, format_identifier};
