use bon::Builder;
use serde::{Deserialize, Serialize};

use super::component::Component;
use super::element::{ContainerType, Element, ElementType, Location};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Builder)]
#[builder(finish_fn(vis = "", name = build_internal))]
pub struct Container {
    #[builder(field)]
    components: Vec<Component>,
    name: String,
    description: String,
    container_type: ContainerType,
    technology: Option<String>,
}

impl<S: container_builder::IsComplete> ContainerBuilder<S> {
    pub fn add_component(mut self, component: Component) -> Self {
        self.components.push(component);
        self
    }

    pub fn build(self) -> Result<Container, ContainerError> {
        let container = self.build_internal();

        if container.name.trim().is_empty() {
            return Err(ContainerError::MissingName);
        }
        if container.description.trim().is_empty() {
            return Err(ContainerError::MissingDescription);
        }
        if let Some(ref tech) = container.technology
            && tech.len() > 255
        {
            return Err(ContainerError::TechnologyTooLong {
                max: 255,
                actual: tech.len(),
            });
        }

        Ok(container)
    }
}

impl Container {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        technology: impl Into<String>,
        container_type: ContainerType,
    ) -> Result<Container, ContainerError> {
        let name = name.into();
        let description = description.into();
        let technology = technology.into();

        if name.trim().is_empty() {
            return Err(ContainerError::MissingName);
        }
        if description.trim().is_empty() {
            return Err(ContainerError::MissingDescription);
        }
        if technology.trim().is_empty() {
            return Err(ContainerError::MissingTechnology);
        }
        if technology.len() > 255 {
            return Err(ContainerError::TechnologyTooLong {
                max: 255,
                actual: technology.len(),
            });
        }

        Ok(Container {
            name,
            description,
            container_type,
            technology: Some(technology),
            components: Vec::new(),
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
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
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
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
    #[error("container technology is required and cannot be empty")]
    MissingTechnology,
    #[error("technology string exceeds maximum length of {max} characters (actual: {actual})")]
    TechnologyTooLong { max: usize, actual: usize },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_builder() {
        let container = Container::builder()
            .name("Web API".into())
            .description("REST API endpoints".into())
            .container_type(ContainerType::Api)
            .technology("Rust/Axum".into())
            .build()
            .unwrap();

        assert_eq!(container.name(), "Web API");
        assert_eq!(container.container_type(), ContainerType::Api);
        assert_eq!(container.technology(), Some("Rust/Axum"));
    }

    #[test]
    fn test_container_with_components() {
        // TODO: migrate Component first
    }
}
