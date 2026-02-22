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

    // --- c4rs-core components ---

    // Root-level modules
    let lib_rs = CodeElement::builder()
        .name("lib.rs".into())
        .description("Crate root, public exports".into())
        .code_type(CodeType::Module)
        .language("Rust".into())
        .file_path("c4rs-core/src/lib.rs".into())
        .build()?;

    let constants_rs = CodeElement::builder()
        .name("constants.rs".into())
        .description("Validation limits (MAX_NAME_LENGTH, etc.)".into())
        .code_type(CodeType::Module)
        .language("Rust".into())
        .file_path("c4rs-core/src/constants.rs".into())
        .build()?;

    let validation_rs = CodeElement::builder()
        .name("validation.rs".into())
        .description("Input validation functions".into())
        .code_type(CodeType::Module)
        .language("Rust".into())
        .file_path("c4rs-core/src/validation.rs".into())
        .build()?;

    // c4 module
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

    let root_modules = Component::builder()
        .name("Root Modules".into())
        .description("lib.rs, constants.rs, validation.rs".into())
        .technology("Rust".into())
        .add_code_element(lib_rs)
        .add_code_element(constants_rs)
        .add_code_element(validation_rs)
        .build()?;

    let c4_module = Component::builder()
        .name("c4 Module".into())
        .description(
            "Core C4 types: Element, Person, SoftwareSystem, Container, Component, etc.".into(),
        )
        .technology("Rust".into())
        .add_code_element(c4_mod_rs)
        .add_code_element(element_rs)
        .add_code_element(macros_rs)
        .add_code_element(context_rs)
        .add_code_element(container_rs)
        .add_code_element(component_rs)
        .add_code_element(code_rs)
        .add_code_element(relationship_rs)
        .build()?;

    let core_container = Container::builder()
        .name("c4rs-core".into())
        .description("Core C4 model types, traits, validation, and builders".into())
        .container_type(ContainerType::Other("Library".into()))
        .technology("Rust".into())
        .add_component(root_modules)
        .add_component(c4_module)
        .build()?;

    // --- c4rs-structurizr-dsl components ---

    let dsl_serializer = Component::builder()
        .name("DslSerializer".into())
        .description("Consuming-builder facade for assembling a complete workspace".into())
        .technology("Rust".into())
        .build()?;

    let workspace_serializer = Component::builder()
        .name("WorkspaceSerializer".into())
        .description("Core serialization engine that walks the model and emits DSL".into())
        .technology("Rust".into())
        .build()?;

    let identifier_generator = Component::builder()
        .name("IdentifierGenerator".into())
        .description("Auto-generates unique DSL identifiers from element names".into())
        .technology("Rust".into())
        .build()?;

    let dsl_writer = Component::builder()
        .name("DslWriter".into())
        .description("Indentation-aware string builder for DSL output".into())
        .technology("Rust".into())
        .build()?;

    let views_serializer = Component::builder()
        .name("ViewsSerializer".into())
        .description("Renders view blocks (systemContext, container, component, etc.)".into())
        .technology("Rust".into())
        .build()?;

    let styles_serializer = Component::builder()
        .name("StylesSerializer".into())
        .description("Renders element and relationship style blocks".into())
        .technology("Rust".into())
        .build()?;

    let askama_templates = Component::builder()
        .name("Askama Templates".into())
        .description("Inline Askama templates for DSL fragment rendering".into())
        .technology("Askama".into())
        .build()?;

    let dsl_container = Container::builder()
        .name("c4rs-structurizr-dsl".into())
        .description("Structurizr DSL serialization module".into())
        .container_type(ContainerType::Other("Library".into()))
        .technology("Rust".into())
        .add_component(dsl_serializer)
        .add_component(workspace_serializer)
        .add_component(identifier_generator)
        .add_component(dsl_writer)
        .add_component(views_serializer)
        .add_component(styles_serializer)
        .add_component(askama_templates)
        .build()?;

    // --- Umbrella crate ---

    let umbrella_container = Container::builder()
        .name("c4rs".into())
        .description("Umbrella crate that re-exports core types and DSL serializer".into())
        .container_type(ContainerType::Other("Library".into()))
        .technology("Rust".into())
        .build()?;

    // --- The system ---

    let c4rs_system = SoftwareSystem::builder()
        .name("c4rs".into())
        .description(
            "Rust library for defining C4 architecture models with pluggable serialization".into(),
        )
        .add_container(umbrella_container)
        .add_container(core_container)
        .add_container(dsl_container)
        .build()?;

    // --- External systems ---

    let structurizr = SoftwareSystem::builder()
        .name("Structurizr".into())
        .description("Renders architecture diagrams from Structurizr DSL files".into())
        .build()?;

    // --- Serialize ---

    // Auto-generated identifiers (first letter of each word, lowercased):
    //   "Library Consumer"      -> "lc"
    //   "c4rs" (system)         -> "c"
    //   "c4rs" (umbrella ctr)  -> "c1"  (collision with system)
    //   "c4rs-core" (ctr)      -> "c2"  (collision)
    //   "c4rs-structurizr-dsl" -> "c3"  (collision)
    //   "Structurizr"          -> "s1"  (collision with StylesSerializer component "s")

    let dsl = DslSerializer::new()
        .with_name("c4rs Architecture")
        .with_description("C4 model of the c4rs Rust library itself")
        .add_person(library_consumer)
        .add_software_system(c4rs_system)
        .add_software_system(structurizr)
        // Library Consumer uses the c4rs system
        .add_relationship("lc", "c", "Uses", Some("Cargo dependency"))
        // c4rs (umbrella) re-exports from c4rs-core
        .add_relationship("c.c1", "c.c2", "Re-exports types from", None)
        // c4rs (umbrella) re-exports from c4rs-structurizr-dsl
        .add_relationship("c.c1", "c.c3", "Re-exports serializer from", None)
        // c4rs-structurizr-dsl depends on c4rs-core
        .add_relationship("c.c3", "c.c2", "Depends on", Some("Cargo path dep"))
        // c4rs produces DSL consumed by Structurizr
        .add_relationship("c", "s1", "Produces DSL for", Some("Structurizr DSL"))
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
        // Component view of c4rs-core (use container identifier c.c2)
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
                .title("c4rs-structurizr-dsl".into())
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
