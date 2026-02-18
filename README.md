# C4RS - C4 Model Types Library

A Rust library providing types that represent the C4 model (Context, Container, Component, Code) for architecture documentation and diagram generation.

## Overview

This library provides type-safe Rust types for the [C4 model](https://c4model.com/) of software architecture. It includes:

- **Core Types**: Person, SoftwareSystem, Container, Component, CodeElement
- **Relationships**: Generic relationships between any Element types
- **Serialization**: JSON serialization via serde for diagram tool integration

## Installation

```toml
[dependencies]
c4rs = "0.1.0"
```

## Quick Start

```rust
use c4rs::c4::{Person, Location, ElementIdentifier};

let person = Person::builder()
    .name("Alice".into())
    .description("System administrator".into())
    .location(Location::Internal)
    .build();

println!("Person: {}", person.name());
```

## Usage Notes

- **Optional string fields**: For fields like `technology`, `language`, `file_path`, you can use `.into()` directly - the builder automatically wraps with `Some()`:
  ```rust
  .technology("Rust".into())           // Sets Some("Rust")
  .technology()                        // Sets None (optional)
  ```
- **Ownership**: The `create_relationship` function takes ownership of elements. Use `.clone()` if you need to reuse them.
- **Defaults**: Location defaults to `Internal` when not specified.

## C4 Model Types

### Context Level

```rust
use c4rs::c4::{Person, SoftwareSystem, Location};

// Create a person (user or actor)
let user = Person::builder()
    .name("Customer".into())
    .description("A registered customer".into())
    .location(Location::External)
    .build();

// Create a software system
let system = SoftwareSystem::builder()
    .name("E-Commerce Platform".into())
    .description("Online shopping system".into())
    .build();
```

### Container Level

```rust
use c4rs::c4::{Container, ContainerType};

// Create containers within a system
let web_api = Container::builder()
    .name("Web API".into())
    .description("REST API endpoints".into())
    .container_type(ContainerType::Api)
    .technology("Rust/Axum".into())
    .build();

let database = Container::builder()
    .name("Database".into())
    .description("PostgreSQL database".into())
    .container_type(ContainerType::Database)
    .technology("PostgreSQL 15".into())
    .build();
```

### Component Level

```rust
use c4rs::c4::Component;

// Create components within a container
let order_handler = Component::builder()
    .name("OrderHandler".into())
    .description("Handles order processing".into())
    .responsibilities(vec![
        "Create order".into(),
        "Process payment".into(),
    ])
    .technology("Rust".into())
    .build();
```

### Code Level

```rust
use c4rs::c4::{CodeElement, CodeType};

// Create code elements (classes, functions, etc.)
let calculate_total = CodeElement::builder()
    .name("calculateTotal".into())
    .description("Calculates order total".into())
    .code_type(CodeType::Function)
    .language("Rust".into())
    .file_path("src/orders/calculator.rs".into())
    .build();
```

## Relationships

Create relationships between any C4 elements:

```rust
use c4rs::c4::{Person, Container, Relationship, InteractionStyle, create_relationship};

// Create a relationship between two elements
let relationship = create_relationship(
    user,
    web_api,
    "Uses".into(),
    Some("HTTP".into()),
    InteractionStyle::Synchronous,
);
```

Or use the Relationship builder directly:

```rust
use c4rs::c4::Relationship;

let relationship: Relationship<Person, Container> = Relationship::builder()
    .source(user)
    .target(web_api)
    .description("Uses".into())
    .interaction_style(InteractionStyle::Synchronous)
    .build();
```

## JSON Serialization

All types implement `serde::Serialize` and `serde::Deserialize`. Add `serde_json` to your `Cargo.toml` to serialize to JSON:

```toml
[dependencies]
serde_json = "1.0"
```

```rust
use serde_json::to_string_pretty;
use c4rs::c4::Person;

let person = Person::builder()
    .name("Alice".into())
    .description("Admin".into())
    .build();

let json = to_string_pretty(&person).unwrap();
println!("{}", json);
```

Output example:

```json
{
  "identifier": null,
  "name": "Alice",
  "description": "System administrator",
  "location": "Internal",
  "technology": null
}
```

## Architecture

```text
src/
├── lib.rs              # Library root
├── cli/                # CLI tool (optional)
└── crates/
    ├── c4rs-core/      # Core C4 types
    │   └── src/c4/
    │       ├── mod.rs         # Module declarations
    │       ├── element.rs     # Element trait + enums
    │       ├── value_types.rs # ElementIdentifier, NonEmptyString
    │       ├── context.rs     # Person, SoftwareSystem
    │       ├── container.rs   # Container
    │       ├── component.rs   # Component
    │       ├── code.rs        # CodeElement
    │       └── relationship.rs # Generic relationships
    └── c4rs-structurizr-dsl/  # Structurizr DSL serialization
```

## Cargo Commands

```bash
# Build
cargo build

# Run tests
cargo test

# Generate documentation
cargo doc --no-deps

# Check code
cargo check

# Format code
cargo fmt

# Run clippy
cargo clippy
```
