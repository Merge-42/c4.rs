//! Container serialization to Structurizr DSL format.

use crate::error::StructurizrDslError;
use crate::templates::elements::ContainerTemplate;
use crate::templates::helpers::format_identifier;
use crate::traits::ElementSerializer;
use askama::Template;
use c4rs_core::c4::Container;

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
    use c4rs_core::c4::{Container, ContainerType};

    #[test]
    fn test_container_serialization() {
        let container = Container::builder()
            .name("Web App".into())
            .description("Frontend application".into())
            .container_type(ContainerType::WebApplication)
            .build();

        let dsl = container.serialize_structurizr_dsl().unwrap();
        assert_eq!(
            dsl,
            r#"Web_App = container "Web App" "Frontend application" "Web Application""#
        );
    }

    #[test]
    fn test_container_database() {
        let container = Container::builder()
            .name("Database".into())
            .description("Stores data".into())
            .container_type(ContainerType::Database)
            .build();

        let dsl = container.serialize_structurizr_dsl().unwrap();
        assert_eq!(
            dsl,
            r#"Database = container "Database" "Stores data" "Database""#
        );
    }
}
