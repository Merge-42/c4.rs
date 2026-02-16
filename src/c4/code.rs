use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

use super::element::{CodeType, Element, ElementType, Location};
use super::value_types::{ElementIdentifier, NonEmptyString};

pub mod code_element_builder {
    #[derive(Debug, Clone, Default)]
    pub struct NoName;
    #[derive(Debug, Clone, Default)]
    pub struct HasName;
    #[derive(Debug, Clone, Default)]
    pub struct NoDescription;
    #[derive(Debug, Clone, Default)]
    pub struct HasDescription;
    #[derive(Debug, Clone, Default)]
    pub struct NoCodeType;
    #[derive(Debug, Clone, Default)]
    pub struct HasCodeType;
}

/// Represents an individual code unit within a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeElement {
    identifier: ElementIdentifier,
    name: NonEmptyString,
    description: NonEmptyString,
    code_type: CodeType,
    language: Option<NonEmptyString>,
    file_path: Option<NonEmptyString>,
}

impl CodeElement {
    pub fn builder() -> CodeElementBuilder<
        code_element_builder::NoName,
        code_element_builder::NoDescription,
        code_element_builder::NoCodeType,
    > {
        CodeElementBuilder::new()
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

    pub fn code_type(&self) -> CodeType {
        self.code_type.clone()
    }

    pub fn language(&self) -> Option<&str> {
        self.language.as_deref()
    }

    pub fn file_path(&self) -> Option<&str> {
        self.file_path.as_deref()
    }
}

impl Element for CodeElement {
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
        ElementType::Code
    }

    fn location(&self) -> Location {
        Location::Internal
    }
}

#[derive(Debug, Clone)]
pub struct CodeElementBuilder<N, D, T> {
    _name: PhantomData<N>,
    _description: PhantomData<D>,
    _code_type: PhantomData<T>,
    identifier: Option<ElementIdentifier>,
    name: Option<NonEmptyString>,
    description: Option<NonEmptyString>,
    code_type: Option<CodeType>,
    language: Option<NonEmptyString>,
    file_path: Option<NonEmptyString>,
}

impl
    CodeElementBuilder<
        code_element_builder::NoName,
        code_element_builder::NoDescription,
        code_element_builder::NoCodeType,
    >
{
    pub fn new() -> Self {
        CodeElementBuilder {
            _name: PhantomData,
            _description: PhantomData,
            _code_type: PhantomData,
            identifier: None,
            name: None,
            description: None,
            code_type: None,
            language: None,
            file_path: None,
        }
    }
}

impl<D, T> CodeElementBuilder<code_element_builder::NoName, D, T> {
    pub fn with_identifier(mut self, identifier: ElementIdentifier) -> Self {
        self.identifier = Some(identifier);
        self
    }

    pub fn with_name(
        self,
        name: NonEmptyString,
    ) -> CodeElementBuilder<code_element_builder::HasName, D, T> {
        CodeElementBuilder {
            _name: PhantomData,
            _description: self._description,
            _code_type: self._code_type,
            identifier: self.identifier,
            name: Some(name),
            description: self.description,
            code_type: self.code_type,
            language: self.language,
            file_path: self.file_path,
        }
    }
}

impl<N, T> CodeElementBuilder<N, code_element_builder::NoDescription, T> {
    pub fn with_description(
        self,
        description: NonEmptyString,
    ) -> CodeElementBuilder<N, code_element_builder::HasDescription, T> {
        CodeElementBuilder {
            _name: self._name,
            _description: PhantomData,
            _code_type: self._code_type,
            identifier: self.identifier,
            name: self.name,
            description: Some(description),
            code_type: self.code_type,
            language: self.language,
            file_path: self.file_path,
        }
    }
}

impl<N, D> CodeElementBuilder<N, D, code_element_builder::NoCodeType> {
    pub fn with_code_type(
        self,
        code_type: CodeType,
    ) -> CodeElementBuilder<N, D, code_element_builder::HasCodeType> {
        CodeElementBuilder {
            _name: self._name,
            _description: self._description,
            _code_type: PhantomData,
            identifier: self.identifier,
            name: self.name,
            description: self.description,
            code_type: Some(code_type),
            language: self.language,
            file_path: self.file_path,
        }
    }
}

impl<N, D, T> CodeElementBuilder<N, D, T> {
    pub fn with_language(mut self, language: NonEmptyString) -> Self {
        self.language = Some(language);
        self
    }

    pub fn with_file_path(mut self, file_path: NonEmptyString) -> Self {
        self.file_path = Some(file_path);
        self
    }
}

impl
    CodeElementBuilder<
        code_element_builder::HasName,
        code_element_builder::HasDescription,
        code_element_builder::HasCodeType,
    >
{
    pub fn build(self) -> CodeElement {
        if let Some(ref lang) = self.language
            && lang.len() > 255
        {
            panic!("language string exceeds maximum length of 255 characters");
        }
        if let Some(ref path) = self.file_path
            && path.len() > 512
        {
            panic!("file path exceeds maximum length of 512 characters");
        }
        CodeElement {
            identifier: self.identifier.unwrap_or_default(),
            name: self.name.unwrap(),
            description: self.description.unwrap(),
            code_type: self.code_type.unwrap(),
            language: self.language,
            file_path: self.file_path,
        }
    }
}

/// Error type for CodeElement construction.
#[derive(Debug, thiserror::Error)]
pub enum CodeElementError {
    #[error("code element identifier is required")]
    MissingIdentifier,

    #[error("code element name is required and cannot be empty")]
    MissingName,

    #[error("code element description is required and cannot be empty")]
    MissingDescription,

    #[error("code element type is required (e.g., Function, Class, Struct)")]
    MissingType,

    #[error("language string exceeds maximum length of {max} characters (actual: {actual})")]
    LanguageTooLong { max: usize, actual: usize },

    #[error("file path exceeds maximum length of {max} characters (actual: {actual})")]
    FilePathTooLong { max: usize, actual: usize },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_element_builder() {
        let code_element = CodeElement::builder()
            .with_name("calculateTotal".try_into().unwrap())
            .with_description("Calculates order total".try_into().unwrap())
            .with_code_type(CodeType::Function)
            .with_language("Rust".try_into().unwrap())
            .with_file_path("src/orders/calculator.rs".try_into().unwrap())
            .build();

        assert_eq!(code_element.name(), "calculateTotal");
        assert_eq!(code_element.code_type(), CodeType::Function);
        assert_eq!(code_element.language(), Some("Rust"));
    }
}
