use crate::error::StructurizrDslError;
use crate::styles_serializer::{ElementStyle, RelationshipStyle, StylesSerializer};
use crate::views_serializer::ViewConfiguration;
use crate::workspace_serializer::WorkspaceSerializer;
use c4rs_core::c4::{Person, SoftwareSystem};

#[derive(Debug, Default)]
pub struct StructurizrDslSerializer {
    workspace_serializer: WorkspaceSerializer,
    styles_serializer: StylesSerializer,
    name: Option<String>,
    description: Option<String>,
}

impl StructurizrDslSerializer {
    pub fn new() -> Self {
        Self {
            workspace_serializer: WorkspaceSerializer::new(),
            styles_serializer: StylesSerializer::new(),
            name: None,
            description: None,
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn add_person(mut self, person: Person) -> Self {
        self.workspace_serializer.add_person(person);
        self
    }

    pub fn add_software_system(mut self, system: SoftwareSystem) -> Self {
        self.workspace_serializer.add_software_system(system);
        self
    }

    pub fn add_view(mut self, view: ViewConfiguration) -> Self {
        self.workspace_serializer.add_view(&view);
        self
    }

    pub fn add_element_style(mut self, style: ElementStyle) -> Self {
        self.styles_serializer.add_element_style(style);
        self
    }

    pub fn add_relationship_style(mut self, style: RelationshipStyle) -> Self {
        self.styles_serializer.add_relationship_style(style);
        self
    }

    pub fn add_relationship(
        mut self,
        source_id: &str,
        target_id: &str,
        description: &str,
        technology: Option<&str>,
    ) -> Self {
        self.workspace_serializer
            .add_relationship(source_id, target_id, description, technology);
        self
    }

    pub fn serialize(self) -> Result<String, StructurizrDslError> {
        let mut workspace_serializer = self.workspace_serializer;
        if let Some(name) = self.name {
            workspace_serializer.set_name(&name);
        }
        if let Some(desc) = self.description {
            workspace_serializer.set_description(&desc);
        }

        let styles_dsl = self.styles_serializer.serialize()?;
        if !styles_dsl.is_empty() {
            workspace_serializer.add_element_styles(&styles_dsl);
        }

        workspace_serializer.serialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::views_serializer::ViewType;
    use c4rs_core::c4::{Container, ContainerType, Person, SoftwareSystem};

    #[test]
    fn test_serialize_empty_model() {
        let serializer = StructurizrDslSerializer::new();
        let result = serializer.serialize().unwrap();
        assert!(result.starts_with("workspace "));
        assert!(result.contains("model {"));
    }

    #[test]
    fn test_serialize_single_person() {
        let person: Person = Person::builder()
            .name("User".into())
            .description("A system user".into())
            .build()
            .unwrap();

        let serializer = StructurizrDslSerializer::new();
        let result = serializer.add_person(person).serialize().unwrap();

        assert!(result.contains(r#"u = person "User" "A system user""#));
    }

    #[test]
    fn test_serialize_full_model() {
        let person: Person = Person::builder()
            .name("User".into())
            .description("A system user".into())
            .build()
            .unwrap();

        let system: SoftwareSystem = SoftwareSystem::builder()
            .name("API".into())
            .description("Backend".into())
            .add_container(
                Container::builder()
                    .name("Web App".into())
                    .description("Frontend".into())
                    .container_type(ContainerType::WebApplication)
                    .build()
                    .unwrap(),
            )
            .add_container(
                Container::builder()
                    .name("Database".into())
                    .description("Data store".into())
                    .container_type(ContainerType::Database)
                    .technology("PostgreSQL".into())
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();

        let serializer = StructurizrDslSerializer::new();
        let result = serializer
            .add_person(person)
            .add_software_system(system)
            .serialize()
            .unwrap();

        assert!(result.contains(r#"u = person "User""#));
        assert!(result.contains(r#"a = softwareSystem "API""#));
        assert!(result.contains(r#"wa = container "Web App""#));
    }

    #[test]
    fn test_serialize_with_views() {
        let person: Person = Person::builder()
            .name("User".into())
            .description("A system user".into())
            .build()
            .unwrap();

        let serializer = StructurizrDslSerializer::new();
        let view = ViewConfiguration::builder()
            .view_type(ViewType::SystemContext)
            .element_identifier("u".to_string())
            .title("System Context".to_string())
            .include_elements(vec!["*".to_string()])
            .build();
        let result = serializer
            .add_person(person)
            .add_view(view)
            .serialize()
            .unwrap();

        assert!(result.contains("views {"));
        assert!(result.contains("systemContext u \"System Context\" {"));
        assert!(result.contains("include *"));
    }

    #[test]
    fn test_serialize_with_styles() {
        let person: Person = Person::builder()
            .name("User".into())
            .description("A system user".into())
            .build()
            .unwrap();

        let serializer = StructurizrDslSerializer::new();
        let result = serializer
            .add_person(person)
            .add_element_style(
                ElementStyle::new("Person")
                    .with_background("#ffcc00")
                    .with_color("#000000"),
            )
            .serialize()
            .unwrap();

        assert!(result.contains("styles {"));
        assert!(result.contains(r#"element "Person""#));
        assert!(result.contains("background #ffcc00"));
    }

    #[test]
    fn test_complete_workspace_serialization() {
        let person: Person = Person::builder()
            .name("User".into())
            .description("A user of the system".into())
            .build()
            .unwrap();

        let system: SoftwareSystem = SoftwareSystem::builder()
            .name("API".into())
            .description("Backend API service".into())
            .add_container(
                Container::builder()
                    .name("Web App".into())
                    .description("Frontend".into())
                    .container_type(ContainerType::WebApplication)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();

        let view = ViewConfiguration::builder()
            .view_type(ViewType::SystemContext)
            .element_identifier("a".to_string())
            .title("SystemContext".to_string())
            .include_elements(vec!["*".to_string()])
            .build();

        let result = StructurizrDslSerializer::new()
            .with_name("Example System")
            .with_description("An example C4 model")
            .add_person(person)
            .add_software_system(system)
            .add_view(view)
            .add_element_style(ElementStyle::new("Person").with_shape("person"))
            .serialize()
            .unwrap();

        let trimmed = result.trim();

        assert!(result.starts_with("workspace "));
        assert!(result.contains("!identifiers hierarchical"));
        assert!(result.contains("model {"));
        assert!(result.contains("views {"));
        assert!(result.contains("systemContext a \"SystemContext\" {"));
        assert!(result.contains("include *"));
        assert!(result.contains("styles {"));
        assert!(result.contains(r#"element "Person""#));
        assert!(result.contains("shape person"));
        assert!(trimmed.ends_with("}"));
    }

    #[test]
    fn test_playground_format_structure() {
        let person: Person = Person::builder()
            .name("User".into())
            .description("A user".into())
            .build()
            .unwrap();

        let system: SoftwareSystem = SoftwareSystem::builder()
            .name("BankApp".into())
            .description("Banking App".into())
            .add_container(
                Container::builder()
                    .name("Web App".into())
                    .description("Frontend".into())
                    .container_type(ContainerType::WebApplication)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();

        let view = ViewConfiguration::builder()
            .view_type(ViewType::SystemContext)
            .element_identifier("b".to_string())
            .title("SystemContext".to_string())
            .include_elements(vec!["*".to_string()])
            .build();

        let result = StructurizrDslSerializer::new()
            .with_name("Test Workspace")
            .with_description("Test")
            .add_person(person)
            .add_software_system(system)
            .add_relationship("u", "b", "Uses", None)
            .add_view(view)
            .serialize()
            .unwrap();

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
            .name("BankApp".into())
            .description("Banking App".into())
            .add_container(
                Container::builder()
                    .name("Web App".into())
                    .description("Frontend".into())
                    .container_type(ContainerType::WebApplication)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();

        let result = StructurizrDslSerializer::new()
            .with_name("Nested Test")
            .with_description("Test")
            .add_software_system(system)
            .serialize()
            .unwrap();

        assert!(result.contains("b = softwareSystem \"BankApp\""));
        assert!(result.contains("wa = container \"Web App\" \"Frontend\""));
    }

    #[test]
    fn test_circular_relationships() {
        let person: Person = Person::builder()
            .name("User".into())
            .description("A user".into())
            .build()
            .unwrap();

        let system1: SoftwareSystem = SoftwareSystem::builder()
            .name("SystemA".into())
            .description("System A".into())
            .build()
            .unwrap();

        let system2: SoftwareSystem = SoftwareSystem::builder()
            .name("SystemB".into())
            .description("System B".into())
            .build()
            .unwrap();

        let result = StructurizrDslSerializer::new()
            .with_name("Circular Test")
            .with_description("Test")
            .add_person(person)
            .add_software_system(system1)
            .add_software_system(system2)
            .add_relationship("person", "a", "Uses", None)
            .add_relationship("a", "b", "Communicates with", Some("HTTP"))
            .add_relationship("b", "person", "Sends data to", None)
            .add_relationship("b", "a", "Receives from", Some("HTTP"))
            .serialize()
            .unwrap();

        assert!(result.contains("person -> a \"Uses\""));
        assert!(result.contains("a -> b \"Communicates with\" \"HTTP\""));
        assert!(result.contains("b -> person \"Sends data to\""));
        assert!(result.contains("b -> a \"Receives from\" \"HTTP\""));
    }

    #[test]
    fn test_special_characters_in_names() {
        let person: Person = Person::builder()
            .name("User's System".into())
            .description("A \"special\" user & <test>".into())
            .build()
            .unwrap();

        let system: SoftwareSystem = SoftwareSystem::builder()
            .name("API-Service_v2".into())
            .description("Backend API (version 2.0)".into())
            .add_container(
                Container::builder()
                    .name("Web/App".into())
                    .description("Frontend".into())
                    .container_type(ContainerType::WebApplication)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();

        let result = StructurizrDslSerializer::new()
            .with_name("Special Chars Test")
            .with_description("Test with special characters")
            .add_person(person)
            .add_software_system(system)
            .serialize()
            .unwrap();

        assert!(result.contains("us = person \"User's System\""));
        assert!(result.contains("a = softwareSystem \"API-Service_v2\""));
        assert!(result.contains("w = container \"Web/App\""));
        assert!(result.contains("special"));
    }

    #[test]
    fn test_relationship_with_technology() {
        let person: Person = Person::builder()
            .name("User".into())
            .description("A user".into())
            .build()
            .unwrap();

        let system: SoftwareSystem = SoftwareSystem::builder()
            .name("API-Service_v2".into())
            .description("Backend API (version 2.0)".into())
            .add_container(
                Container::builder()
                    .name("Web/App".into())
                    .description("Frontend".into())
                    .container_type(ContainerType::WebApplication)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();

        let result = StructurizrDslSerializer::new()
            .with_name("Tech Test")
            .with_description("Test")
            .add_person(person)
            .add_software_system(system)
            .add_relationship("u", "a", "Uses", Some("HTTPS"))
            .add_relationship("a", "u", "Responds to", Some("JSON/HTTPS"))
            .serialize()
            .unwrap();

        assert!(result.contains("u -> a \"Uses\" \"HTTPS\""));
        assert!(result.contains("a -> u \"Responds to\" \"JSON/HTTPS\""));
    }

    #[test]
    fn test_multiple_identical_element_names() {
        let person1: Person = Person::builder()
            .name("User".into())
            .description("First user".into())
            .build()
            .unwrap();

        let person2: Person = Person::builder()
            .name("User".into())
            .description("Second user".into())
            .build()
            .unwrap();

        let person3: Person = Person::builder()
            .name("User".into())
            .description("Third user".into())
            .build()
            .unwrap();

        let result = StructurizrDslSerializer::new()
            .with_name("Duplicate Names Test")
            .with_description("Test")
            .add_person(person1)
            .add_person(person2)
            .add_person(person3)
            .serialize()
            .unwrap();

        assert!(result.contains("u = person \"User\" \"First user\""));
        assert!(result.contains("u1 = person \"User\" \"Second user\""));
        assert!(result.contains("u2 = person \"User\" \"Third user\""));
    }

    #[test]
    fn test_golden_file_complete_workspace() {
        let person: Person = Person::builder()
            .name("User".into())
            .description("A user of the system".into())
            .build()
            .unwrap();

        let system: SoftwareSystem = SoftwareSystem::builder()
            .name("API".into())
            .description("Backend API service".into())
            .add_container(
                Container::builder()
                    .name("Web App".into())
                    .description("Frontend".into())
                    .container_type(ContainerType::WebApplication)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();

        let view = ViewConfiguration::builder()
            .view_type(ViewType::SystemContext)
            .element_identifier("a".to_string())
            .title("SystemContext".to_string())
            .include_elements(vec!["*".to_string()])
            .build();

        let result = StructurizrDslSerializer::new()
            .with_name("Example System")
            .with_description("An example C4 model")
            .add_person(person)
            .add_software_system(system)
            .add_view(view)
            .add_element_style(ElementStyle::new("Person").with_shape("person"))
            .serialize()
            .unwrap();

        let expected = r#"workspace "Example System" "An example C4 model" {
    !identifiers hierarchical

    model {
        u = person "User" "A user of the system"
        a = softwareSystem "API" "Backend API service" {
            wa = container "Web App" "Frontend" {}
        }
    }

    views {
        systemContext a "SystemContext" {
            include *
        }

        styles {
            element "Person" {
                shape person
            }
        }
    }
}"#;

        assert_eq!(result, expected);
    }
}
