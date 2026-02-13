//! C4 Model Types - A Rust library for representing the C4 model of software architecture.
//!
//! This library provides types for all four C4 levels: Context, Container, Component, and Code.
//! Types follow SOLID principles and support JSON serialization via serde.
//!
//! # Features
//!
//! - **Core Types**: Person, SoftwareSystem, Container, Component, CodeElement
//! - **Relationships**: Generic relationships between any Element types
//! - **Validation**: Construction-time validation with clear error messages
//! - **Serialization**: JSON serialization via serde
//!
//! # Example
//!
//! ```
//! use c4rs::c4::{Person, Location, ElementIdentifier};
//!
//! let person = Person::builder()
//!     .with_name("Alice".try_into().unwrap())
//!     .with_description("System administrator".try_into().unwrap())
//!     .with_location(Location::Internal)
//!     .build()
//!     .unwrap();
//!
//! assert_eq!(person.name(), "Alice");
//! ```

pub mod code;
pub mod component;
pub mod container;
pub mod context;
pub mod element;
pub mod relationship;
pub mod value_types;

pub use code::{CodeElement, CodeElementError};
pub use component::{Component, ComponentError};
pub use container::{Container, ContainerError};
pub use context::{Person, PersonError, SoftwareSystem, SoftwareSystemError};
pub use element::{CodeType, ContainerType, Element, ElementType, InteractionStyle, Location};
pub use relationship::{create_relationship, Relationship, RelationshipBuilder, RelationshipError};
pub use value_types::{ElementIdentifier, NonEmptyString, NonEmptyStringError};
