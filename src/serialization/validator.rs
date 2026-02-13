//! Hierarchy validation for Structurizr DSL serialization.

use crate::c4::ElementType;
use crate::serialization::error::StructurizrDslError;

/// Validates parent-child relationships in the C4 model hierarchy.
///
/// The C4 hierarchy is:
/// - Person (no parent)
/// - SoftwareSystem (no parent)
/// - Container parent must be SoftwareSystem
/// - Component parent must be Container
#[derive(Debug, Default)]
pub struct HierarchyValidator {
    software_systems: Vec<String>,
    containers: Vec<String>,
}

impl HierarchyValidator {
    /// Create a new hierarchy validator.
    pub fn new() -> Self {
        Self {
            software_systems: Vec::new(),
            containers: Vec::new(),
        }
    }

    /// Register a SoftwareSystem in the model.
    pub fn register_software_system(&mut self, name: &str) {
        self.software_systems.push(name.to_string());
    }

    /// Register a Container with its parent SoftwareSystem.
    pub fn register_container(&mut self, name: &str, parent: Option<&str>) {
        if let Some(parent_name) = parent {
            self.containers.push(format!("{}->{}", name, parent_name));
        } else {
            self.containers.push(name.to_string());
        }
    }

    /// Check if a Container has a valid SoftwareSystem parent.
    pub fn validate_container_parent(
        &self,
        container_name: &str,
        parent_name: Option<&str>,
    ) -> Result<(), StructurizrDslError> {
        if let Some(parent) = parent_name
            && !self.software_systems.contains(&parent.to_string())
        {
            return Err(StructurizrDslError::InvalidParentType {
                child: container_name.to_string(),
                expected: "SoftwareSystem".to_string(),
                actual: parent.to_string(),
            });
        }
        Ok(())
    }

    /// Check if a Component has a valid Container parent.
    pub fn validate_component_parent(
        &self,
        component_name: &str,
        parent_name: Option<&str>,
        container_parent: Option<&str>,
    ) -> Result<(), StructurizrDslError> {
        // First validate that the container has a valid parent
        if let Some(cp) = container_parent
            && !self.software_systems.contains(&cp.to_string())
        {
            return Err(StructurizrDslError::InvalidParentType {
                child: component_name.to_string(),
                expected: "SoftwareSystem".to_string(),
                actual: cp.to_string(),
            });
        }

        if let Some(parent) = parent_name {
            // Check if this parent is a valid container
            let parent_is_container = self
                .containers
                .iter()
                .any(|c| c == parent || c.starts_with(&format!("{}->", parent)));

            if !parent_is_container {
                return Err(StructurizrDslError::InvalidParentType {
                    child: component_name.to_string(),
                    expected: "Container".to_string(),
                    actual: parent.to_string(),
                });
            }
        }
        Ok(())
    }

    /// Detect circular relationships in the parent chain.
    pub fn detect_circular_relationship(
        &self,
        child_name: &str,
        parent_chain: &[String],
    ) -> Result<(), StructurizrDslError> {
        if parent_chain.contains(&child_name.to_string()) {
            return Err(StructurizrDslError::CircularRelationship(
                child_name.to_string(),
            ));
        }
        Ok(())
    }
}

/// Represents a parent-child relationship in the C4 model.
#[derive(Debug, Clone)]
pub struct ParentChildRelationship {
    child_name: String,
    child_type: ElementType,
    parent_name: String,
    parent_type: ElementType,
}

impl ParentChildRelationship {
    /// Create a new parent-child relationship.
    pub fn new(
        child_name: String,
        child_type: ElementType,
        parent_name: String,
        parent_type: ElementType,
    ) -> Self {
        Self {
            child_name,
            child_type,
            parent_name,
            parent_type,
        }
    }

    /// Serialize to DSL format.
    pub fn serialize(&self) -> String {
        format!(
            r#"{} <- {} "<- contained""#,
            self.child_name.replace(' ', "_"),
            self.parent_name.replace(' ', "_")
        )
    }

    /// Check if this is a valid hierarchy relationship.
    pub fn is_valid(&self) -> bool {
        matches!(
            (self.child_type.clone(), self.parent_type.clone()),
            (ElementType::Container, ElementType::SoftwareSystem)
                | (ElementType::Component, ElementType::Container)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::c4::{Container, ContainerType, SoftwareSystem};

    #[test]
    fn test_validate_valid_container_parent() {
        let mut validator = HierarchyValidator::new();
        validator.register_software_system("API");

        let result = validator.validate_container_parent("WebApp", Some("API"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_invalid_container_parent() {
        let mut validator = HierarchyValidator::new();
        validator.register_software_system("API");

        let result = validator.validate_container_parent("WebApp", Some("Database"));
        assert!(result.is_err());
        if let Err(e) = result {
            let err_msg = e.to_string();
            assert!(err_msg.contains("invalid parent type"));
        }
    }

    #[test]
    fn test_validate_container_without_parent() {
        let validator = HierarchyValidator::new();

        let result = validator.validate_container_parent("WebApp", None);
        assert!(result.is_ok()); // Container without parent is valid
    }

    #[test]
    fn test_circular_detection() {
        let validator = HierarchyValidator::new();
        let chain = vec!["A".to_string(), "B".to_string(), "C".to_string()];

        let result = validator.detect_circular_relationship("A", &chain);
        assert!(result.is_err());
    }

    #[test]
    fn test_parent_child_relationship_serialization() {
        let rel = ParentChildRelationship::new(
            "WebApp".to_string(),
            ElementType::Container,
            "API".to_string(),
            ElementType::SoftwareSystem,
        );

        let dsl = rel.serialize();
        assert_eq!(dsl, r#"WebApp <- API "<- contained""#);
    }

    #[test]
    fn test_valid_hierarchy() {
        let rel = ParentChildRelationship::new(
            "WebApp".to_string(),
            ElementType::Container,
            "API".to_string(),
            ElementType::SoftwareSystem,
        );
        assert!(rel.is_valid());
    }

    #[test]
    fn test_invalid_hierarchy() {
        let rel = ParentChildRelationship::new(
            "API".to_string(),
            ElementType::SoftwareSystem,
            "WebApp".to_string(),
            ElementType::Container,
        );
        assert!(!rel.is_valid()); // SoftwareSystem shouldn't have Container parent
    }
}
