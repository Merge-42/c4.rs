//! Generates a Structurizr DSL diagram of the c4rs codebase itself.
//!
//! Run with: cargo run --example self_diagram

use c4rs::c4::ContainerType;
use c4rs::{
    CodeElement, CodeType, Component, Container, DslSerializer, ElementStyle, Person,
    RelationshipStyle, SoftwareSystem, ViewConfiguration, ViewType,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // --- People ---

    let library_consumer = Person::builder()
        .name("Library Consumer".into())
        .description("A Rust developer using c4rs to model their architecture".into())
        .build()?;

    // --- Library Consumer's Project Code ---
    // The consumer's own codebase that depends on c4rs

    let project_lib_rs = CodeElement::builder()
        .name("lib.rs".into())
        .description("Consumer's crate root, public API".into())
        .code_type(CodeType::Module)
        .language("Rust".into())
        .file_path("my-project/src/lib.rs".into())
        .build()?;

    let project_models_rs = CodeElement::builder()
        .name("models.rs".into())
        .description("Domain models and C4 element definitions".into())
        .code_type(CodeType::Module)
        .language("Rust".into())
        .file_path("my-project/src/models.rs".into())
        .build()?;

    let project_diagrams_rs = CodeElement::builder()
        .name("diagrams.rs".into())
        .description("C4 view configurations and diagram definitions".into())
        .code_type(CodeType::Module)
        .language("Rust".into())
        .file_path("my-project/src/diagrams.rs".into())
        .build()?;

    let project_api_component = Component::builder()
        .name("Public API".into())
        .description("Public API modules exposing c4rs types".into())
        .technology("Rust".into())
        .add_code_element(&project_lib_rs)
        .build()?;

    let project_models_component = Component::builder()
        .name("Domain Models".into())
        .description("Domain-specific C4 model definitions".into())
        .technology("Rust".into())
        .add_code_element(&project_models_rs)
        .build()?;

    let project_diagrams_component = Component::builder()
        .name("View Config".into())
        .description("Diagram and view configurations".into())
        .technology("Rust".into())
        .add_code_element(&project_diagrams_rs)
        .build()?;

    let project_container = Container::builder()
        .name("my-project".into())
        .description("Consumer's project that depends on c4rs".into())
        .container_type(ContainerType::WebApplication)
        .technology("Rust".into())
        .add_component(&project_api_component)
        .add_component(&project_models_component)
        .add_component(&project_diagrams_component)
        .build()?;

    // --- c4rs-core (Container) ---
    // The core library crate - this is a deployable unit (library)

    // Root-level modules - these expose the public API
    let core_lib_rs = CodeElement::builder()
        .name("lib.rs".into())
        .description("Crate root, public exports".into())
        .code_type(CodeType::Module)
        .language("Rust".into())
        .file_path("c4rs-core/src/lib.rs".into())
        .build()?;

    let core_constants_rs = CodeElement::builder()
        .name("constants.rs".into())
        .description("Validation limits (MAX_NAME_LENGTH, etc.)".into())
        .code_type(CodeType::Module)
        .language("Rust".into())
        .file_path("c4rs-core/src/constants.rs".into())
        .build()?;

    let core_validation_rs = CodeElement::builder()
        .name("validation.rs".into())
        .description("Input validation functions".into())
        .code_type(CodeType::Module)
        .language("Rust".into())
        .file_path("c4rs-core/src/validation.rs".into())
        .build()?;

    // c4 module - exposes the core C4 types
    let c4_mod_rs = CodeElement::builder()
        .name("c4/mod.rs".into())
        .description("Module declarations and re-exports".into())
        .code_type(CodeType::Module)
        .language("Rust".into())
        .file_path("c4rs-core/src/c4/mod.rs".into())
        .build()?;

    let element_rs = CodeElement::builder()
        .name("element.rs".into())
        .description("Element trait and enums (ElementType, Location, ContainerType, etc.)".into())
        .code_type(CodeType::Module)
        .language("Rust".into())
        .file_path("c4rs-core/src/c4/element.rs".into())
        .build()?;

    let macros_rs = CodeElement::builder()
        .name("macros.rs".into())
        .description("impl_element! macro for implementing Element trait".into())
        .code_type(CodeType::Module)
        .language("Rust".into())
        .file_path("c4rs-core/src/c4/macros.rs".into())
        .build()?;

    let context_rs = CodeElement::builder()
        .name("context.rs".into())
        .description("Person and SoftwareSystem types".into())
        .code_type(CodeType::Module)
        .language("Rust".into())
        .file_path("c4rs-core/src/c4/context.rs".into())
        .build()?;

    let container_rs = CodeElement::builder()
        .name("container.rs".into())
        .description("Container type with nested component ownership".into())
        .code_type(CodeType::Module)
        .language("Rust".into())
        .file_path("c4rs-core/src/c4/container.rs".into())
        .build()?;

    let component_rs = CodeElement::builder()
        .name("component.rs".into())
        .description("Component type with nested code element ownership".into())
        .code_type(CodeType::Module)
        .language("Rust".into())
        .file_path("c4rs-core/src/c4/component.rs".into())
        .build()?;

    let code_rs = CodeElement::builder()
        .name("code.rs".into())
        .description("CodeElement type (Class, Struct, Function, etc.)".into())
        .code_type(CodeType::Module)
        .language("Rust".into())
        .file_path("c4rs-core/src/c4/code.rs".into())
        .build()?;

    let relationship_rs = CodeElement::builder()
        .name("relationship.rs".into())
        .description("Generic Relationship<S, T> type".into())
        .code_type(CodeType::Module)
        .language("Rust".into())
        .file_path("c4rs-core/src/c4/relationship.rs".into())
        .build()?;

    // Components = modules that expose public API
    let root_modules_component = Component::builder()
        .name("Root API".into())
        .description("lib.rs, constants.rs, validation.rs - root module exports".into())
        .technology("Rust".into())
        .add_code_element(&core_lib_rs)
        .add_code_element(&core_constants_rs)
        .add_code_element(&core_validation_rs)
        .build()?;

    let c4_module_component = Component::builder()
        .name("C4 Types".into())
        .description(
            "Core C4 types: Element, Person, SoftwareSystem, Container, Component, etc.".into(),
        )
        .technology("Rust".into())
        .add_code_element(&c4_mod_rs)
        .add_code_element(&element_rs)
        .add_code_element(&macros_rs)
        .add_code_element(&context_rs)
        .add_code_element(&container_rs)
        .add_code_element(&component_rs)
        .add_code_element(&code_rs)
        .add_code_element(&relationship_rs)
        .build()?;

    let core_container = Container::builder()
        .name("c4rs-core".into())
        .description("Core C4 model types, traits, validation, and builders".into())
        .container_type(ContainerType::Other("Library".into()))
        .technology("Rust".into())
        .add_component(&root_modules_component)
        .add_component(&c4_module_component)
        .build()?;

    // --- c4rs-structurizr-dsl (Container) ---
    // DSL serialization module

    // Code elements = implementation details (not public API modules)
    let serializer_code = CodeElement::builder()
        .name("serializer.rs".into())
        .description("DslSerializer and consuming-builder facade".into())
        .code_type(CodeType::Module)
        .language("Rust".into())
        .file_path("c4rs-structurizr-dsl/src/serializer.rs".into())
        .build()?;

    let workspace_code = CodeElement::builder()
        .name("workspace.rs".into())
        .description("WorkspaceSerializer - walks model and emits DSL".into())
        .code_type(CodeType::Module)
        .language("Rust".into())
        .file_path("c4rs-structurizr-dsl/src/workspace.rs".into())
        .build()?;

    let identifier_code = CodeElement::builder()
        .name("identifier.rs".into())
        .description("IdentifierGenerator - auto-generates DSL identifiers".into())
        .code_type(CodeType::Module)
        .language("Rust".into())
        .file_path("c4rs-structurizr-dsl/src/identifier.rs".into())
        .build()?;

    let writer_code = CodeElement::builder()
        .name("writer.rs".into())
        .description("DslWriter - indentation-aware string builder".into())
        .code_type(CodeType::Module)
        .language("Rust".into())
        .file_path("c4rs-structurizr-dsl/src/writer.rs".into())
        .build()?;

    let views_code = CodeElement::builder()
        .name("views.rs".into())
        .description("ViewsSerializer - renders view blocks".into())
        .code_type(CodeType::Module)
        .language("Rust".into())
        .file_path("c4rs-structurizr-dsl/src/views.rs".into())
        .build()?;

    let styles_code = CodeElement::builder()
        .name("styles.rs".into())
        .description("StylesSerializer - renders style blocks".into())
        .code_type(CodeType::Module)
        .language("Rust".into())
        .file_path("c4rs-structurizr-dsl/src/styles.rs".into())
        .build()?;

    // This is the public API module for the DSL crate
    let dsl_mod_rs = CodeElement::builder()
        .name("lib.rs".into())
        .description("Public exports for structurizr-dsl crate".into())
        .code_type(CodeType::Module)
        .language("Rust".into())
        .file_path("c4rs-structurizr-dsl/src/lib.rs".into())
        .build()?;

    // Components = modules that expose public API
    let dsl_public_api_component = Component::builder()
        .name("Public API".into())
        .description("lib.rs - public exports (DslSerializer, etc.)".into())
        .technology("Rust".into())
        .add_code_element(&dsl_mod_rs)
        .build()?;

    let dsl_impl_component = Component::builder()
        .name("Implementation".into())
        .description("Internal serialization implementation details".into())
        .technology("Rust".into())
        .add_code_element(&serializer_code)
        .add_code_element(&workspace_code)
        .add_code_element(&identifier_code)
        .add_code_element(&writer_code)
        .add_code_element(&views_code)
        .add_code_element(&styles_code)
        .build()?;

    let dsl_container = Container::builder()
        .name("c4rs-structurizr-dsl".into())
        .description("Structurizr DSL serialization module".into())
        .container_type(ContainerType::Other("Library".into()))
        .technology("Rust".into())
        .add_component(&dsl_public_api_component)
        .add_component(&dsl_impl_component)
        .build()?;

    // --- Umbrella crate (Container) ---
    // This is the main c4rs crate that re-exports everything

    let umbrella_lib_rs = CodeElement::builder()
        .name("lib.rs".into())
        .description("Umbrella crate root, re-exports from core and dsl".into())
        .code_type(CodeType::Module)
        .language("Rust".into())
        .file_path("c4rs/src/lib.rs".into())
        .build()?;

    let umbrella_component = Component::builder()
        .name("Public API".into())
        .description("Re-exports core types and DslSerializer".into())
        .technology("Rust".into())
        .add_code_element(&umbrella_lib_rs)
        .build()?;

    let umbrella_container = Container::builder()
        .name("c4rs".into())
        .description("Umbrella crate that re-exports core types and DSL serializer".into())
        .container_type(ContainerType::Other("Library".into()))
        .technology("Rust".into())
        .add_component(&umbrella_component)
        .build()?;

    // --- The c4rs system ---

    let c4rs_system = SoftwareSystem::builder()
        .name("c4rs".into())
        .description(
            "Rust library for defining C4 architecture models with pluggable serialization".into(),
        )
        .add_container(&umbrella_container)
        .add_container(&core_container)
        .add_container(&dsl_container)
        .build()?;

    // --- Library Consumer's Project (external system) ---

    let project_system = SoftwareSystem::builder()
        .name("my-project".into())
        .description("Library consumer's project using c4rs".into())
        .add_container(&project_container)
        .build()?;

    // --- External systems ---

    let structurizr = SoftwareSystem::builder()
        .name("Structurizr".into())
        .description("Renders architecture diagrams from Structurizr DSL files".into())
        .build()?;

    // --- Serialize ---

    let dsl = DslSerializer::new()
        .with_name("c4rs Architecture")
        .with_description("C4 model of the c4rs Rust library itself")
        .add_person(&library_consumer)
        .add_software_system(&project_system)
        .add_software_system(&c4rs_system)
        .add_software_system(&structurizr)
        .add_relationship(
            &library_consumer,
            &project_container,
            "Writes and maintains",
            Some("Rust code"),
        )
        .add_relationship(
            &project_container,
            &umbrella_container,
            "Uses",
            Some("Cargo dependency"),
        )
        .add_relationship(
            &umbrella_container,
            &core_container,
            "Re-exports types from",
            None,
        )
        .add_relationship(
            &umbrella_container,
            &dsl_container,
            "Re-exports serializer from",
            None,
        )
        .add_relationship(
            &dsl_container,
            &core_container,
            "Depends on",
            Some("Cargo path dep"),
        )
        .add_relationship(
            &c4rs_system,
            &structurizr,
            "Produces DSL for",
            Some("Structurizr DSL"),
        )
        // System landscape view
        .add_view(
            ViewConfiguration::builder()
                .view_type(ViewType::SystemLandscape)
                .element_identifier("*".into())
                .title("System Landscape".into())
                .include_elements(vec!["*".into()])
                .build(),
        )
        // System context view of c4rs
        .add_view(
            ViewConfiguration::builder()
                .view_type(ViewType::SystemContext)
                .element_identifier("c".into())
                .title("c4rs System Context".into())
                .include_elements(vec!["*".into()])
                .build(),
        )
        // Container view of c4rs
        .add_view(
            ViewConfiguration::builder()
                .view_type(ViewType::Container)
                .element_identifier("c".into())
                .title("c4rs Containers".into())
                .include_elements(vec!["*".into()])
                .build(),
        )
        // Container view of project
        .add_view(
            ViewConfiguration::builder()
                .view_type(ViewType::Container)
                .element_identifier("m".into())
                .title("my-project Containers".into())
                .include_elements(vec!["*".into()])
                .build(),
        )
        // Component view of c4rs-core
        .add_view(
            ViewConfiguration::builder()
                .view_type(ViewType::Component)
                .element_identifier("c.c2".into())
                .title("c4rs-core Components".into())
                .include_elements(vec!["*".into()])
                .build(),
        )
        // Component view of c4rs-structurizr-dsl
        .add_view(
            ViewConfiguration::builder()
                .view_type(ViewType::Component)
                .element_identifier("c.c3".into())
                .title("c4rs-structurizr-dsl Components".into())
                .include_elements(vec!["*".into()])
                .build(),
        )
        // Styles
        .add_element_style(
            ElementStyle::builder()
                .identifier("Person".into())
                .shape("person".into())
                .background("#08427B".into())
                .color("#ffffff".into())
                .build(),
        )
        .add_element_style(
            ElementStyle::builder()
                .identifier("Software System".into())
                .background("#1168BD".into())
                .color("#ffffff".into())
                .build(),
        )
        .add_element_style(
            ElementStyle::builder()
                .identifier("Container".into())
                .background("#438DD5".into())
                .color("#ffffff".into())
                .build(),
        )
        .add_element_style(
            ElementStyle::builder()
                .identifier("Component".into())
                .background("#85BBF0".into())
                .color("#000000".into())
                .build(),
        )
        .add_element_style(
            ElementStyle::builder()
                .identifier("Code".into())
                .shape("hexagon".into())
                .background("#E3F2FD".into())
                .color("#000000".into())
                .build(),
        )
        .add_relationship_style(
            RelationshipStyle::builder()
                .identifier("Relationship".into())
                .color("#707070".into())
                .dashed(false)
                .build(),
        )
        .serialize()?;

    println!("{dsl}");

    Ok(())
}
