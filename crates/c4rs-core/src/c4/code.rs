use super::element::{CodeType, ElementId, ElementType};
use crate::constants::limits::{
    MAX_DESCRIPTION_LENGTH, MAX_FILE_PATH_LENGTH, MAX_LANGUAGE_LENGTH, MAX_NAME_LENGTH,
};
use crate::validation::{validate_max_length, validate_non_empty};
use bon::Builder;

#[derive(Debug, Clone, PartialEq, Eq, Builder)]
#[builder(finish_fn(vis = "", name = build_internal))]
pub struct CodeElement {
    name: String,
    description: String,
    #[builder(skip = ElementId::from_name(&name))]
    id: ElementId,
    code_type: CodeType,
    language: Option<String>,
    file_path: Option<String>,
}

impl<S: code_element_builder::IsComplete> CodeElementBuilder<S> {
    pub fn build(self) -> Result<CodeElement, CodeElementError> {
        let code_element = self.build_internal();
        validate_non_empty(&code_element.name, "name")?;
        validate_max_length(&code_element.name, MAX_NAME_LENGTH, "name")?;
        validate_non_empty(&code_element.description, "description")?;
        validate_max_length(
            &code_element.description,
            MAX_DESCRIPTION_LENGTH,
            "description",
        )?;
        validate_max_length(&code_element.language, MAX_LANGUAGE_LENGTH, "language")?;
        validate_max_length(&code_element.file_path, MAX_FILE_PATH_LENGTH, "file_path")?;
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
    #[error("file_path string exceeds maximum length of {max} characters (actual: {actual})")]
    FilePathTooLong { max: usize, actual: usize },
    #[error("validation error: {0}")]
    Validation(#[from] crate::validation::ValidationError),
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
        assert_eq!(c.id().as_str(), "c");
    }
}
