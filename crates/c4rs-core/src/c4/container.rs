use bon::Builder;
use serde::{Deserialize, Serialize};

use super::component::Component;
use super::element::{ContainerType, ElementType};
use super::macros::impl_element;
use crate::constants::limits::{MAX_DESCRIPTION_LENGTH, MAX_NAME_LENGTH, MAX_TECHNOLOGY_LENGTH};
use crate::validation::{validate_max_length, validate_non_empty};

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
        validate_non_empty(&container.name, "name")?;
        validate_max_length(&container.name, MAX_NAME_LENGTH, "name")?;
        validate_non_empty(&container.description, "description")?;
        validate_max_length(
            &container.description,
            MAX_DESCRIPTION_LENGTH,
            "description",
        )?;
        validate_max_length(&container.technology, MAX_TECHNOLOGY_LENGTH, "technology")?;
        Ok(container)
    }
}

impl Container {
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

impl_element!(Container, ElementType::Container);

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
    #[error("validation error: {0}")]
    Validation(#[from] crate::validation::ValidationError),
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_container_builder() {
        let c = Container::builder()
            .name("API".into())
            .description("REST".into())
            .container_type(ContainerType::Api)
            .technology("Rust".into())
            .build()
            .unwrap();
        assert_eq!(c.name(), "API");
    }
}
