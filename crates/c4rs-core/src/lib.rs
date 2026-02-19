//! C4RS Core - C4 Model Types Library
//!
//! A Rust library providing types that represent the C4 model (Context, Container, Component, Code)
//! for architecture documentation.
//!
//! # Quick Start
//!
//! ```
//! use c4rs_core::c4::{Person, Location};
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

pub mod c4;

pub use c4::code::CodeElement;
pub use c4::component::{Component, ComponentError};
pub use c4::container::{Container, ContainerError};
pub use c4::context::{Person, PersonError, SoftwareSystem, SoftwareSystemError};
pub use c4::element::{CodeType, ContainerType, Element, ElementType, InteractionStyle, Location};
pub use c4::relationship::{Relationship, RelationshipError, create_relationship};
