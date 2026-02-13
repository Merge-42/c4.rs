use serde::{Deserialize, Serialize};

use super::element::{ContainerType, Element, ElementType, Location};
use super::value_types::{ElementIdentifier, NonEmptyString};

use super::component::Component;

/// Represents a deployable unit within a software system.
///
/// Containers are the executable units that make up a software system.
/// Examples include web applications, databases, file systems, or APIs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Container {
    identifier: ElementIdentifier,
    name: NonEmptyString,
    description: NonEmptyString,
    container_type: ContainerType,
    technology: Option<NonEmptyString>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    components: Vec<Component>,
}

impl Container {
    /// Creates a new ContainerBuilder.
    pub fn builder() -> ContainerBuilder {
        ContainerBuilder::new()
    }

    /// Returns a reference to the container's unique identifier.
    pub fn identifier(&self) -> &ElementIdentifier {
        &self.identifier
    }

    /// Returns the container's name.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the container's description.
    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    /// Returns the container's type.
    pub fn container_type(&self) -> ContainerType {
        self.container_type.clone()
    }

    /// Returns the technology used by this container.
    pub fn technology(&self) -> Option<&str> {
        self.technology.as_deref()
    }

    /// Returns the components in this container.
    pub fn components(&self) -> &[Component] {
        &self.components
    }

    /// Adds a component to this container.
    pub fn add_component(&mut self, component: Component) {
        self.components.push(component);
    }
}

impl Element for Container {
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
        ElementType::Container
    }

    fn location(&self) -> Location {
        Location::Internal // Containers are always internal to their software system
    }
}

/// Builder for constructing Container instances.
#[derive(Debug, Clone, Default)]
pub struct ContainerBuilder {
    identifier: Option<ElementIdentifier>,
    name: Option<NonEmptyString>,
    description: Option<NonEmptyString>,
    container_type: Option<ContainerType>,
    technology: Option<NonEmptyString>,
    components: Vec<Component>,
}

impl ContainerBuilder {
    /// Creates a new ContainerBuilder.
    pub fn new() -> Self {
        Self {
            identifier: None,
            name: None,
            description: None,
            container_type: None,
            technology: None,
            components: Vec::new(),
        }
    }

    /// Sets the element identifier.
    pub fn with_identifier(mut self, identifier: ElementIdentifier) -> Self {
        self.identifier = Some(identifier);
        self
    }

    /// Sets the container's name.
    pub fn with_name(mut self, name: NonEmptyString) -> Self {
        self.name = Some(name);
        self
    }

    /// Sets the container's description.
    pub fn with_description(mut self, description: NonEmptyString) -> Self {
        self.description = Some(description);
        self
    }

    /// Sets the container's type.
    pub fn with_container_type(mut self, container_type: ContainerType) -> Self {
        self.container_type = Some(container_type);
        self
    }

    /// Sets the technology used by this container.
    pub fn with_technology(mut self, technology: NonEmptyString) -> Self {
        self.technology = Some(technology);
        self
    }

    /// Adds a component to the container.
    pub fn add_component(mut self, component: Component) -> Self {
        self.components.push(component);
        self
    }

    /// Builds the Container.
    pub fn build(self) -> Result<Container, ContainerError> {
        let identifier = self.identifier.unwrap_or_else(ElementIdentifier::new);
        let name = self.name.ok_or(ContainerError::MissingName)?;
        let description = self.description.ok_or(ContainerError::MissingDescription)?;
        let container_type = self.container_type.ok_or(ContainerError::MissingType)?;

        if let Some(ref tech) = self.technology {
            if tech.len() > 255 {
                return Err(ContainerError::TechnologyTooLong {
                    max: 255,
                    actual: tech.len(),
                });
            }
        }

        Ok(Container {
            identifier,
            name,
            description,
            container_type,
            technology: self.technology,
            components: self.components,
        })
    }
}

/// Error type for Container construction.
#[derive(Debug, thiserror::Error)]
pub enum ContainerError {
    #[error("container identifier is required")]
    MissingIdentifier,

    #[error("container name is required and cannot be empty")]
    MissingName,

    #[error("container description is required and cannot be empty")]
    MissingDescription,

    #[error("container type is required (e.g., Api, Database, WebApplication)")]
    MissingType,

    #[error("technology string exceeds maximum length of {max} characters (actual: {actual})")]
    TechnologyTooLong { max: usize, actual: usize },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_builder() {
        let container = Container::builder()
            .with_name("Web API".try_into().unwrap())
            .with_description("REST API endpoints".try_into().unwrap())
            .with_container_type(ContainerType::Api)
            .with_technology("Rust/Axum".try_into().unwrap())
            .build()
            .unwrap();

        assert_eq!(container.name(), "Web API");
        assert_eq!(container.container_type(), ContainerType::Api);
        assert_eq!(container.technology(), Some("Rust/Axum"));
    }

    #[test]
    fn test_container_error_missing_type() {
        let result = Container::builder()
            .with_name("API".try_into().unwrap())
            .with_description("Missing type".try_into().unwrap())
            .build();

        assert!(result.is_err());
    }
}
