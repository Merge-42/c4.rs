//! Integration tests for Structurizr DSL serialization.

use c4rs::DslSerializer;
use c4rs::c4::{Container, ContainerType, Person, SoftwareSystem};

#[test]
fn test_integration_single_person() {
    let person = Person::builder()
        .name("User".into())
        .description("A system user".into())
        .build()
        .unwrap();

    let dsl = DslSerializer::new()
        .add_person(&person)
        .serialize()
        .unwrap();

    assert!(dsl.contains("workspace {"));
    assert!(dsl.contains("model {"));
    assert!(dsl.contains(r#"u = person "User""#));
}

#[test]
fn test_integration_full_model() {
    let person = Person::builder()
        .name("User".into())
        .description("A system user".into())
        .build()
        .unwrap();

    let system = SoftwareSystem::builder()
        .name("API".into())
        .description("Backend API service".into())
        .build()
        .unwrap();

    let container = Container::builder()
        .name("Web App".into())
        .description("Frontend application".into())
        .container_type(ContainerType::WebApplication)
        .build()
        .unwrap();

    let dsl = DslSerializer::new()
        .add_person(&person)
        .add_software_system(&system)
        .serialize()
        .unwrap();

    assert!(dsl.contains(r#"u = person "User""#));
    assert!(dsl.contains(r#"a = softwareSystem "API""#));
}
