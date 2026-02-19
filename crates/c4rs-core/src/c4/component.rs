use bon::Builder;
use serde::{Deserialize, Serialize};

use super::code::CodeElement;
use super::element::{Element, ElementType, Location};
use super::value_types::ElementIdentifier;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Builder)]
#[builder(finish_fn(vis = "", name = build_internal))]
pub struct Component {
    #[builder(field)]
    code_elements: Vec<CodeElement>,
    #[serde(skip)]
    identifier: Option<ElementIdentifier>,
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
            identifier: None,
            name,
            description,
            responsibilities: Vec::new(),
            technology: None,
            code_elements: Vec::new(),
        })
    }

    pub fn identifier(&self) -> &ElementIdentifier {
        self.identifier.as_ref().unwrap_or_else(|| {
            static DEFAULT: std::sync::LazyLock<ElementIdentifier> =
                std::sync::LazyLock::new(ElementIdentifier::default);
            &DEFAULT
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
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

impl Element for Component {
    fn identifier(&self) -> &ElementIdentifier {
        self.identifier.as_ref().unwrap_or_else(|| {
            static DEFAULT: std::sync::LazyLock<ElementIdentifier> =
                std::sync::LazyLock::new(ElementIdentifier::default);
            &DEFAULT
        })
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn element_type(&self) -> ElementType {
        ElementType::Component
    }

    fn location(&self) -> Location {
        Location::Internal
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ComponentError {
    #[error("component name is required and cannot be empty")]
    MissingName,
    #[error("component description is required and cannot be empty")]
    MissingDescription,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_builder() {
        let component = Component::builder()
            .name("UserHandler".into())
            .description("Handles user requests".into())
            .responsibilities(vec!["Create user".into(), "Update user".into()])
            .technology("Rust".into())
            .build()
            .unwrap();

        assert_eq!(component.name(), "UserHandler");
        assert_eq!(component.responsibilities().len(), 2);
    }

    #[test]
    fn test_component_with_code_elements() {
        // TODO: migrate CodeElement first
    }
}
