//! Main Structurizr DSL Serializer.

use crate::serialization::error::StructurizrDslError;
use crate::serialization::styles_serializer::{ElementStyle, RelationshipStyle, StylesSerializer};
use crate::serialization::traits::ElementSerializer;
use crate::serialization::views_serializer::{ViewConfiguration, ViewsSerializer};
use crate::serialization::writer::DslWriter;

/// Serializer for converting C4 models to Structurizr DSL format.
#[derive(Debug, Default)]
pub struct StructurizrDslSerializer {
    writer: DslWriter,
    views_serializer: ViewsSerializer,
    styles_serializer: StylesSerializer,
}

impl StructurizrDslSerializer {
    /// Create a new Structurizr DSL serializer.
    pub fn new() -> Self {
        Self {
            writer: DslWriter::new(),
            views_serializer: ViewsSerializer::new(),
            styles_serializer: StylesSerializer::new(),
        }
    }

    /// Add a view configuration.
    pub fn add_view(&mut self, view: ViewConfiguration) {
        self.views_serializer.add_view(view);
    }

    /// Add an element style.
    pub fn add_element_style(&mut self, style: ElementStyle) {
        self.styles_serializer.add_element_style(style);
    }

    /// Add a relationship style.
    pub fn add_relationship_style(&mut self, style: RelationshipStyle) {
        self.styles_serializer.add_relationship_style(style);
    }

    /// Serialize a collection of elements to Structurizr DSL.
    ///
    /// # Arguments
    ///
    /// * `elements` - Slice of element serializers
    ///
    /// # Returns
    ///
    /// A string containing the Structurizr DSL representation.
    ///
    /// # Errors
    ///
    /// Returns a `StructurizrDslError` if serialization fails.
    pub fn serialize(
        &mut self,
        elements: &[&dyn ElementSerializer],
    ) -> Result<String, StructurizrDslError> {
        self.writer.clear();

        // Write workspace block with !identifiers
        self.writer.add_line("workspace {");
        self.writer.indent();
        self.writer
            .add_line("!identifiers people systems containers components");
        self.writer.unindent();
        self.writer.add_line("}");
        self.writer.add_empty_line();

        // Write model block
        self.writer.add_line("model {");
        self.writer.indent();

        // Serialize elements
        for element in elements {
            let dsl = element.serialize_structurizr_dsl()?;
            self.writer.add_line(&dsl);
        }

        self.writer.unindent();
        self.writer.add_line("}");
        self.writer.add_empty_line();

        // Write views block if views configured
        let views_dsl = self.views_serializer.serialize();
        if !views_dsl.is_empty() {
            self.writer.add_line(&views_dsl);
            self.writer.add_empty_line();
        }

        // Write styles block if styles configured
        let styles_dsl = self.styles_serializer.serialize();
        if !styles_dsl.is_empty() {
            self.writer.add_line(&styles_dsl);
        }

        Ok(self.writer.as_output())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::c4::{Container, ContainerType, ElementType, Person, SoftwareSystem};

    #[test]
    fn test_serialize_empty_model() {
        let mut serializer = StructurizrDslSerializer::new();
        let result = serializer.serialize(&[]).unwrap();
        assert!(result.contains("workspace {"));
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
        let result = serializer.serialize(&[&person]).unwrap();

        assert!(result.contains(r#"User = person "User" "A system user""#));
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
        let result = serializer
            .serialize(&[&person, &system, &container])
            .unwrap();

        assert!(result.contains(r#"User = person "User""#));
        assert!(result.contains(r#"API = softwareSystem "API""#));
        assert!(result.contains(r#"Web_App = container "Web App""#));
    }

    #[test]
    fn test_serialize_with_views() {
        let person: Person = Person::builder()
            .with_name("User".try_into().unwrap())
            .with_description("A system user".try_into().unwrap())
            .build()
            .unwrap();

        let mut serializer = StructurizrDslSerializer::new();
        let mut view = ViewConfiguration::new("context", "System Context", ElementType::Person);
        view.include_element("User");
        serializer.add_view(view);

        let result = serializer.serialize(&[&person]).unwrap();

        assert!(result.contains("views {"));
        assert!(result.contains("person context {"));
        assert!(result.contains("include User"));
    }

    #[test]
    fn test_serialize_with_styles() {
        let person: Person = Person::builder()
            .with_name("User".try_into().unwrap())
            .with_description("A system user".try_into().unwrap())
            .build()
            .unwrap();

        let mut serializer = StructurizrDslSerializer::new();
        serializer.add_element_style(
            ElementStyle::new("person", ElementType::Person)
                .with_background("#ffcc00")
                .with_color("#000000"),
        );

        let result = serializer.serialize(&[&person]).unwrap();

        assert!(result.contains("styles {"));
        assert!(result.contains("person {"));
        assert!(result.contains("background #ffcc00"));
    }
}
