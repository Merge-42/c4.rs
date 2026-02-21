//! C4RS Core - C4 Model Types Library
//!
//! A Rust library providing types that represent the C4 model (Context, Container, Component, Code)
//! for architecture documentation.
//!
//! See the `c4` module for details.

pub mod constants;
pub mod validation;

pub mod c4;

pub use c4::code::CodeElement;
pub use c4::component::{Component, ComponentError};
pub use c4::container::{Container, ContainerError};
pub use c4::context::{Person, PersonError, SoftwareSystem, SoftwareSystemError};
pub use c4::element::{CodeType, ContainerType, Element, ElementType, InteractionStyle, Location};
pub use c4::relationship::{Relationship, RelationshipError, create_relationship};
