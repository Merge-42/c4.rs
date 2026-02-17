//! Main Structurizr DSL Serializer.

use crate::c4::{Person, SoftwareSystem};
use crate::serialization::error::StructurizrDslError;
use crate::serialization::styles_serializer::{ElementStyle, RelationshipStyle, StylesSerializer};
use crate::serialization::views_serializer::{ViewConfiguration, ViewsSerializer};
use crate::serialization::workspace_serializer::WorkspaceSerializer;

/// Serializer for converting C4 models to Structurizr DSL format.
#[derive(Debug, Default)]
pub struct StructurizrDslSerializer {
    workspace_serializer: WorkspaceSerializer,
    views_serializer: ViewsSerializer,
    styles_serializer: StylesSerializer,
    name: Option<String>,
    description: Option<String>,
}

impl StructurizrDslSerializer {
    /// Create a new Structurizr DSL serializer.
    pub fn new() -> Self {
        Self {
            workspace_serializer: WorkspaceSerializer::new(),
            views_serializer: ViewsSerializer::new(),
            styles_serializer: StylesSerializer::new(),
            name: None,
            description: None,
        }
    }

    /// Set the workspace name.
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Set the workspace description.
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    /// Add a person to the workspace.
    pub fn add_person(&mut self, person: Person) {
        self.workspace_serializer.add_person(person);
    }

    /// Add a software system to the workspace.
    pub fn add_software_system(&mut self, system: SoftwareSystem) {
        self.workspace_serializer.add_software_system(system);
    }

    /// Add a view configuration.
    pub fn add_view(&mut self, view: &ViewConfiguration) {
        self.views_serializer.add_view(view.clone());
        self.workspace_serializer.add_view(view);
    }

    /// Add an element style.
    pub fn add_element_style(&mut self, style: ElementStyle) {
        self.styles_serializer.add_element_style(style);
    }

    /// Add a relationship style.
    pub fn add_relationship_style(&mut self, style: RelationshipStyle) {
        self.styles_serializer.add_relationship_style(style);
    }

    pub fn add_relationship(
        &mut self,
        source_id: &str,
        target_id: &str,
        description: &str,
        technology: Option<&str>,
    ) {
        self.workspace_serializer
            .add_relationship(source_id, target_id, description, technology);
    }

