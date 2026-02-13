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
//!     .with_name("Alice".try_into().unwrap())
//!     .with_description("System administrator".try_into().unwrap())
//!     .with_location(Location::Internal)
//!     .build()
//!     .unwrap();
//! ```

pub mod c4;
