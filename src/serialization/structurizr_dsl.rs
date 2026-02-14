//! Main Structurizr DSL Serializer.

use crate::c4::{Container, Person, SoftwareSystem};
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

    /// Add a container to a specific software system.
    pub fn add_container(&mut self, system_name: &str, container: Container) {
        self.workspace_serializer
            .add_container_to_system(system_name, container);
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
            .with_name("User".try_into().unwrap())
            .with_description("A system user".try_into().unwrap())
            .build()
            .unwrap();

        let mut serializer = StructurizrDslSerializer::new();
        serializer.add_person(person);
        let result = serializer.serialize().unwrap();

        assert!(result.contains(r#"u = person "User" "A system user""#));
    }

    #[test]
    fn test_serialize_full_model() {
        let person: Person = Person::builder()
            .with_name("User".try_into().unwrap())
            .with_description("A system user".try_into().unwrap())
            .build()
            .unwrap();

        let system: SoftwareSystem = SoftwareSystem::builder()
            .with_name("API".try_into().unwrap())
            .with_description("Backend API".try_into().unwrap())
            .build()
            .unwrap();

        let container: Container = Container::builder()
            .with_name("Web App".try_into().unwrap())
            .with_description("Frontend".try_into().unwrap())
            .with_container_type(ContainerType::WebApplication)
            .build()
            .unwrap();

        let mut serializer = StructurizrDslSerializer::new();
        serializer.add_person(person);
        serializer.add_software_system(system);
        serializer.add_container("API", container);
        let result = serializer.serialize().unwrap();

        assert!(result.contains(r#"u = person "User""#));
        assert!(result.contains(r#"a = softwareSystem "API""#));
        assert!(result.contains(r#"wa = container "Web App""#));
    }

    #[test]
    fn test_serialize_with_views() {
        let person: Person = Person::builder()
            .with_name("User".try_into().unwrap())
            .with_description("A system user".try_into().unwrap())
            .build()
            .unwrap();

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
            .with_name("User".try_into().unwrap())
            .with_description("A system user".try_into().unwrap())
            .build()
            .unwrap();

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
}
