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
//!     .name("Alice".try_into().unwrap())
//!     .description("System administrator".try_into().unwrap())
//!     .location(Location::Internal)
//!     .build();
//!
//! // Types implement serde::Serialize
//! // With serde_json: to_string_pretty(&person)
//! ```

pub mod c4;
pub mod serialization;
