//! Container serialization to Structurizr DSL format.

use crate::c4::Container;
use crate::serialization::error::StructurizrDslError;
use crate::serialization::templates::elements::ContainerTemplate;
use crate::serialization::templates::helpers::format_identifier;
use crate::serialization::traits::ElementSerializer;
use askama::Template;

/// Serializes a Container element to Structurizr DSL format.
///
/// Container format: `container = container "name" "description" "technology"`
impl ElementSerializer for Container {
    fn serialize_structurizr_dsl(&self) -> Result<String, StructurizrDslError> {
        let identifier = format_identifier(self.name());
        let template = ContainerTemplate {
            identifier,
            name: self.name().to_string(),
            description: self.description().to_string(),
            technology: Some(self.container_type().to_string()),
        };
        Ok(template.render()?)
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
