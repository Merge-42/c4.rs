use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("{field} cannot be empty")]
    Empty { field: String },
    #[error("{field} exceeds maximum length of {max} characters (actual: {actual})")]
    TooLong {
        field: String,
        max: usize,
        actual: usize,
    },
}

/// Trait for types that can be converted to `Option<&str>` for validation.
///
/// Implemented for `&str`, `&String`, `Option<&str>`, and `&Option<String>`,
/// allowing `validate_max_length` to accept any of these without manual conversion.
pub trait AsOptionalStr<'a> {
    fn to_optional_str(self) -> Option<&'a str>;
}

impl<'a> AsOptionalStr<'a> for &'a str {
    fn to_optional_str(self) -> Option<&'a str> {
        Some(self)
    }
}

impl<'a> AsOptionalStr<'a> for &'a String {
    fn to_optional_str(self) -> Option<&'a str> {
        Some(self.as_str())
    }
}

impl<'a> AsOptionalStr<'a> for Option<&'a str> {
    fn to_optional_str(self) -> Option<&'a str> {
        self
    }
}

impl<'a> AsOptionalStr<'a> for &'a Option<String> {
    fn to_optional_str(self) -> Option<&'a str> {
        self.as_deref()
    }
}

pub fn validate_non_empty(value: &str, field: &str) -> Result<(), ValidationError> {
    if value.trim().is_empty() {
        Err(ValidationError::Empty {
            field: field.to_string(),
        })
    } else {
        Ok(())
    }
}

/// Validates max length for both required and optional string fields.
///
/// Accepts `&str`, `&String`, `Option<&str>`, and `&Option<String>`.
/// For `None` values, validation passes (field is absent).
pub fn validate_max_length<'a>(
    value: impl AsOptionalStr<'a>,
    max: usize,
    field: &str,
) -> Result<(), ValidationError> {
    if let Some(v) = value.to_optional_str()
        && v.len() > max
    {
        return Err(ValidationError::TooLong {
            field: field.to_string(),
            max,
            actual: v.len(),
        });
    }
    Ok(())
}

pub fn validate_vec_max_length(
    values: &[String],
    max: usize,
    field: &str,
) -> Result<(), ValidationError> {
    for (i, v) in values.iter().enumerate() {
        if v.len() > max {
            return Err(ValidationError::TooLong {
                field: format!("{}[{}]", field, i),
                max,
                actual: v.len(),
            });
        }
    }
    Ok(())
}
