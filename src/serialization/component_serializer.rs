//! Component serialization to Structurizr DSL format.

use crate::c4::Component;
use crate::serialization::error::StructurizrDslError;
use crate::serialization::templates::elements::ComponentTemplate;
use crate::serialization::templates::helpers::format_identifier;
use crate::serialization::traits::ElementSerializer;
use askama::Template;

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
    use crate::c4::Component;

    #[test]
    fn test_component_serialization() {
        let component = Component::builder()
            .with_name("UserController".try_into().unwrap())
            .with_description("Handles user requests".try_into().unwrap())
            .with_technology("Rust".try_into().unwrap())
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
            .with_name("UserController".try_into().unwrap())
            .with_description("Handles user requests".try_into().unwrap())
            .build()
            .unwrap();

        let dsl = component.serialize_structurizr_dsl().unwrap();
        assert_eq!(
            dsl,
            r#"UserController = component "UserController" "Handles user requests" """#
        );
    }
}
