//! C4RS - C4 Model Types Library
//!
//! A Rust library providing types that represent the C4 model (Context, Container, Component, Code)
//! for architecture documentation and diagram generation.
//!
//! # Quick Start
//!
//! ```
//! use c4rs::c4::{Person, Location, ElementIdentifier};
//!
//! // Create a person
//! let person = Person::builder()
//!     .name("Alice".into())
//!     .description("System administrator".into())
//!     .location(Location::Internal)
//!     .build();
//!
//! // Types implement serde::Serialize
//! // With serde_json: to_string_pretty(&person)
//! ```

pub use c4rs_core::ElementIdentifier;
pub use c4rs_core::c4;
pub use c4rs_core::{CodeElement, Component, Container, Person, Relationship, SoftwareSystem};
pub use c4rs_core::{CodeType, ContainerType, ElementType, InteractionStyle, Location};

pub use c4rs_structurizr_dsl::StructurizrDslError;
pub use c4rs_structurizr_dsl::StructurizrDslSerializer;
pub use c4rs_structurizr_dsl::{ElementStyle, RelationshipStyle, StylesSerializer};
pub use c4rs_structurizr_dsl::{ViewConfiguration, ViewType, ViewsSerializer};
