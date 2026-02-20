use bon::Builder;
use serde::{Deserialize, Serialize};

use super::code::CodeElement;
use super::element::ElementType;
use super::macros::impl_element;
use crate::constants::limits::{
    MAX_DESCRIPTION_LENGTH, MAX_NAME_LENGTH, MAX_RESPONSIBILITY_LENGTH, MAX_TECHNOLOGY_LENGTH,
};
use crate::validation::{validate_max_length, validate_non_empty, validate_vec_max_length};

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
        validate_non_empty(&component.name, "name")?;
        validate_max_length(&component.name, MAX_NAME_LENGTH, "name")?;
        validate_non_empty(&component.description, "description")?;
        validate_max_length(
            &component.description,
            MAX_DESCRIPTION_LENGTH,
            "description",
        )?;
        validate_max_length(&component.technology, MAX_TECHNOLOGY_LENGTH, "technology")?;
        validate_vec_max_length(
            &component.responsibilities,
            MAX_RESPONSIBILITY_LENGTH,
            "responsibilities",
        )?;
        Ok(component)
    }
}

impl Component {
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
    #[error(
        "responsibility at index {index} exceeds maximum length of {max} characters (actual: {actual})"
    )]
    ResponsibilityTooLong {
        index: usize,
        max: usize,
        actual: usize,
    },
    #[error("validation error: {0}")]
    Validation(#[from] crate::validation::ValidationError),
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
