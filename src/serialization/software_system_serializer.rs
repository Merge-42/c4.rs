//! SoftwareSystem serialization to Structurizr DSL format.

use crate::c4::SoftwareSystem;
use crate::serialization::error::StructurizrDslError;
use crate::serialization::templates::elements::SoftwareSystemTemplate;
use crate::serialization::traits::{ElementSerializer, escape_dsl_string, format_identifier};
use askama::Template;

/// Serializes a SoftwareSystem element to Structurizr DSL format.
///
/// SoftwareSystem format: `system = softwareSystem "name" "description"`
impl ElementSerializer for SoftwareSystem {
    fn serialize_structurizr_dsl(&self) -> Result<String, StructurizrDslError> {
        let identifier = format_identifier(self.name());
        let name = escape_dsl_string(self.name());
        let description = escape_dsl_string(self.description());

        let template = SoftwareSystemTemplate {
            identifier: &identifier,
            name: &name,
            description: &description,
        };
        Ok(template.render().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::c4::SoftwareSystem;

    #[test]
    fn test_software_system_serialization() {
        let system = SoftwareSystem::builder()
            .with_name("API".try_into().unwrap())
            .with_description("Backend API service".try_into().unwrap())
            .build()
            .unwrap();

        let dsl = system.serialize_structurizr_dsl().unwrap();
        assert_eq!(dsl, r#"API = softwareSystem "API" "Backend API service""#);
    }

    #[test]
    fn test_software_system_with_spaces() {
        let system = SoftwareSystem::builder()
            .with_name("Payment Gateway".try_into().unwrap())
            .with_description("Processes payments".try_into().unwrap())
            .build()
            .unwrap();

        let dsl = system.serialize_structurizr_dsl().unwrap();
        assert_eq!(
            dsl,
            r#"Payment_Gateway = softwareSystem "Payment Gateway" "Processes payments""#
        );
    }
}
