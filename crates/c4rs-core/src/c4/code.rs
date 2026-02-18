use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::element::{CodeType, Element, ElementType, Location};
use super::value_types::{ElementIdentifier, NonEmptyString};

/// Represents an individual code unit within a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
pub struct CodeElement {
    #[serde(skip)]
    #[builder(default)]
    identifier: Option<ElementIdentifier>,
    name: NonEmptyString,
    description: NonEmptyString,
    code_type: CodeType,
    #[builder(default, setter(strip_option))]
    language: Option<NonEmptyString>,
    #[builder(default, setter(strip_option))]
    file_path: Option<NonEmptyString>,
}

impl CodeElement {
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

    pub fn code_type(&self) -> CodeType {
        self.code_type.clone()
    }

    pub fn language(&self) -> Option<&str> {
        self.language.as_deref()
    }

    pub fn file_path(&self) -> Option<&str> {
        self.file_path.as_deref()
    }

    pub fn build(self) -> Result<CodeElement, CodeElementError> {
        if let Some(ref lang) = self.language
            && lang.len() > 255
        {
            return Err(CodeElementError::LanguageTooLong {
                max: 255,
                actual: lang.len(),
            });
        }
        if let Some(ref path) = self.file_path
            && path.len() > 512
        {
            return Err(CodeElementError::FilePathTooLong {
                max: 512,
                actual: path.len(),
            });
        }
        Ok(CodeElement {
            identifier: self.identifier,
            name: self.name,
            description: self.description,
            code_type: self.code_type,
            language: self.language,
            file_path: self.file_path,
        })
    }
}

impl Element for CodeElement {
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
        ElementType::Code
    }

    fn location(&self) -> Location {
        Location::Internal
    }
}

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
            .name("calculateTotal".into())
            .description("Calculates order total".into())
            .code_type(CodeType::Function)
            .language("Rust".into())
            .file_path("src/orders/calculator.rs".into())
            .build();

        assert_eq!(code_element.name(), "calculateTotal");
        assert_eq!(code_element.code_type(), CodeType::Function);
        assert_eq!(code_element.language(), Some("Rust"));
    }
}
