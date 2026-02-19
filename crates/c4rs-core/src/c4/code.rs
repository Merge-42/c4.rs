use bon::Builder;
use serde::{Deserialize, Serialize};

use super::element::{CodeType, ElementType};

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

super::macros::impl_element!(CodeElement, ElementType::Code);

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
        let c = CodeElement::builder()
            .name("calc".into())
            .description("Calculates".into())
            .code_type(CodeType::Function)
            .build()
            .unwrap();
        assert_eq!(c.name(), "calc");
    }
}
