//! Types for the C4 model: Person, SoftwareSystem, Container, Component, and CodeElement.

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
pub use element::{
    CodeType, ContainerType, Element, ElementId, ElementType, InteractionStyle, Location,
};
pub use relationship::{Relationship, RelationshipError, create_relationship};
