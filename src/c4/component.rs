use serde::{Deserialize, Serialize};

use super::element::{Element, ElementType, Location};
use super::value_types::{ElementIdentifier, NonEmptyString};

use super::code::CodeElement;

/// Represents a logical grouping of code within a container.
///
/// Components are the logical building blocks of a container.
/// They represent a grouping of related code that delivers a specific functionality.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Component {
    identifier: ElementIdentifier,
    name: NonEmptyString,
    description: NonEmptyString,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    responsibilities: Vec<NonEmptyString>,
    technology: Option<NonEmptyString>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    code_elements: Vec<CodeElement>,
}

impl Component {
    /// Creates a new ComponentBuilder.
    pub fn builder() -> ComponentBuilder {
        ComponentBuilder::new()
    }

    /// Returns a reference to the component's unique identifier.
    pub fn identifier(&self) -> &ElementIdentifier {
        &self.identifier
    }

    /// Returns the component's name.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the component's description.
    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    /// Returns the component's responsibilities.
    pub fn responsibilities(&self) -> Vec<String> {
        self.responsibilities
            .iter()
            .map(|s| s.as_str().to_string())
            .collect()
    }

    /// Returns the technology used by this component.
    pub fn technology(&self) -> Option<&str> {
        self.technology.as_deref()
    }

    /// Returns the code elements in this component.
    pub fn code_elements(&self) -> &[CodeElement] {
        &self.code_elements
    }

    /// Adds a responsibility to this component.
    pub fn add_responsibility(&mut self, responsibility: NonEmptyString) {
        self.responsibilities.push(responsibility);
    }

    /// Adds a code element to this component.
    pub fn add_code_element(&mut self, code_element: CodeElement) {
        self.code_elements.push(code_element);
    }
}

impl Element for Component {
    fn identifier(&self) -> &ElementIdentifier {
        &self.identifier
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn description(&self) -> &str {
        self.description.as_str()
    }

    fn element_type(&self) -> ElementType {
        ElementType::Component
    }

    fn location(&self) -> Location {
        Location::Internal
    }
}

/// Builder for constructing Component instances.
#[derive(Debug, Clone, Default)]
pub struct ComponentBuilder {
    identifier: Option<ElementIdentifier>,
    name: Option<NonEmptyString>,
    description: Option<NonEmptyString>,
    responsibilities: Vec<NonEmptyString>,
    technology: Option<NonEmptyString>,
    code_elements: Vec<CodeElement>,
}

impl ComponentBuilder {
    /// Creates a new ComponentBuilder.
    pub fn new() -> Self {
        Self {
            identifier: None,
            name: None,
            description: None,
            responsibilities: Vec::new(),
            technology: None,
            code_elements: Vec::new(),
        }
    }

    /// Sets the element identifier.
    pub fn with_identifier(mut self, identifier: ElementIdentifier) -> Self {
        self.identifier = Some(identifier);
        self
    }

    /// Sets the component's name.
    pub fn with_name(mut self, name: NonEmptyString) -> Self {
        self.name = Some(name);
        self
    }

    /// Sets the component's description.
    pub fn with_description(mut self, description: NonEmptyString) -> Self {
        self.description = Some(description);
        self
    }

    /// Adds a responsibility.
    pub fn add_responsibility(mut self, responsibility: NonEmptyString) -> Self {
        self.responsibilities.push(responsibility);
        self
    }

    /// Sets the technology used by this component.
    pub fn with_technology(mut self, technology: NonEmptyString) -> Self {
        self.technology = Some(technology);
        self
    }

    /// Adds a code element.
    pub fn add_code_element(mut self, code_element: CodeElement) -> Self {
        self.code_elements.push(code_element);
        self
    }

    /// Builds the Component.
    pub fn build(self) -> Result<Component, ComponentError> {
        let identifier = self.identifier.unwrap_or_else(ElementIdentifier::new);
        let name = self.name.ok_or(ComponentError::MissingName)?;
        let description = self.description.ok_or(ComponentError::MissingDescription)?;

        if let Some(ref tech) = self.technology {
            if tech.len() > 255 {
                return Err(ComponentError::TechnologyTooLong {
                    max: 255,
                    actual: tech.len(),
                });
            }
        }

        Ok(Component {
            identifier,
            name,
            description,
            responsibilities: self.responsibilities,
            technology: self.technology,
            code_elements: self.code_elements,
        })
    }
}

/// Error type for Component construction.
#[derive(Debug, thiserror::Error)]
pub enum ComponentError {
    #[error("component identifier is required")]
    MissingIdentifier,

    #[error("component name is required and cannot be empty")]
    MissingName,

    #[error("component description is required and cannot be empty")]
    MissingDescription,

    #[error("technology string exceeds maximum length of {max} characters (actual: {actual})")]
    TechnologyTooLong { max: usize, actual: usize },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_builder() {
        let component = Component::builder()
            .with_name("UserHandler".try_into().unwrap())
            .with_description("Handles user-related requests".try_into().unwrap())
            .add_responsibility("Create user".try_into().unwrap())
            .add_responsibility("Update user".try_into().unwrap())
            .with_technology("Rust".try_into().unwrap())
            .build()
            .unwrap();

        assert_eq!(component.name(), "UserHandler");
        assert_eq!(component.responsibilities().len(), 2);
    }

    #[test]
    fn test_component_error_missing_name() {
        let result = Component::builder()
            .with_description("No name".try_into().unwrap())
            .build();

        assert!(result.is_err());
    }
}
