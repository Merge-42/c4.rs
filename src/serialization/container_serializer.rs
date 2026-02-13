//! Container serialization to Structurizr DSL format.

use crate::c4::Container;
use crate::serialization::error::StructurizrDslError;
use crate::serialization::traits::{ElementSerializer, format_identifier};
use crate::serialization::writer::format_element_assignment;

/// Serializes a Container element to Structurizr DSL format.
///
/// Container format: `container = container "name" "description" "technology"`
impl ElementSerializer for Container {
    fn serialize_structurizr_dsl(&self) -> Result<String, StructurizrDslError> {
        let identifier = format_identifier(self.name());
        let name = self.name();
        let description = self.description();
        let technology = self.container_type().to_string();

        Ok(format_element_assignment(
            &identifier,
            "container",
            name,
            description,
            Some(&technology),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::c4::{Container, ContainerType};

    #[test]
    fn test_container_serialization() {
        let container = Container::builder()
            .with_name("Web App".try_into().unwrap())
            .with_description("Frontend application".try_into().unwrap())
            .with_container_type(ContainerType::WebApplication)
            .build()
            .unwrap();

        let dsl = container.serialize_structurizr_dsl().unwrap();
        assert_eq!(
            dsl,
            r#"Web_App = container "Web App" "Frontend application" "Web Application""#
        );
    }

    #[test]
    fn test_container_database() {
        let container = Container::builder()
            .with_name("Database".try_into().unwrap())
            .with_description("Stores data".try_into().unwrap())
            .with_container_type(ContainerType::Database)
            .build()
            .unwrap();

        let dsl = container.serialize_structurizr_dsl().unwrap();
        assert_eq!(
            dsl,
            r#"Database = container "Database" "Stores data" "Database""#
        );
    }
}
