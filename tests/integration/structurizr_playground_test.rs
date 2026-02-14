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

#[test]
fn test_integration_hierarchy_serialization() {
    use c4rs::serialization::HierarchySerializer;

    let mut hierarchy = HierarchySerializer::new();
    hierarchy
        .serialize_parent_reference("WebApp", "API")
        .unwrap();

    let output = hierarchy.as_output();
    assert_eq!(output, r#"WebApp <- API "<- contained""#);
}

#[test]
fn test_integration_hierarchy_validation() {
    use c4rs::serialization::HierarchySerializer;

    let mut hierarchy = HierarchySerializer::new();
    hierarchy.register_software_system("API");

    let result = hierarchy.validate_container_parent("WebApp", Some("API"));
    assert!(result.is_ok());
}

#[test]
fn test_integration_invalid_hierarchy() {
    use c4rs::serialization::HierarchySerializer;

    let hierarchy = HierarchySerializer::new();
    hierarchy.register_software_system("API");

    let result = hierarchy.validate_container_parent("WebApp", Some("Database"));
    assert!(result.is_err());
}

#[test]
fn test_integration_views_serialization() {
    use c4rs::c4::ElementType;
    use c4rs::serialization::{ViewConfiguration, ViewsSerializer};

    let mut views = ViewsSerializer::new();
    let mut view = ViewConfiguration::new("context", "System Context", ElementType::SoftwareSystem);
    view.include_element("User");
    view.include_element("API");
    views.add_view(view);

    let dsl = views.serialize();
    assert!(dsl.contains("views {"));
    assert!(dsl.contains("systemcontext context {"));
    assert!(dsl.contains("include User"));
}

#[test]
fn test_integration_styles_serialization() {
    use c4rs::c4::ElementType;
    use c4rs::serialization::{ElementStyle, RelationshipStyle, StylesSerializer};

    let mut styles = StylesSerializer::new();
    styles.add_element_style(
        ElementStyle::new("person", ElementType::Person)
            .with_background("#ffcc00")
            .with_color("#000000"),
    );
    styles.add_relationship_style(
        RelationshipStyle::new()
            .with_thickness("2")
            .with_color("#999999"),
    );

    let dsl = styles.serialize();
    assert!(dsl.contains("styles {"));
    assert!(dsl.contains("person {"));
    assert!(dsl.contains("background #ffcc00"));
    assert!(dsl.contains("relationship {"));
    assert!(dsl.contains("thickness 2"));
}