    /// Serialize the workspace to Structurizr DSL.
    ///
    /// # Returns
    ///
    /// A string containing the Structurizr DSL representation.
    ///
    /// # Errors
    ///
    /// Returns a `StructurizrDslError` if serialization fails.
    pub fn serialize(&mut self) -> Result<String, StructurizrDslError> {
        if let Some(ref name) = self.name {
            self.workspace_serializer.set_name(name);
        }
        if let Some(ref desc) = self.description {
            self.workspace_serializer.set_description(desc);
        }

        let styles_dsl = self.styles_serializer.serialize();
        if !styles_dsl.is_empty() {
            self.workspace_serializer.add_element_styles(&styles_dsl);
        }

        self.workspace_serializer.serialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::c4::{Container, ContainerType, Person, SoftwareSystem};
    use crate::serialization::views_serializer::ViewType;

    #[test]
    fn test_serialize_empty_model() {
        let mut serializer = StructurizrDslSerializer::new();
        let result = serializer.serialize().unwrap();
        assert!(result.starts_with("workspace "));
        assert!(result.contains("model {"));
    }

    #[test]
    fn test_serialize_single_person() {
        let person: Person = Person::builder()
            .name("User".try_into().unwrap())
            .description("A system user".try_into().unwrap())
            .build();

        let mut serializer = StructurizrDslSerializer::new();
        serializer.add_person(person);
        let result = serializer.serialize().unwrap();

        assert!(result.contains(r#"u = person "User" "A system user""#));
    }

    #[test]
    fn test_serialize_full_model() {
        let person: Person = Person::builder()
            .name("User".try_into().unwrap())
            .description("A system user".try_into().unwrap())
            .build();

        let system: SoftwareSystem = SoftwareSystem::builder()
            .name("API".try_into().unwrap())
            .description("Backend API".try_into().unwrap())
            .containers(vec![
                Container::builder()
                    .name("Web App".try_into().unwrap())
                    .description("Frontend".try_into().unwrap())
                    .container_type(ContainerType::WebApplication)
                    .build(),
            ])
            .build();

        let mut serializer = StructurizrDslSerializer::new();
        serializer.add_person(person);
        serializer.add_software_system(system);
        let result = serializer.serialize().unwrap();

        assert!(result.contains(r#"u = person "User""#));
        assert!(result.contains(r#"a = softwareSystem "API""#));
        assert!(result.contains(r#"wa = container "Web App""#));
    }

    #[test]
    fn test_serialize_with_views() {
        let person: Person = Person::builder()
            .name("User".try_into().unwrap())
            .description("A system user".try_into().unwrap())
            .build();

        let mut serializer = StructurizrDslSerializer::new();
        serializer.add_person(person);
        let mut view = ViewConfiguration::new(ViewType::SystemContext, "u", "System Context");
        view.include_element("*");
        serializer.add_view(&view);

        let result = serializer.serialize().unwrap();

        assert!(result.contains("views {"));
        assert!(result.contains("systemContext u \"System Context\" {"));
        assert!(result.contains("include *"));
    }

    #[test]
    fn test_serialize_with_styles() {
        let person: Person = Person::builder()
            .name("User".try_into().unwrap())
            .description("A system user".try_into().unwrap())
            .build();

        let mut serializer = StructurizrDslSerializer::new();
        serializer.add_person(person);
        serializer.add_element_style(
            ElementStyle::new("Person")
                .with_background("#ffcc00")
                .with_color("#000000"),
        );

        let result = serializer.serialize().unwrap();

        assert!(result.contains("styles {"));
        assert!(result.contains(r#"element "Person""#));
        assert!(result.contains("background #ffcc00"));
    }

    #[test]
    fn test_complete_workspace_serialization() {
        let person: Person = Person::builder()
            .name("User".try_into().unwrap())
            .description("A user of the system".try_into().unwrap())
            .build();

        let system: SoftwareSystem = SoftwareSystem::builder()
            .name("API".try_into().unwrap())
            .description("Backend API service".try_into().unwrap())
            .containers(vec![
                Container::builder()
                    .name("Web App".try_into().unwrap())
                    .description("Frontend".try_into().unwrap())
                    .container_type(ContainerType::WebApplication)
                    .build(),
            ])
            .build();

        let mut serializer = StructurizrDslSerializer::new()
            .with_name("Example System")
            .with_description("An example C4 model");
        serializer.add_person(person);
        serializer.add_software_system(system);

        let mut view = ViewConfiguration::new(ViewType::SystemContext, "a", "SystemContext");
        view.include_element("*");
        serializer.add_view(&view);

        serializer.add_element_style(ElementStyle::new("Person").with_shape("person"));

        let result = serializer.serialize().unwrap();

        assert!(result.starts_with("workspace "));
        assert!(result.contains("!identifiers hierarchical"));
        assert!(result.contains("model {"));
        assert!(result.contains("views {"));
        assert!(result.contains("systemContext a \"SystemContext\" {"));
        assert!(result.contains("include *"));
        assert!(result.contains("styles {"));
        assert!(result.contains(r#"element "Person""#));
        assert!(result.contains("shape person"));
        assert!(result.ends_with("}\n}"));
    }

    #[test]
    fn test_playground_format_structure() {
        let person: Person = Person::builder()
            .name("User".try_into().unwrap())
            .description("A user".try_into().unwrap())
            .build();

        let system: SoftwareSystem = SoftwareSystem::builder()
            .name("BankApp".try_into().unwrap())
            .description("Banking App".try_into().unwrap())
            .containers(vec![
                Container::builder()
                    .name("Web App".try_into().unwrap())
                    .description("Frontend".try_into().unwrap())
                    .container_type(ContainerType::WebApplication)
                    .build(),
            ])
            .build();

        let mut serializer = StructurizrDslSerializer::new()
            .with_name("Test Workspace")
            .with_description("Test");
        serializer.add_person(person);
        serializer.add_software_system(system);
        serializer.add_relationship("u", "b", "Uses", None);

        let mut view = ViewConfiguration::new(ViewType::SystemContext, "b", "SystemContext");
        view.include_element("*");
        serializer.add_view(&view);

        let result = serializer.serialize().unwrap();

        let opens = result.matches('{').count();
        let closes = result.matches('}').count();
        assert_eq!(opens, closes, "Braces should be balanced");
        assert!(result.contains("u = person \"User\""));
        assert!(result.contains("b = softwareSystem \"BankApp\""));
        assert!(result.contains("u -> b \"Uses\""));
        assert!(result.contains("systemContext b \"SystemContext\" {"));
    }

    #[test]
    fn test_nested_container_serialization() {
        let system: SoftwareSystem = SoftwareSystem::builder()
            .name("API".try_into().unwrap())
            .description("Backend".try_into().unwrap())
            .containers(vec![
                Container::builder()
                    .name("Web App".try_into().unwrap())
                    .description("Frontend".try_into().unwrap())
                    .container_type(ContainerType::WebApplication)
                    .build(),
                Container::builder()
                    .name("Database".try_into().unwrap())
                    .description("Data store".try_into().unwrap())
                    .container_type(ContainerType::Database)
                    .technology(Some("PostgreSQL".into()))
                    .build(),
            ])
            .build();

        let mut serializer = StructurizrDslSerializer::new()
            .with_name("Nested Test")
            .with_description("Test");
        serializer.add_software_system(system);

        let result = serializer.serialize().unwrap();

        assert!(result.contains("a = softwareSystem \"API\""));
        assert!(result.contains("wa = container \"Web App\" \"Frontend\""));
        assert!(result.contains("d = container \"Database\" \"Data store\""));
        assert!(result.contains("wa = container \"Web App\""));
        assert!(result.contains("d = container \"Database\""));
    }

    #[test]
    fn test_circular_relationships() {
        let person: Person = Person::builder()
            .name("User".try_into().unwrap())
            .description("A user".try_into().unwrap())
            .build();

        let system1: SoftwareSystem = SoftwareSystem::builder()
            .name("SystemA".try_into().unwrap())
            .description("System A".try_into().unwrap())
            .build();

        let system2: SoftwareSystem = SoftwareSystem::builder()
            .name("SystemB".try_into().unwrap())
            .description("System B".try_into().unwrap())
            .build();

        let mut serializer = StructurizrDslSerializer::new()
            .with_name("Circular Test")
            .with_description("Test");
        serializer.add_person(person);
        serializer.add_software_system(system1);
        serializer.add_software_system(system2);
        serializer.add_relationship("person", "a", "Uses", None);
        serializer.add_relationship("a", "b", "Communicates with", Some("HTTP"));
        serializer.add_relationship("b", "person", "Sends data to", None);
        serializer.add_relationship("b", "a", "Receives from", Some("HTTP"));

        let result = serializer.serialize().unwrap();

        assert!(result.contains("person -> a \"Uses\""));
        assert!(result.contains("a -> b \"Communicates with\" \"HTTP\""));
        assert!(result.contains("b -> person \"Sends data to\""));
        assert!(result.contains("b -> a \"Receives from\" \"HTTP\""));
    }

    #[test]
    fn test_special_characters_in_names() {
        let person: Person = Person::builder()
            .name("User's System".try_into().unwrap())
            .description("A \"special\" user & <test>".try_into().unwrap())
            .build();

        let system: SoftwareSystem = SoftwareSystem::builder()
            .name("API-Service_v2".try_into().unwrap())
            .description("Backend API (version 2.0)".try_into().unwrap())
            .containers(vec![
                Container::builder()
                    .name("Web/App".try_into().unwrap())
                    .description("Frontend".try_into().unwrap())
                    .container_type(ContainerType::WebApplication)
                    .build(),
            ])
            .build();

        let mut serializer = StructurizrDslSerializer::new()
            .with_name("Special Chars Test")
            .with_description("Test with special characters");
        serializer.add_person(person);
        serializer.add_software_system(system);

        let result = serializer.serialize().unwrap();

        assert!(result.contains("us = person \"User's System\""));
        assert!(result.contains("a = softwareSystem \"API-Service_v2\""));
        assert!(result.contains("w = container \"Web/App\""));
        assert!(result.contains("special"));
    }

    #[test]
    fn test_relationship_with_technology() {
        let person: Person = Person::builder()
            .name("User".try_into().unwrap())
            .description("A user".try_into().unwrap())
            .build();

        let system: SoftwareSystem = SoftwareSystem::builder()
            .name("API".try_into().unwrap())
            .description("Backend API".try_into().unwrap())
            .build();

        let mut serializer = StructurizrDslSerializer::new()
            .with_name("Tech Test")
            .with_description("Test");
        serializer.add_person(person);
        serializer.add_software_system(system);
        serializer.add_relationship("u", "a", "Uses", Some("HTTPS"));
        serializer.add_relationship("a", "u", "Responds to", Some("JSON/HTTPS"));

        let result = serializer.serialize().unwrap();

        assert!(result.contains("u -> a \"Uses\" \"HTTPS\""));
        assert!(result.contains("a -> u \"Responds to\" \"JSON/HTTPS\""));
    }

    #[test]
    fn test_multiple_identical_element_names() {
        let person1: Person = Person::builder()
            .name("User".try_into().unwrap())
            .description("First user".try_into().unwrap())
            .build();

        let person2: Person = Person::builder()
            .name("User".try_into().unwrap())
            .description("Second user".try_into().unwrap())
            .build();

        let person3: Person = Person::builder()
            .name("User".try_into().unwrap())
            .description("Third user".try_into().unwrap())
            .build();

        let mut serializer = StructurizrDslSerializer::new()
            .with_name("Duplicate Names Test")
            .with_description("Test");
        serializer.add_person(person1);
        serializer.add_person(person2);
        serializer.add_person(person3);

        let result = serializer.serialize().unwrap();

        assert!(result.contains("u = person \"User\" \"First user\""));
        assert!(result.contains("u1 = person \"User\" \"Second user\""));
        assert!(result.contains("u2 = person \"User\" \"Third user\""));
    }
}
