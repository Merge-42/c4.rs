//! Golden-file integration tests that pin the exact DSL output.
//! These ensure refactoring does not change serialization behavior.

use c4rs::c4::ContainerType;
use c4rs::{
    Component, Container, DslSerializer, ElementStyle, Person, RelationshipStyle, SoftwareSystem,
    ViewConfiguration, ViewType,
};

#[test]
fn test_golden_self_diagram_output() {
    let library_consumer = Person::builder()
        .name("Library Consumer".into())
        .description("A Rust developer using c4rs to model their architecture".into())
        .build()
        .unwrap();

    let element_trait = Component::builder()
        .name("Element Trait".into())
        .description("Common trait implemented by all C4 element types".into())
        .technology("Rust Trait".into())
        .build()
        .unwrap();

    let context_types = Component::builder()
        .name("Context Types".into())
        .description("Person and SoftwareSystem types for the context level".into())
        .technology("Rust".into())
        .build()
        .unwrap();

    let container_type = Component::builder()
        .name("Container Type".into())
        .description("Container type with nested component ownership".into())
        .technology("Rust".into())
        .build()
        .unwrap();

    let component_type = Component::builder()
        .name("Component Type".into())
        .description("Component type with nested code element ownership".into())
        .technology("Rust".into())
        .build()
        .unwrap();

    let code_element_type = Component::builder()
        .name("Code Element Type".into())
        .description("CodeElement type for classes, functions, structs, etc.".into())
        .technology("Rust".into())
        .build()
        .unwrap();

    let relationship_type = Component::builder()
        .name("Relationship".into())
        .description("Generic Relationship<S, T> between any two Element types".into())
        .technology("Rust Generics".into())
        .build()
        .unwrap();

    let validation = Component::builder()
        .name("Validation".into())
        .description("Input validation for names, descriptions, and field lengths".into())
        .technology("Rust".into())
        .build()
        .unwrap();

    let builders = Component::builder()
        .name("Builders".into())
        .description("Fallible builder pattern via bon with validation on build()".into())
        .technology("bon".into())
        .build()
        .unwrap();

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
        .build()
        .unwrap();

    let dsl_serializer = Component::builder()
        .name("DslSerializer".into())
        .description("Consuming-builder facade for assembling a complete workspace".into())
        .technology("Rust".into())
        .build()
        .unwrap();

    let workspace_serializer = Component::builder()
        .name("WorkspaceSerializer".into())
        .description("Core serialization engine that walks the model and emits DSL".into())
        .technology("Rust".into())
        .build()
        .unwrap();

    let identifier_generator = Component::builder()
        .name("IdentifierGenerator".into())
        .description("Auto-generates unique DSL identifiers from element names".into())
        .technology("Rust".into())
        .build()
        .unwrap();

    let dsl_writer = Component::builder()
        .name("DslWriter".into())
        .description("Indentation-aware string builder for DSL output".into())
        .technology("Rust".into())
        .build()
        .unwrap();

    let views_serializer = Component::builder()
        .name("ViewsSerializer".into())
        .description("Renders view blocks (systemContext, container, component, etc.)".into())
        .technology("Askama".into())
        .build()
        .unwrap();

    let styles_serializer = Component::builder()
        .name("StylesSerializer".into())
        .description("Renders element and relationship style blocks".into())
        .technology("Askama".into())
        .build()
        .unwrap();

    let askama_templates = Component::builder()
        .name("Askama Templates".into())
        .description("Inline Askama templates for DSL fragment rendering".into())
        .technology("Askama".into())
        .build()
        .unwrap();

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
        .build()
        .unwrap();

    let umbrella_container = Container::builder()
        .name("c4rs".into())
        .description("Umbrella crate that re-exports core types and DSL serializer".into())
        .container_type(ContainerType::Other("Library".into()))
        .technology("Rust".into())
        .build()
        .unwrap();

    let c4rs_system = SoftwareSystem::builder()
        .name("c4rs".into())
        .description(
            "Rust library for defining C4 architecture models with pluggable serialization".into(),
        )
        .add_container(umbrella_container)
        .add_container(core_container)
        .add_container(dsl_container)
        .build()
        .unwrap();

    let structurizr = SoftwareSystem::builder()
        .name("Structurizr".into())
        .description("Renders architecture diagrams from Structurizr DSL files".into())
        .build()
        .unwrap();

    let dsl = DslSerializer::new()
        .with_name("c4rs Architecture")
        .with_description("C4 model of the c4rs Rust library itself")
        .add_person(library_consumer)
        .add_software_system(c4rs_system)
        .add_software_system(structurizr)
        .add_relationship("lc", "c", "Uses", Some("Cargo dependency"))
        .add_relationship("c.c1", "c.c2", "Re-exports types from", None)
        .add_relationship("c.c1", "c.c3", "Re-exports serializer from", None)
        .add_relationship("c.c3", "c.c2", "Depends on", Some("Cargo path dep"))
        .add_relationship("c", "s1", "Produces DSL for", Some("Structurizr DSL"))
        .add_view(
            ViewConfiguration::builder()
                .view_type(ViewType::SystemLandscape)
                .element_identifier("*".into())
                .title("System Landscape".into())
                .include_elements(vec!["*".into()])
                .build(),
        )
        .add_view(
            ViewConfiguration::builder()
                .view_type(ViewType::SystemContext)
                .element_identifier("c".into())
                .title("c4rs System Context".into())
                .include_elements(vec!["*".into()])
                .build(),
        )
        .add_view(
            ViewConfiguration::builder()
                .view_type(ViewType::Container)
                .element_identifier("c".into())
                .title("c4rs Containers".into())
                .include_elements(vec!["*".into()])
                .build(),
        )
        .add_view(
            ViewConfiguration::builder()
                .view_type(ViewType::Component)
                .element_identifier("c.c1".into())
                .title("c4rs-core Components".into())
                .include_elements(vec!["*".into()])
                .build(),
        )
        .add_view(
            ViewConfiguration::builder()
                .view_type(ViewType::Component)
                .element_identifier("c.c2".into())
                .title("c4rs-structurizr-dsl Components".into())
                .include_elements(vec!["*".into()])
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
        .serialize()
        .unwrap();

    let expected = r#"workspace "c4rs Architecture" "C4 model of the c4rs Rust library itself" {
    !identifiers hierarchical

    model {
        lc = person "Library Consumer" "A Rust developer using c4rs to model their architecture"
        c = softwareSystem "c4rs" "Rust library for defining C4 architecture models with pluggable serialization" {
            c1 = container "c4rs" "Umbrella crate that re-exports core types and DSL serializer" {}
            c2 = container "c4rs-core" "Core C4 model types, traits, validation, and builders" {
                et = component "Element Trait" "Common trait implemented by all C4 element types" "Rust Trait"
                ct = component "Context Types" "Person and SoftwareSystem types for the context level" "Rust"
                ct1 = component "Container Type" "Container type with nested component ownership" "Rust"
                ct2 = component "Component Type" "Component type with nested code element ownership" "Rust"
                cet = component "Code Element Type" "CodeElement type for classes, functions, structs, etc." "Rust"
                r = component "Relationship" "Generic Relationship<S, T> between any two Element types" "Rust Generics"
                v = component "Validation" "Input validation for names, descriptions, and field lengths" "Rust"
                b = component "Builders" "Fallible builder pattern via bon with validation on build()" "bon"
            }
            c3 = container "c4rs-structurizr-dsl" "Structurizr DSL serialization module" {
                d = component "DslSerializer" "Consuming-builder facade for assembling a complete workspace" "Rust"
                w = component "WorkspaceSerializer" "Core serialization engine that walks the model and emits DSL" "Rust"
                i = component "IdentifierGenerator" "Auto-generates unique DSL identifiers from element names" "Rust"
                d1 = component "DslWriter" "Indentation-aware string builder for DSL output" "Rust"
                v1 = component "ViewsSerializer" "Renders view blocks (systemContext, container, component, etc.)" "Askama"
                s = component "StylesSerializer" "Renders element and relationship style blocks" "Askama"
                at = component "Askama Templates" "Inline Askama templates for DSL fragment rendering" "Askama"
            }
        }
        s1 = softwareSystem "Structurizr" "Renders architecture diagrams from Structurizr DSL files"
        lc -> c "Uses" "Cargo dependency"
        c.c1 -> c.c2 "Re-exports types from"
        c.c1 -> c.c3 "Re-exports serializer from"
        c.c3 -> c.c2 "Depends on" "Cargo path dep"
        c -> s1 "Produces DSL for" "Structurizr DSL"
    }

    views {
        systemLandscape "System_Landscape" {
            include *
        }
        systemContext c "c4rs_System_Context" {
            include *
        }
        container c "c4rs_Containers" {
            include *
        }
        component c.c1 "c4rs-core_Components" {
            include *
        }
        component c.c2 "c4rs-structurizr-dsl_Components" {
            include *
        }

        styles {
            element "Person" {
                background #08427B
                color #ffffff
                shape person
            }
            element "Software System" {
                background #1168BD
                color #ffffff
            }
            element "Container" {
                background #438DD5
                color #ffffff
            }
            element "Component" {
                background #85BBF0
                color #000000
            }
            relationship "Relationship" {
                color #707070
                dashed false
            }
        }
    }
}"#;

    assert_eq!(dsl, expected);
}

