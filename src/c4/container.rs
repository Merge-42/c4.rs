use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

use super::component::Component;
use super::element::{ContainerType, Element, ElementType, Location};
use super::value_types::{ElementIdentifier, NonEmptyString};

pub mod container_builder {
    #[derive(Debug, Clone, Default)]
    pub struct NoName;
    #[derive(Debug, Clone, Default)]
    pub struct HasName;
    #[derive(Debug, Clone, Default)]
    pub struct NoDescription;
    #[derive(Debug, Clone, Default)]
    pub struct HasDescription;
    #[derive(Debug, Clone, Default)]
    pub struct NoContainerType;
    #[derive(Debug, Clone, Default)]
    pub struct HasContainerType;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Container {
    identifier: ElementIdentifier,
    name: NonEmptyString,
    description: NonEmptyString,
    container_type: ContainerType,
    technology: Option<NonEmptyString>,
    components: Vec<Component>,
}

impl Container {
    pub fn builder() -> ContainerBuilder<
        container_builder::NoName,
        container_builder::NoDescription,
        container_builder::NoContainerType,
    > {
        ContainerBuilder {
            _name: PhantomData,
            _description: PhantomData,
            _container_type: PhantomData,
            identifier: None,
            name: None,
            description: None,
            container_type: None,
            technology: None,
            components: Vec::new(),
        }
    }

    pub fn identifier(&self) -> &ElementIdentifier {
        &self.identifier
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
        Location::Internal
    }
}

#[derive(Debug, Clone)]
pub struct ContainerBuilder<N, D, T> {
    _name: PhantomData<N>,
    _description: PhantomData<D>,
    _container_type: PhantomData<T>,
    identifier: Option<ElementIdentifier>,
    name: Option<NonEmptyString>,
    description: Option<NonEmptyString>,
    container_type: Option<ContainerType>,
    technology: Option<NonEmptyString>,
    components: Vec<Component>,
}

impl Default
    for ContainerBuilder<
        container_builder::NoName,
        container_builder::NoDescription,
        container_builder::NoContainerType,
    >
{
    fn default() -> Self {
        Self::new()
    }
}

impl
    ContainerBuilder<
        container_builder::NoName,
        container_builder::NoDescription,
        container_builder::NoContainerType,
    >
{
    pub fn new() -> Self {
        ContainerBuilder {
            _name: PhantomData,
            _description: PhantomData,
            _container_type: PhantomData,
            identifier: None,
            name: None,
            description: None,
            container_type: None,
            technology: None,
            components: Vec::new(),
        }
    }
}

impl<D, T> ContainerBuilder<container_builder::NoName, D, T> {
    pub fn with_identifier(mut self, identifier: ElementIdentifier) -> Self {
        self.identifier = Some(identifier);
        self
    }

    pub fn with_name(
        self,
        name: NonEmptyString,
    ) -> ContainerBuilder<container_builder::HasName, D, T> {
        ContainerBuilder {
            _name: PhantomData,
            _description: self._description,
            _container_type: self._container_type,
            identifier: self.identifier,
            name: Some(name),
            description: self.description,
            container_type: self.container_type,
            technology: self.technology,
            components: self.components,
        }
    }
}

impl<N, T> ContainerBuilder<N, container_builder::NoDescription, T> {
    pub fn with_description(
        self,
        description: NonEmptyString,
    ) -> ContainerBuilder<N, container_builder::HasDescription, T> {
        ContainerBuilder {
            _name: self._name,
            _description: PhantomData,
            _container_type: self._container_type,
            identifier: self.identifier,
            name: self.name,
            description: Some(description),
            container_type: self.container_type,
            technology: self.technology,
            components: self.components,
        }
    }
}

impl<N, D> ContainerBuilder<N, D, container_builder::NoContainerType> {
    pub fn with_container_type(
        self,
        container_type: ContainerType,
    ) -> ContainerBuilder<N, D, container_builder::HasContainerType> {
        ContainerBuilder {
            _name: self._name,
            _description: self._description,
            _container_type: PhantomData,
            identifier: self.identifier,
            name: self.name,
            description: self.description,
            container_type: Some(container_type),
            technology: self.technology,
            components: self.components,
        }
    }
}

impl<N, D, T> ContainerBuilder<N, D, T> {
    pub fn with_technology(mut self, technology: NonEmptyString) -> Self {
        self.technology = Some(technology);
        self
    }

    pub fn add_component(mut self, component: Component) -> Self {
        self.components.push(component);
        self
    }
}

impl
    ContainerBuilder<
        container_builder::HasName,
        container_builder::HasDescription,
        container_builder::HasContainerType,
    >
{
    pub fn build(self) -> Container {
        if let Some(ref tech) = self.technology
            && tech.len() > 255
        {
            panic!("technology string exceeds maximum length of 255 characters");
        }
        Container {
            identifier: self.identifier.unwrap_or_default(),
            name: self.name.unwrap(),
            description: self.description.unwrap(),
            container_type: self.container_type.unwrap(),
            technology: self.technology,
            components: self.components,
        }
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
            .with_name("Web API".try_into().unwrap())
            .with_description("REST API endpoints".try_into().unwrap())
            .with_container_type(ContainerType::Api)
            .with_technology("Rust/Axum".try_into().unwrap())
            .build();

        assert_eq!(container.name(), "Web API");
        assert_eq!(container.container_type(), ContainerType::Api);
        assert_eq!(container.technology(), Some("Rust/Axum"));
    }

    #[test]
    fn test_container_with_components() {
        use super::super::Component;

        let container = Container::builder()
            .with_name("API".try_into().unwrap())
            .with_description("REST API".try_into().unwrap())
            .with_container_type(ContainerType::Api)
            .add_component(
                Component::builder()
                    .with_name("UserController".try_into().unwrap())
                    .with_description("User handling".try_into().unwrap())
                    .build(),
            )
            .build();

        assert_eq!(container.components().len(), 1);
    }
}
