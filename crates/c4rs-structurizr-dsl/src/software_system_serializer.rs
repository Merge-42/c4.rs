//! SoftwareSystem serialization to Structurizr DSL format.

use crate::error::StructurizrDslError;
use crate::templates::elements::SoftwareSystemTemplate;
use crate::templates::helpers::format_identifier;
use crate::traits::ElementSerializer;
use askama::Template;
use c4rs_core::c4::SoftwareSystem;

/// Serializes a SoftwareSystem element to Structurizr DSL format.
///
/// SoftwareSystem format: `system = softwareSystem "name" "description"`
impl ElementSerializer for SoftwareSystem {
    fn serialize_structurizr_dsl(&self) -> Result<String, StructurizrDslError> {
        let identifier = format_identifier(self.name());
        let template = SoftwareSystemTemplate {
            identifier,
            name: self.name().to_string(),
            description: self.description().to_string(),
        };
        Ok(template.render()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use c4rs_core::c4::SoftwareSystem;

    #[test]
    fn test_software_system_serialization() {
        let system = SoftwareSystem::builder()
            .name("API".into())
            .description("Backend API service".into())
            .build();

        let dsl = system.serialize_structurizr_dsl().unwrap();
        assert_eq!(dsl, r#"API = softwareSystem "API" "Backend API service""#);
    }

    #[test]
    fn test_software_system_with_spaces() {
        let system = SoftwareSystem::builder()
            .name("Payment Gateway".into())
            .description("Processes payments".into())
            .build();

        let dsl = system.serialize_structurizr_dsl().unwrap();
        assert_eq!(
            dsl,
            r#"Payment_Gateway = softwareSystem "Payment Gateway" "Processes payments""#
        );
    }
}
