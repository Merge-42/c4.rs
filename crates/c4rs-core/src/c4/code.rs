use bon::Builder;
use serde::{Deserialize, Serialize};

use super::element::{CodeType, Element, ElementType, Location};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Builder)]
#[builder(finish_fn(vis = "", name = build_internal))]
pub struct CodeElement {
    name: String,
    description: String,
    code_type: CodeType,
    language: Option<String>,
    file_path: Option<String>,
}

impl<S: code_element_builder::IsComplete> CodeElementBuilder<S> {
    pub fn build(self) -> Result<CodeElement, CodeElementError> {
        let code_element = self.build_internal();

        if code_element.name.trim().is_empty() {
            return Err(CodeElementError::MissingName);
        }
        if code_element.description.trim().is_empty() {
            return Err(CodeElementError::MissingDescription);
        }

        Ok(code_element)
    }
}

impl CodeElement {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
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
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
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
    #[error("code element name is required and cannot be empty")]
    MissingName,

    #[error("code element description is required and cannot be empty")]
    MissingDescription,
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
            .build()
            .unwrap();

        assert_eq!(code_element.name(), "calculateTotal");
        assert_eq!(code_element.code_type(), CodeType::Function);
        assert_eq!(code_element.language(), Some("Rust"));
    }
}
