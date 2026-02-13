//! Hierarchy serialization for C4 models.

use crate::serialization::error::StructurizrDslError;
use crate::serialization::traits::format_identifier;
use crate::serialization::validator::HierarchyValidator;
use crate::serialization::writer::DslWriter;

/// Handles serialization of parent-child hierarchies in C4 models.
///
/// This module provides functionality to:
/// - Track parent-child relationships during serialization
/// - Validate hierarchy constraints
/// - Output parent references in Structurizr DSL format
#[derive(Debug, Default)]
pub struct HierarchySerializer {
    writer: DslWriter,
    validator: HierarchyValidator,
}

impl HierarchySerializer {
    /// Create a new hierarchy serializer.
    pub fn new() -> Self {
        Self {
            writer: DslWriter::new(),
            validator: HierarchyValidator::new(),
        }
    }

    /// Register a SoftwareSystem for hierarchy validation.
    pub fn register_software_system(&mut self, name: &str) {
        self.validator.register_software_system(name);
    }

    /// Register a Container with its parent SoftwareSystem.
    pub fn register_container(&mut self, container_name: &str, parent_name: Option<&str>) {
        self.validator
            .register_container(container_name, parent_name);
    }

    /// Serialize a parent-child relationship.
    pub fn serialize_parent_reference(
        &mut self,
        child_name: &str,
        parent_name: &str,
    ) -> Result<(), StructurizrDslError> {
        let child = format_identifier(child_name);
        let parent = format_identifier(parent_name);
        self.writer
            .add_line(&format!(r#"{} <- {} "<- contained""#, child, parent));
        Ok(())
    }

    /// Validate a Container's parent.
    pub fn validate_container_parent(
        &self,
        container_name: &str,
        parent_name: Option<&str>,
    ) -> Result<(), StructurizrDslError> {
        self.validator
            .validate_container_parent(container_name, parent_name)
    }

    /// Validate a Component's parent.
    pub fn validate_component_parent(
        &self,
        component_name: &str,
        parent_name: Option<&str>,
        container_parent: Option<&str>,
    ) -> Result<(), StructurizrDslError> {
        self.validator
            .validate_component_parent(component_name, parent_name, container_parent)
    }

    /// Detect circular relationships.
    pub fn detect_circular(
        &self,
        child_name: &str,
        chain: &[String],
    ) -> Result<(), StructurizrDslError> {
        self.validator
            .detect_circular_relationship(child_name, chain)
    }

    /// Get the accumulated DSL output.
    pub fn as_output(&self) -> String {
        self.writer.as_output()
    }

    /// Clear the serializer state.
    pub fn clear(&mut self) {
        self.writer.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::c4::{Container, ContainerType, SoftwareSystem};

    #[test]
    fn test_hierarchy_serialization() {
        let mut serializer = HierarchySerializer::new();
        serializer
            .serialize_parent_reference("WebApp", "API")
            .unwrap();

        let output = serializer.as_output();
        assert_eq!(output, r#"WebApp <- API "<- contained""#);
    }

    #[test]
    fn test_validate_container_parent_valid() {
        let mut serializer = HierarchySerializer::new();
        serializer.register_software_system("API");

        let result = serializer.validate_container_parent("WebApp", Some("API"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_container_parent_invalid() {
        let mut serializer = HierarchySerializer::new();
        serializer.register_software_system("API");

        let result = serializer.validate_container_parent("WebApp", Some("Database"));
        assert!(result.is_err());
    }
}
