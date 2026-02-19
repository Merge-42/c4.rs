//! Component serialization to Structurizr DSL format.

use crate::error::StructurizrDslError;
use crate::templates::elements::ComponentTemplate;
use crate::templates::helpers::format_identifier;
use crate::traits::ElementSerializer;
use askama::Template;
use c4rs_core::c4::Component;

/// Serializes a Component element to Structurizr DSL format.
///
/// Component format: `component = component "name" "description" "technology"`
impl ElementSerializer for Component {
    fn serialize_structurizr_dsl(&self) -> Result<String, StructurizrDslError> {
        let identifier = format_identifier(self.name());
        let template = ComponentTemplate {
            identifier,
            name: self.name().to_string(),
            description: self.description().to_string(),
            technology: self.technology().map(|s| s.to_string()),
        };
        Ok(template.render()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use c4rs_core::c4::Component;

    #[test]
    fn test_component_serialization() {
        let component = Component::builder()
            .name("UserController".into())
            .description("Handles user requests".into())
            .technology("Rust".into())
            .build()
            .unwrap();

        let dsl = component.serialize_structurizr_dsl().unwrap();
        assert_eq!(
            dsl,
            r#"UserController = component "UserController" "Handles user requests" "Rust""#
        );
    }

    #[test]
    fn test_component_without_technology() {
        let component = Component::builder()
            .name("UserController".into())
            .description("Handles user requests".into())
            .build()
            .unwrap();

        let dsl = component.serialize_structurizr_dsl().unwrap();
        assert_eq!(
            dsl,
            r#"UserController = component "UserController" "Handles user requests" """#
        );
    }
}
