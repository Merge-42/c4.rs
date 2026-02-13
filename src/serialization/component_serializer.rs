//! Component serialization to Structurizr DSL format.

use crate::c4::Component;
use crate::serialization::error::StructurizrDslError;
use crate::serialization::traits::{ElementSerializer, format_identifier};
use crate::serialization::writer::format_element_assignment;

/// Serializes a Component element to Structurizr DSL format.
///
/// Component format: `component = component "name" "description" "technology"`
impl ElementSerializer for Component {
    fn serialize_structurizr_dsl(&self) -> Result<String, StructurizrDslError> {
        let identifier = format_identifier(self.name());
        let name = self.name();
        let description = self.description();
        let technology = self.technology().unwrap_or("");

        Ok(format_element_assignment(
            &identifier,
            "component",
            name,
            description,
            Some(technology),
        ))
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
