//! Main Structurizr DSL Serializer.

use crate::serialization::error::StructurizrDslError;
use crate::serialization::traits::ElementSerializer;
use crate::serialization::writer::DslWriter;

/// Serializer for converting C4 models to Structurizr DSL format.
#[derive(Debug, Default)]
pub struct StructurizrDslSerializer {
    writer: DslWriter,
}

impl StructurizrDslSerializer {
    /// Create a new Structurizr DSL serializer.
    pub fn new() -> Self {
        Self {
            writer: DslWriter::new(),
        }
    }

    /// Serialize a collection of elements to Structurizr DSL.
    ///
    /// # Arguments
    ///
    /// * `elements` - Slice of boxed element serializers
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

        Ok(self.writer.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::c4::{Container, ContainerType, Person, SoftwareSystem};

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
}
