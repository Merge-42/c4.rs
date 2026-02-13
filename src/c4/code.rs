use serde::{Deserialize, Serialize};

use super::element::{CodeType, Element, ElementType, Location};
use super::value_types::{ElementIdentifier, NonEmptyString};

/// Represents an individual code unit within a component.
///
/// CodeElements are the lowest level of the C4 model, representing
/// classes, functions, modules, or other code constructs.
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
    /// Creates a new CodeElementBuilder.
    pub fn builder() -> CodeElementBuilder {
        CodeElementBuilder::new()
    }

    /// Returns a reference to the code element's unique identifier.
    pub fn identifier(&self) -> &ElementIdentifier {
        &self.identifier
    }

    /// Returns the code element's name.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the code element's description.
    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    /// Returns the code element's type.
    pub fn code_type(&self) -> CodeType {
        self.code_type.clone()
    }

    /// Returns the programming language.
    pub fn language(&self) -> Option<&str> {
        self.language.as_deref()
    }

    /// Returns the file path where this code is located.
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

/// Builder for constructing CodeElement instances.
#[derive(Debug, Clone, Default)]
pub struct CodeElementBuilder {
    identifier: Option<ElementIdentifier>,
    name: Option<NonEmptyString>,
    description: Option<NonEmptyString>,
    code_type: Option<CodeType>,
    language: Option<NonEmptyString>,
    file_path: Option<NonEmptyString>,
}

impl CodeElementBuilder {
    /// Creates a new CodeElementBuilder.
    pub fn new() -> Self {
        Self {
            identifier: None,
            name: None,
            description: None,
            code_type: None,
            language: None,
            file_path: None,
        }
    }

    /// Sets the element identifier.
    pub fn with_identifier(mut self, identifier: ElementIdentifier) -> Self {
        self.identifier = Some(identifier);
        self
    }

    /// Sets the code element's name.
    pub fn with_name(mut self, name: NonEmptyString) -> Self {
        self.name = Some(name);
        self
    }

    /// Sets the code element's description.
    pub fn with_description(mut self, description: NonEmptyString) -> Self {
        self.description = Some(description);
        self
    }

    /// Sets the code element's type.
    pub fn with_code_type(mut self, code_type: CodeType) -> Self {
        self.code_type = Some(code_type);
        self
    }

    /// Sets the programming language.
    pub fn with_language(mut self, language: NonEmptyString) -> Self {
        self.language = Some(language);
        self
    }

    /// Sets the file path.
    pub fn with_file_path(mut self, file_path: NonEmptyString) -> Self {
        self.file_path = Some(file_path);
        self
    }

    /// Builds the CodeElement.
    pub fn build(self) -> Result<CodeElement, CodeElementError> {
        let identifier = self.identifier.unwrap_or_else(ElementIdentifier::new);
        let name = self.name.ok_or(CodeElementError::MissingName)?;
        let description = self
            .description
            .ok_or(CodeElementError::MissingDescription)?;
        let code_type = self.code_type.ok_or(CodeElementError::MissingType)?;

        if let Some(ref lang) = self.language {
            if lang.len() > 255 {
                return Err(CodeElementError::LanguageTooLong {
                    max: 255,
                    actual: lang.len(),
                });
            }
        }

        if let Some(ref path) = self.file_path {
            if path.len() > 512 {
                return Err(CodeElementError::FilePathTooLong {
                    max: 512,
                    actual: path.len(),
                });
            }
        }

        Ok(CodeElement {
            identifier,
            name,
            description,
            code_type,
            language: self.language,
            file_path: self.file_path,
        })
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
            .build()
            .unwrap();

        assert_eq!(code_element.name(), "calculateTotal");
        assert_eq!(code_element.code_type(), CodeType::Function);
        assert_eq!(code_element.language(), Some("Rust"));
    }

    #[test]
    fn test_code_element_error_missing_type() {
        let result = CodeElement::builder()
            .with_name("myFunction".try_into().unwrap())
            .with_description("Missing type".try_into().unwrap())
            .build();

        assert!(result.is_err());
    }
}
