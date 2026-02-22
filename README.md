# c4rs

A Rust library for defining [C4 model](https://c4model.com/) architectures as typed, composable structures with pluggable serialization -- starting with Structurizr DSL output.

## Overview

c4rs lets you build C4 architecture models (Context, Container, Component, Code) using Rust's type system. Models are constructed with validated builders and can be serialized to formats like [Structurizr DSL](https://docs.structurizr.com/dsl) for rendering with tools like Structurizr Lite or the Structurizr CLI.

The library is split into two crates behind an umbrella `c4rs` package:

- **`c4rs-core`** -- Core C4 types: `Person`, `SoftwareSystem`, `Container`, `Component`, `CodeElement`, and generic `Relationship<S, T>`. All types use fallible builders with input validation.
- **`c4rs-structurizr-dsl`** -- Serializes a composed model to Structurizr DSL, including support for views and styles.

## Installation

```toml
[dependencies]
c4rs = { git = "https://github.com/Merge-42/c4rs" }
```

## Quick Start

```rust
use c4rs::{
    Person, SoftwareSystem, Container, ContainerType, Location,
    DslSerializer, ViewConfiguration, ViewType, ElementStyle,
};

// Define elements
let user = Person::builder()
    .name("User".into())
    .description("A user of the system".into())
    .location(Location::External)
    .build()
    .unwrap();

let web_app = Container::builder()
    .name("Web App".into())
    .description("Frontend application".into())
    .container_type(ContainerType::WebApplication)
    .technology("React".into())
    .build()
    .unwrap();

let api = SoftwareSystem::builder()
    .name("API".into())
    .description("Backend API service".into())
    .add_container(web_app)
    .build()
    .unwrap();

// Serialize to Structurizr DSL
let dsl = DslSerializer::new()
    .with_name("Example System")
    .with_description("An example C4 model")
    .add_person(user)
    .add_software_system(api)
    .add_relationship("u", "api", "Uses", Some("HTTPS"))
    .add_view(
        ViewConfiguration::builder()
            .view_type(ViewType::SystemContext)
            .element_identifier("api".into())
            .title("SystemContext".into())
            .build(),
    )
    .add_element_style(
        ElementStyle::builder()
            .identifier("Person".into())
            .shape("person".into())
            .build(),
    )
    .serialize()
    .unwrap();

println!("{dsl}");
```

Output:

```dsl
workspace "Example System" "An example C4 model" {
    !identifiers hierarchical

    model {
        u = person "User" "A user of the system"
        api = softwareSystem "API" "Backend API service" {
            wa = container "Web App" "Frontend application" "React" {}
        }

        u -> api "Uses" "HTTPS"
    }

    views {
        systemContext api "SystemContext" {
            include *
        }

        styles {
            element "Person" {
                shape person
            }
        }
    }
}
```

## Core Types

All builders return `Result` -- construction fails if validation rules are violated (empty names, fields exceeding length limits, etc.).

### Person

```rust
use c4rs::{Person, Location};

let person = Person::builder()
    .name("Customer".into())
    .description("A registered customer".into())
    .location(Location::External)       // optional, defaults to Internal
    .technology("Mobile App".into())    // optional
    .build()?;
```

### SoftwareSystem

Systems own containers. Add them via the builder or after construction:

```rust
use c4rs::{SoftwareSystem, Container, ContainerType};

let db = Container::builder()
    .name("Database".into())
    .description("PostgreSQL database".into())
    .container_type(ContainerType::Database)
    .technology("PostgreSQL 15".into())
    .build()?;

let system = SoftwareSystem::builder()
    .name("E-Commerce Platform".into())
    .description("Online shopping system".into())
    .add_container(db)
    .build()?;

// Or mutate after construction:
// system.add_container(another_container);
```

### Container

Containers own components:

```rust
use c4rs::{Container, ContainerType, Component};

let handler = Component::builder()
    .name("OrderHandler".into())
    .description("Handles order processing".into())
    .technology("Rust".into())
    .responsibilities(vec![
        "Create order".into(),
        "Process payment".into(),
    ])
    .build()?;

let api = Container::builder()
    .name("Web API".into())
    .description("REST API endpoints".into())
    .container_type(ContainerType::Api)
    .technology("Rust/Axum".into())
    .add_component(handler)
    .build()?;
```

### Component

Components own code elements:

```rust
use c4rs::{Component, CodeElement, CodeType};

let func = CodeElement::builder()
    .name("calculate_total".into())
    .description("Calculates order total".into())
    .code_type(CodeType::Function)
    .language("Rust".into())
    .file_path("src/orders/calculator.rs".into())
    .build()?;

let component = Component::builder()
    .name("Calculator".into())
    .description("Price calculation logic".into())
    .add_code_element(func)
    .build()?;
```

### Relationships

Generic relationships between any two element types:

```rust
use c4rs::c4::{create_relationship, Relationship, InteractionStyle};
use c4rs::{Person, Container, ContainerType};

let rel = create_relationship(user, api, "Uses".into())?;

// Or with the builder for more control:
let rel: Relationship<Person, Container> = Relationship::builder()
    .source(user)
    .target(api)
    .description("Uses".into())
    .technology("HTTPS".into())
    .interaction_style(InteractionStyle::Asynchronous)
    .build()?;
```

Type aliases are provided for common pairings:

```rust
use c4rs::c4::{
    PersonRelationship,              // Relationship<Person, Person>
    PersonToContainerRelationship,   // Relationship<Person, Container>
    ContainerRelationship,           // Relationship<Container, Container>
    ComponentRelationship,           // Relationship<Component, Component>
    ComponentToCodeRelationship,     // Relationship<Component, CodeElement>
};
```

## Structurizr DSL Serialization

The `DslSerializer` is a consuming-builder that assembles a complete Structurizr DSL workspace:

```rust
use c4rs::{DslSerializer, ViewConfiguration, ViewType, ElementStyle, RelationshipStyle};

let dsl = DslSerializer::new()
    .with_name("My System")
    .with_description("Architecture model")
    .add_person(user)
    .add_software_system(system)
    .add_relationship("u", "ms", "Calls", Some("gRPC"))
    .add_view(
        ViewConfiguration::builder()
            .view_type(ViewType::SystemLandscape)
            .element_identifier("*".into())
            .title("Landscape".into())
            .build(),
    )
    .add_element_style(
        ElementStyle::builder()
            .identifier("Person".into())
            .shape("person".into())
            .background("#08427B".into())
            .color("#ffffff".into())
            .build(),
    )
    .add_relationship_style(
        RelationshipStyle::builder()
            .color("#707070".into())
            .dashed(true)
            .build(),
    )
    .serialize()?;
```

### Identifiers

Identifiers are auto-generated during serialization from element names -- the first letter of each word, lowercased:

| Name              | Identifier |
| ----------------- | ---------- |
| `User`            | `u`        |
| `Software System` | `ss`       |
| `Web App`         | `wa`       |

Collisions are resolved by appending a counter (`u`, `u1`, `u2`, ...). When adding relationships via `add_relationship`, use the identifier that will be generated for each element.

The output always includes `!identifiers hierarchical`, so nested elements use dot-separated paths (e.g. `ss.wa` for a container inside a system).

### Views

Supported view types: `SystemContext`, `Container`, `Component`, `SystemLandscape`, `Filtered`, `Dynamic`, `Deployment`, `Custom`.

### Styles

`ElementStyle` supports: `background`, `color`, `shape`, `size`, `stroke`, `stroke_width`.

`RelationshipStyle` supports: `thickness`, `color`, `router`, `dashed`.

## Composition Model

Elements form a strict ownership hierarchy -- no separate `Workspace` or `Model` struct is needed at the type level:

```text
SoftwareSystem
  └── Container[]
        └── Component[]
              └── CodeElement[]
```

The `DslSerializer` / `WorkspaceSerializer` composes these into a full workspace for serialization.

## Validation

All builders enforce validation rules:

| Field              | Constraint                |
| ------------------ | ------------------------- |
| `name`             | Non-empty, max 255 chars  |
| `description`      | Non-empty, max 1000 chars |
| `technology`       | Max 255 chars             |
| `language`         | Max 255 chars             |
| `file_path`        | Max 512 chars             |
| `responsibilities` | Each max 500 chars        |

Builders return typed errors (`PersonError`, `ContainerError`, etc.) that wrap `ValidationError`.

## Project Structure

```text
c4rs/
├── src/lib.rs                          # Umbrella crate re-exports
├── crates/
│   ├── c4rs-core/                      # Core C4 types and validation
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── constants.rs            # Validation limits
│   │       ├── validation.rs           # Shared validation logic
│   │       └── c4/
│   │           ├── mod.rs
│   │           ├── element.rs          # Element trait, enums
│   │           ├── macros.rs           # impl_element! macro
│   │           ├── context.rs          # Person, SoftwareSystem
│   │           ├── container.rs        # Container
│   │           ├── component.rs        # Component
│   │           ├── code.rs             # CodeElement
│   │           └── relationship.rs     # Relationship<S, T>
│   │
│   └── c4rs-structurizr-dsl/           # Structurizr DSL serializer
│       └── src/
│           ├── lib.rs
│           ├── error.rs                # DslError
│           ├── structurizr_dsl.rs      # DslSerializer (facade)
│           ├── workspace_serializer.rs # Core serialization logic
│           ├── views_serializer.rs     # View rendering
│           ├── styles.rs               # ElementStyle, RelationshipStyle
│           ├── styles_serializer.rs    # Style rendering
│           ├── identifier_generator.rs # Auto-identifier generation
│           ├── writer.rs               # Indentation-aware DSL writer
│           └── templates/              # Askama templates for DSL fragments
├── examples/                           # Runnable examples
└── tests/integration/                  # Integration tests
```

## Development

```bash
cargo build              # Build all crates
cargo test               # Run all tests
cargo test -p c4rs-core  # Test core only
cargo clippy             # Lint (unwrap/expect/panic denied)
cargo fmt                # Format
cargo doc --no-deps      # Generate docs
cargo run --example smoke_test_1  # Run basic example
cargo run --example self_diagram  # Generate C4 diagram of c4rs itself
```

Rust 2024 edition. Uses `bon` for builder derivation, `askama` for DSL templates, `thiserror` for error types.
