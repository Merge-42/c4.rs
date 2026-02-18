use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::code::CodeElement;
use super::element::{Element, ElementType, Location};
use super::value_types::{ElementIdentifier, NonEmptyString};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
#[builder(mutators(
    pub fn add_code_element(&mut self, code_element: CodeElement) {
        self.code_elements.push(code_element);
    }
))]
pub struct Component {
    #[serde(skip)]
    #[builder(default)]
    identifier: Option<ElementIdentifier>,
    name: NonEmptyString,
    description: NonEmptyString,
    #[builder(default)]
    responsibilities: Vec<NonEmptyString>,
    #[builder(default, setter(strip_option))]
    technology: Option<NonEmptyString>,
    #[builder(via_mutators(init = Vec::new()))]
    code_elements: Vec<CodeElement>,
}

impl Component {
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

    pub fn responsibilities(&self) -> Vec<String> {
        self.responsibilities
            .iter()
            .map(|s| s.as_str().to_string())
            .collect()
    }

    pub fn technology(&self) -> Option<&str> {
        self.technology.as_deref()
    }

    pub fn code_elements(&self) -> &[CodeElement] {
        &self.code_elements
    }

    pub fn build(self) -> Result<Component, ComponentError> {
        Ok(Component {
            identifier: self.identifier,
            name: self.name,
            description: self.description,
            responsibilities: self.responsibilities,
            technology: self.technology,
            code_elements: self.code_elements,
        })
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
        let component = Component::builder()
            .name("UserHandler".into())
            .description("Handles user requests".into())
            .responsibilities(vec!["Create user".into(), "Update user".into()])
            .technology("Rust".into())
            .build();

        assert_eq!(component.name(), "UserHandler");
        assert_eq!(component.responsibilities().len(), 2);
    }

    #[test]
    fn test_component_with_code_elements() {
        // Skip until CodeElement is migrated to typed_builder
    }
}
