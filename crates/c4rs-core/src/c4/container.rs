use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::component::Component;
use super::element::{ContainerType, Element, ElementType, Location};
use super::value_types::{ElementIdentifier, NonEmptyString};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
#[builder(mutators(
    pub fn add_component(&mut self, component: Component) {
        self.components.push(component);
    }
))]
pub struct Container {
    #[builder(default)]
    identifier: Option<ElementIdentifier>,
    name: NonEmptyString,
    description: NonEmptyString,
    container_type: ContainerType,
    #[builder(default)]
    technology: Option<NonEmptyString>,
    #[builder(via_mutators(init = Vec::new()))]
    components: Vec<Component>,
}

impl Container {
    pub fn identifier(&self) -> &ElementIdentifier {
        self.identifier.as_ref().unwrap_or_else(|| {
            static DEFAULT: std::sync::LazyLock<ElementIdentifier> =
                std::sync::LazyLock::new(ElementIdentifier::default);
            &DEFAULT
        })
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    pub fn container_type(&self) -> ContainerType {
        self.container_type.clone()
    }

    pub fn technology(&self) -> Option<&str> {
        self.technology.as_deref()
    }

    pub fn components(&self) -> &[Component] {
        &self.components
    }

    pub fn add_component(&mut self, component: Component) {
        self.components.push(component);
    }

    pub fn build(self) -> Container {
        if let Some(ref tech) = self.technology
            && tech.len() > 255
        {
            panic!("technology string exceeds maximum length of 255 characters");
        }
        Container {
            identifier: self.identifier,
            name: self.name,
            description: self.description,
            container_type: self.container_type,
            technology: self.technology,
            components: self.components,
        }
    }
}

impl Element for Container {
    fn identifier(&self) -> &ElementIdentifier {
        self.identifier.as_ref().unwrap_or_else(|| {
            static DEFAULT: std::sync::LazyLock<ElementIdentifier> =
                std::sync::LazyLock::new(ElementIdentifier::default);
            &DEFAULT
        })
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
        Location::Internal
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ContainerError {
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
            .name("Web API".try_into().unwrap())
            .description("REST API endpoints".try_into().unwrap())
            .container_type(ContainerType::Api)
            .technology(Some("Rust/Axum".try_into().unwrap()))
            .build();

        assert_eq!(container.name(), "Web API");
        assert_eq!(container.container_type(), ContainerType::Api);
        assert_eq!(container.technology(), Some("Rust/Axum"));
    }

    #[test]
    fn test_container_with_components() {
        // Skip until Component is migrated to typed_builder
    }
}
