use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

use super::code::CodeElement;
use super::element::{Element, ElementType, Location};
use super::value_types::{ElementIdentifier, NonEmptyString};

pub mod component_builder {
    #[derive(Debug, Clone, Default)]
    pub struct NoName;
    #[derive(Debug, Clone, Default)]
    pub struct HasName;
    #[derive(Debug, Clone, Default)]
    pub struct NoDescription;
    #[derive(Debug, Clone, Default)]
    pub struct HasDescription;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Component {
    identifier: ElementIdentifier,
    name: NonEmptyString,
    description: NonEmptyString,
    responsibilities: Vec<NonEmptyString>,
    technology: Option<NonEmptyString>,
    code_elements: Vec<CodeElement>,
}

impl Component {
    pub fn builder() -> ComponentBuilder<component_builder::NoName, component_builder::NoDescription>
    {
        ComponentBuilder {
            _name: PhantomData,
            _description: PhantomData,
            identifier: None,
            name: None,
            description: None,
            responsibilities: Vec::new(),
            technology: None,
            code_elements: Vec::new(),
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

    pub fn add_responsibility(&mut self, responsibility: NonEmptyString) {
        self.responsibilities.push(responsibility);
    }

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

#[derive(Debug, Clone)]
pub struct ComponentBuilder<N, D> {
    _name: PhantomData<N>,
    _description: PhantomData<D>,
    identifier: Option<ElementIdentifier>,
    name: Option<NonEmptyString>,
    description: Option<NonEmptyString>,
    responsibilities: Vec<NonEmptyString>,
    technology: Option<NonEmptyString>,
    code_elements: Vec<CodeElement>,
}

impl Default for ComponentBuilder<component_builder::NoName, component_builder::NoDescription> {
    fn default() -> Self {
        Self::new()
    }
}

impl ComponentBuilder<component_builder::NoName, component_builder::NoDescription> {
    pub fn new() -> Self {
        ComponentBuilder {
            _name: PhantomData,
            _description: PhantomData,
            identifier: None,
            name: None,
            description: None,
            responsibilities: Vec::new(),
            technology: None,
            code_elements: Vec::new(),
        }
    }
}

impl<D> ComponentBuilder<component_builder::NoName, D> {
    pub fn with_identifier(mut self, identifier: ElementIdentifier) -> Self {
        self.identifier = Some(identifier);
        self
    }

    pub fn with_name(
        self,
        name: NonEmptyString,
    ) -> ComponentBuilder<component_builder::HasName, D> {
        ComponentBuilder {
            _name: PhantomData,
            _description: self._description,
            identifier: self.identifier,
            name: Some(name),
            description: self.description,
            responsibilities: self.responsibilities,
            technology: self.technology,
            code_elements: self.code_elements,
        }
    }
}

impl<N> ComponentBuilder<N, component_builder::NoDescription> {
    pub fn with_description(
        self,
        description: NonEmptyString,
    ) -> ComponentBuilder<N, component_builder::HasDescription> {
        ComponentBuilder {
            _name: self._name,
            _description: PhantomData,
            identifier: self.identifier,
            name: self.name,
            description: Some(description),
            responsibilities: self.responsibilities,
            technology: self.technology,
            code_elements: self.code_elements,
        }
    }
}

impl<N, D> ComponentBuilder<N, D> {
    pub fn with_technology(mut self, technology: NonEmptyString) -> Self {
        self.technology = Some(technology);
        self
    }

    pub fn add_responsibility(mut self, responsibility: NonEmptyString) -> Self {
        self.responsibilities.push(responsibility);
        self
    }

    pub fn add_code_element(mut self, code_element: CodeElement) -> Self {
        self.code_elements.push(code_element);
        self
    }
}

impl ComponentBuilder<component_builder::HasName, component_builder::HasDescription> {
    pub fn build(self) -> Component {
        if let Some(ref tech) = self.technology
            && tech.len() > 255
        {
            panic!("technology string exceeds maximum length of 255 characters");
        }
        Component {
            identifier: self.identifier.unwrap_or_default(),
            name: self.name.unwrap(),
            description: self.description.unwrap(),
            responsibilities: self.responsibilities,
            technology: self.technology,
            code_elements: self.code_elements,
        }
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
            .with_name("UserHandler".try_into().unwrap())
            .with_description("Handles user-related requests".try_into().unwrap())
            .add_responsibility("Create user".try_into().unwrap())
            .add_responsibility("Update user".try_into().unwrap())
            .with_technology("Rust".try_into().unwrap())
            .build();

        assert_eq!(component.name(), "UserHandler");
        assert_eq!(component.responsibilities().len(), 2);
    }

    #[test]
    fn test_component_with_code_elements() {
        use super::super::CodeElement;
        use super::super::CodeType;

        let component = Component::builder()
            .with_name("UserHandler".try_into().unwrap())
            .with_description("Handles user requests".try_into().unwrap())
            .add_code_element(
                CodeElement::builder()
                    .with_name("createUser".try_into().unwrap())
                    .with_description("Creates a user".try_into().unwrap())
                    .with_code_type(CodeType::Function)
                    .build(),
            )
            .build();

        assert_eq!(component.code_elements().len(), 1);
    }
}
