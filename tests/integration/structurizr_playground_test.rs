//! Integration tests for Structurizr DSL serialization.

use c4rs::c4::{Container, ContainerType, Person, SoftwareSystem};
use c4rs::serialization::StructurizrDslSerializer;

#[test]
fn test_integration_single_person() {
    let person: Person = Person::builder()
        .with_name("User".try_into().unwrap())
        .with_description("A system user".try_into().unwrap())
        .build()
        .unwrap();

    let mut serializer = StructurizrDslSerializer::new();
    let dsl = serializer.serialize(&[&person]).unwrap();

    // Verify basic structure
    assert!(dsl.contains("workspace {"));
    assert!(dsl.contains("model {"));
    assert!(dsl.contains(r#"user = person "User" "A system user""#));
}

#[test]
fn test_integration_full_model() {
    let person: Person = Person::builder()
        .with_name("User".try_into().unwrap())
        .with_description("A system user".try_into().unwrap())
        .build()
        .unwrap();

    let system: SoftwareSystem = SoftwareSystem::builder()
        .with_name("API".try_into().unwrap())
        .with_description("Backend API service".try_into().unwrap())
        .build()
        .unwrap();

    let container: Container = Container::builder()
        .with_name("Web App".try_into().unwrap())
        .with_description("Frontend application".try_into().unwrap())
        .with_container_type(ContainerType::WebApplication)
        .build()
        .unwrap();

    let mut serializer = StructurizrDslSerializer::new();
    let dsl = serializer
        .serialize(&[&person, &system, &container])
        .unwrap();

    // Verify all elements present
    assert!(dsl.contains(r#"user = person "User""#));
    assert!(dsl.contains(r#"api = softwareSystem "API""#));
    assert!(dsl.contains(r#"web_app = container "Web App""#));
}

#[test]
fn test_integration_special_characters() {
    let person: Person = Person::builder()
        .with_name("User\"Name".try_into().unwrap())
        .with_description("A \"test\" user".try_into().unwrap())
        .build()
        .unwrap();

    let mut serializer = StructurizrDslSerializer::new();
    let dsl = serializer.serialize(&[&person]).unwrap();

    // Verify escaping
    assert!(dsl.contains(r#"\"#));
}

#[test]
fn test_integration_empty_model() {
    let mut serializer = StructurizrDslSerializer::new();
    let dsl = serializer.serialize(&[]).unwrap();

    // Verify structure with no elements
    assert!(dsl.contains("workspace {"));
    assert!(dsl.contains("model {"));
    assert!(dsl.contains("!identifiers"));
}

#[test]
fn test_integration_performance() {
    use std::time::Instant;

    // Create a model with many elements
    let person: Person = Person::builder()
        .with_name("User".try_into().unwrap())
        .with_description("A system user".try_into().unwrap())
        .build()
        .unwrap();

    let mut systems: Vec<SoftwareSystem> = Vec::new();
    for i in 0..50 {
        let system = SoftwareSystem::builder()
            .with_name(format!("System {}", i).try_into().unwrap())
            .with_description(format!("System {} description", i).try_into().unwrap())
            .build()
            .unwrap();
        systems.push(system);
    }

    let mut serializer = StructurizrDslSerializer::new();

    let start = Instant::now();
    let dsl = serializer.serialize(&[&person]).unwrap();
    let elapsed = start.elapsed();

    // Verify performance (should complete in under 10 seconds for 100+ elements)
    assert!(
        elapsed.as_secs() < 10,
        "Serialization took too long: {:?}",
        elapsed
    );
    assert!(!dsl.is_empty());
}