#[test]
fn test_golden_smoke_test_1_output() {
    let person = Person::builder()
        .name("User".into())
        .description("A user of the system".into())
        .build()
        .unwrap();

    let api_system = SoftwareSystem::builder()
        .name("API".into())
        .description("Backend API service".into())
        .add_container(
            Container::builder()
                .name("Web App".into())
                .description("Frontend application".into())
                .container_type(ContainerType::WebApplication)
                .build()
                .unwrap(),
        )
        .add_container(
            Container::builder()
                .name("Database".into())
                .description("PostgreSQL database".into())
                .container_type(ContainerType::Database)
                .technology("PostgreSQL 15".into())
                .build()
                .unwrap(),
        )
        .add_container(
            Container::builder()
                .name("API Service".into())
                .description("Backend API".into())
                .container_type(ContainerType::Api)
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    let web_system = SoftwareSystem::builder()
        .name("Web Portal".into())
        .description("Customer web portal".into())
        .add_container(
            Container::builder()
                .name("Frontend".into())
                .description("React frontend".into())
                .container_type(ContainerType::WebApplication)
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    let ctx_view = ViewConfiguration::builder()
        .view_type(ViewType::SystemContext)
        .element_identifier("a".to_string())
        .title("SystemContext".to_string())
        .include_elements(vec!["*".to_string()])
        .build();

    let dsl = DslSerializer::new()
        .with_name("Example System")
        .with_description("An example C4 model")
        .add_person(person)
        .add_software_system(api_system)
        .add_software_system(web_system)
        .add_relationship("u", "a", "Uses", None)
        .add_view(ctx_view)
        .add_element_style(
            ElementStyle::builder()
                .identifier("Person".into())
                .shape("person".into())
                .build(),
        )
        .add_element_style(
            ElementStyle::builder()
                .identifier("Database".into())
                .shape("cylinder".into())
                .build(),
        )
        .serialize()
        .unwrap();

    let expected = r#"workspace "Example System" "An example C4 model" {
    !identifiers hierarchical

    model {
        u = person "User" "A user of the system"
        a = softwareSystem "API" "Backend API service" {
            wa = container "Web App" "Frontend application" {}
            d = container "Database" "PostgreSQL database" {}
            as = container "API Service" "Backend API" {}
        }
        wp = softwareSystem "Web Portal" "Customer web portal" {
            f = container "Frontend" "React frontend" {}
        }
        u -> a "Uses"
    }

    views {
        systemContext a "SystemContext" {
            include *
        }

        styles {
            element "Person" {
                shape person
            }
            element "Database" {
                shape cylinder
            }
        }
    }
}"#;

    assert_eq!(dsl, expected);
}

#[test]
fn test_golden_system_landscape_no_identifier() {
    let person = Person::builder()
        .name("User".into())
        .description("A user".into())
        .build()
        .unwrap();

    let system = SoftwareSystem::builder()
        .name("System".into())
        .description("A system".into())
        .build()
        .unwrap();

    let dsl = DslSerializer::new()
        .with_name("Landscape Test")
        .with_description("Test")
        .add_person(person)
        .add_software_system(system)
        .add_view(
            ViewConfiguration::builder()
                .view_type(ViewType::SystemLandscape)
                .element_identifier("*".into())
                .title("Landscape".into())
                .include_elements(vec!["*".into()])
                .build(),
        )
        .serialize()
        .unwrap();

    // SystemLandscape must not emit an identifier between the keyword and title
    assert!(dsl.contains(r#"systemLandscape "Landscape" {"#));
    assert!(!dsl.contains("systemLandscape *"));
}

#[test]
fn test_golden_relationship_style_requires_tag() {
    let person = Person::builder()
        .name("User".into())
        .description("A user".into())
        .build()
        .unwrap();

    let dsl = DslSerializer::new()
        .add_person(person)
        .add_relationship_style(
            RelationshipStyle::builder()
                .identifier("Relationship".into())
                .color("#ff0000".into())
                .build(),
        )
        .serialize()
        .unwrap();

    // relationship must have a tag identifier
    assert!(dsl.contains(r#"relationship "Relationship" {"#));
    assert!(!dsl.contains("relationship {\n"));
}

#[test]
fn test_golden_hierarchical_relationship_identifiers_preserved() {
    let system = SoftwareSystem::builder()
        .name("System".into())
        .description("A system".into())
        .add_container(
            Container::builder()
                .name("API".into())
                .description("API".into())
                .container_type(ContainerType::Api)
                .build()
                .unwrap(),
        )
        .add_container(
            Container::builder()
                .name("DB".into())
                .description("Database".into())
                .container_type(ContainerType::Database)
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    let dsl = DslSerializer::new()
        .add_software_system(system)
        .add_relationship("s.a", "s.d", "Reads from", Some("SQL"))
        .serialize()
        .unwrap();

    // Dot-separated hierarchical identifiers must not be mangled
    assert!(dsl.contains(r#"s.a -> s.d "Reads from" "SQL""#));
}
