use bon::Builder;
use serde::{Deserialize, Serialize};

use super::code::CodeElement;
use super::element::ElementType;
use super::macros::impl_element;
use crate::constants::limits::MAX_TECHNOLOGY_LENGTH;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Builder)]
#[builder(finish_fn(vis = "", name = build_internal))]
pub struct Component {
    #[builder(field)]
    code_elements: Vec<CodeElement>,
    name: String,
    description: String,
    #[builder(default)]
    responsibilities: Vec<String>,
    technology: Option<String>,
}

impl<S: component_builder::IsComplete> ComponentBuilder<S> {
    pub fn add_code_element(mut self, code_element: CodeElement) -> Self {
        self.code_elements.push(code_element);
        self
    }
    pub fn build(self) -> Result<Component, ComponentError> {
        let component = self.build_internal();
        if component.name.trim().is_empty() {
            return Err(ComponentError::MissingName);
        }
        if component.description.trim().is_empty() {
            return Err(ComponentError::MissingDescription);
        }
        if let Some(ref tech) = component.technology
            && tech.len() > MAX_TECHNOLOGY_LENGTH
        {
            return Err(ComponentError::TechnologyTooLong {
                max: MAX_TECHNOLOGY_LENGTH,
                actual: tech.len(),
            });
        }
        Ok(component)
    }
}

impl Component {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Result<Component, ComponentError> {
        let name = name.into();
        let description = description.into();
        if name.trim().is_empty() {
            return Err(ComponentError::MissingName);
        }
        if description.trim().is_empty() {
            return Err(ComponentError::MissingDescription);
        }
        Ok(Component {
            name,
            description,
            responsibilities: Vec::new(),
            technology: None,
            code_elements: Vec::new(),
        })
    }
    pub fn responsibilities(&self) -> &[String] {
        &self.responsibilities
    }
    pub fn technology(&self) -> Option<&str> {
        self.technology.as_deref()
    }
    pub fn code_elements(&self) -> &[CodeElement] {
        &self.code_elements
    }
    pub fn add_code_element(&mut self, code_element: CodeElement) {
        self.code_elements.push(code_element);
    }
}

impl_element!(Component, ElementType::Component);

#[derive(Debug, thiserror::Error)]
pub enum ComponentError {
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
        let c = Component::builder()
            .name("Handler".into())
            .description("Handles requests".into())
            .technology("Rust".into())
            .build()
            .unwrap();
        assert_eq!(c.name(), "Handler");
    }
}
