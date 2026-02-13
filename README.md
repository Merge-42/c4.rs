# C4RS - C4 Model Types Library

A Rust library providing types that represent the C4 model (Context, Container, Component, Code) for architecture documentation and diagram generation.

## Overview

This library provides type-safe Rust types for the [C4 model](https://c4model.com/) of software architecture. It includes:

- **Core Types**: Person, SoftwareSystem, Container, Component, CodeElement
- **Relationships**: Generic relationships between any Element types
- **Validation**: Construction-time validation with clear error messages
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
    .with_name("Alice".try_into().unwrap())
    .with_description("System administrator".try_into().unwrap())
    .with_location(Location::Internal)
    .build()
    .unwrap();

println!("Person: {}", person.name());
```

## C4 Model Types

### Context Level

```rust
use c4rs::c4::{Person, SoftwareSystem, Location};

// Create a person (user or actor)
let user = Person::builder()
    .with_name("Customer".try_into().unwrap())
    .with_description("A registered customer".try_into().unwrap())
    .with_location(Location::External)
    .build()
    .unwrap();

// Create a software system
let system = SoftwareSystem::builder()
    .with_name("E-Commerce Platform".try_into().unwrap())
    .with_description("Online shopping system".try_into().unwrap())
    .build()
    .unwrap();
```

### Container Level

```rust
use c4rs::c4::{Container, ContainerType};

// Create containers within a system
let web_api = Container::builder()
    .with_name("Web API".try_into().unwrap())
    .with_description("REST API endpoints".try_into().unwrap())
    .with_container_type(ContainerType::Api)
    .with_technology("Rust/Axum".try_into().unwrap())
    .build()
    .unwrap();

let database = Container::builder()
    .with_name("Database".try_into().unwrap())
    .with_description("PostgreSQL database".try_into().unwrap())
    .with_container_type(ContainerType::Database)
    .with_technology("PostgreSQL 15".try_into().unwrap())
    .build()
    .unwrap();
```

### Component Level

```rust
use c4rs::c4::{Component};

// Create components within a container
let order_handler = Component::builder()
    .with_name("OrderHandler".try_into().unwrap())
    .with_description("Handles order processing".try_into().unwrap())
    .add_responsibility("Create order".try_into().unwrap())
    .add_responsibility("Process payment".try_into().unwrap())
    .with_technology("Rust".try_into().unwrap())
    .build()
    .unwrap();
```

### Code Level

```rust
use c4rs::c4::{CodeElement, CodeType};

// Create code elements (classes, functions, etc.)
let calculate_total = CodeElement::builder()
    .with_name("calculateTotal".try_into().unwrap())
    .with_description("Calculates order total".try_into().unwrap())
    .with_code_type(CodeType::Function)
    .with_language("Rust".try_into().unwrap())
    .with_file_path("src/orders/calculator.rs".try_into().unwrap())
    .build()
    .unwrap();
```

## Relationships

Create relationships between any C4 elements:

```rust
use c4rs::c4::{Person, Container, Relationship, InteractionStyle};

// Same type relationship
let relationship1: Relationship<Person, Person> = Relationship::builder()
    .with_source(user1)
    .with_target(user2)
    .with_description("Communicates with".try_into().unwrap())
    .with_interaction_style(InteractionStyle::Synchronous)
    .build()
    .unwrap();

// Cross-level relationship (Person → Container)
let relationship2: Relationship<Person, Container> = Relationship::builder()
    .with_source(customer)
    .with_target(web_api)
    .with_description("Uses".try_into().unwrap())
    .build()
    .unwrap();
```

## JSON Serialization

All types implement serde::Serialize and serde::Deserialize:

```rust
use serde_json::to_string_pretty;

let json = to_string_pretty(&person).unwrap();
println!("{}", json);
```

Output example:

```json
{
  "identifier": "550e8400-e29b-41d4-a716-446655440000",
  "name": "Alice",
  "description": "System administrator",
  "location": "Internal",
  "technology": null
}
```

## Validation

All builders validate fields during construction:

```rust
use c4rs::c4::{Person, PersonError};

let result = Person::builder()
    .with_description("No name provided".try_into().unwrap())
    .build();

match result {
    Ok(person) => println!("Created: {}", person.name()),
    Err(PersonError::MissingName) => println!("Error: Name is required"),
    Err(e) => println!("Error: {}", e),
}
```

## Error Types

| Type                  | Purpose                          |
| --------------------- | -------------------------------- |
| `PersonError`         | Person validation errors         |
| `SoftwareSystemError` | SoftwareSystem validation errors |
| `ContainerError`      | Container validation errors      |
| `ComponentError`      | Component validation errors      |
| `CodeElementError`    | CodeElement validation errors    |
| `RelationshipError`   | Relationship validation errors   |
| `NonEmptyStringError` | String validation errors         |

## Architecture

```
src/
├── lib.rs              # Library root
└── c4/
    ├── mod.rs         # Module declarations
    ├── element.rs     # Element trait + enums
    ├── value_types.rs # ElementIdentifier, NonEmptyString
    ├── context.rs     # Person, SoftwareSystem
    ├── container.rs   # Container
    ├── component.rs   # Component
    ├── code.rs        # CodeElement
    └── relationship.rs # Generic relationships
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

## License

MIT
