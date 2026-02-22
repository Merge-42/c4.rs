//! Generates a Structurizr DSL diagram of the c4rs codebase itself.
//!
//! Run with: cargo run --example self_diagram

use c4rs::c4::ContainerType;
use c4rs::{
    Component, Container, DslSerializer, ElementStyle, Person, RelationshipStyle, SoftwareSystem,
    ViewConfiguration, ViewType,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // --- People ---

    let library_consumer = Person::builder()
        .name("Library Consumer".into())
        .description("A Rust developer using c4rs to model their architecture".into())
        .build()?;

    // --- c4rs-core components ---

    let element_trait = Component::builder()
        .name("Element Trait".into())
        .description("Common trait implemented by all C4 element types".into())
        .technology("Rust Trait".into())
        .build()?;

    let context_types = Component::builder()
        .name("Context Types".into())
        .description("Person and SoftwareSystem types for the context level".into())
        .technology("Rust".into())
        .build()?;

    let container_type = Component::builder()
        .name("Container Type".into())
        .description("Container type with nested component ownership".into())
        .technology("Rust".into())
        .build()?;

    let component_type = Component::builder()
        .name("Component Type".into())
        .description("Component type with nested code element ownership".into())
        .technology("Rust".into())
        .build()?;

    let code_element_type = Component::builder()
        .name("Code Element Type".into())
        .description("CodeElement type for classes, functions, structs, etc.".into())
        .technology("Rust".into())
        .build()?;

    let relationship_type = Component::builder()
        .name("Relationship".into())
        .description("Generic Relationship<S, T> between any two Element types".into())
        .technology("Rust Generics".into())
        .build()?;

    let validation = Component::builder()
        .name("Validation".into())
        .description("Input validation for names, descriptions, and field lengths".into())
        .technology("Rust".into())
        .build()?;

    let builders = Component::builder()
        .name("Builders".into())
        .description("Fallible builder pattern via bon with validation on build()".into())
        .technology("bon".into())
        .build()?;

    let core_container = Container::builder()
        .name("c4rs-core".into())
        .description("Core C4 model types, traits, validation, and builders".into())
        .container_type(ContainerType::Other("Library".into()))
        .technology("Rust".into())
        .add_component(element_trait)
        .add_component(context_types)
        .add_component(container_type)
        .add_component(component_type)
        .add_component(code_element_type)
        .add_component(relationship_type)
        .add_component(validation)
        .add_component(builders)
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
        .technology("Askama".into())
        .build()?;

    let styles_serializer = Component::builder()
        .name("StylesSerializer".into())
        .description("Renders element and relationship style blocks".into())
        .technology("Askama".into())
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
    //   "c4rs" (umbrella ctr)   -> "c1"  (collision with system)
    //   "c4rs-core" (ctr)       -> "c2"  (collision)
    //   "c4rs-structurizr-dsl"  -> "c3"  (collision)
    //   "Structurizr"           -> "s1"  (collision with StylesSerializer component "s")

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
        // Component view of c4rs-core
        .add_view(
            ViewConfiguration::builder()
                .view_type(ViewType::Component)
                .element_identifier("c.c1".into())
                .title("c4rs-core Components".into())
                .include_elements(vec!["*".into()])
                .build(),
        )
        // Component view of c4rs-structurizr-dsl
        .add_view(
            ViewConfiguration::builder()
                .view_type(ViewType::Component)
                .element_identifier("c.c2".into())
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
