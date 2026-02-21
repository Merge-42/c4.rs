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

pub mod code;
pub mod component;
pub mod container;
pub mod context;
pub mod element;
pub mod macros;
pub mod relationship;

pub use code::CodeElement;
pub use component::{Component, ComponentError};
pub use container::{Container, ContainerError};
pub use context::{Person, PersonError, SoftwareSystem, SoftwareSystemError};
pub use element::{CodeType, ContainerType, Element, ElementType, InteractionStyle, Location};
pub use relationship::{Relationship, RelationshipError, create_relationship};
