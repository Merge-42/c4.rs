//! DSL output formatting and writing.

use crate::serialization::traits::{escape_dsl_string, format_identifier};
use std::fmt::{Display, Formatter};

/// Writer for Structurizr DSL output.
#[derive(Debug, Default)]
pub struct DslWriter {
    lines: Vec<String>,
    indent_level: usize,
}

impl DslWriter {
    /// Create a new DSL writer.
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            indent_level: 0,
        }
    }

    /// Add a line to the output.
    pub fn add_line(&mut self, line: &str) {
        let indent = "    ".repeat(self.indent_level);
        self.lines.push(format!("{}{}", indent, line));
    }

    /// Add an empty line.
    pub fn add_empty_line(&mut self) {
        self.lines.push(String::new());
    }

    /// Increase indentation.
    pub fn indent(&mut self) {
        self.indent_level += 1;
    }

    /// Decrease indentation.
    pub fn unindent(&mut self) {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }
    }

    /// Write a block with opening and closing braces.
    pub fn write_block<F>(&mut self, name: &str, f: F)
    where
        F: FnOnce(&mut DslWriter),
    {
        self.add_line(&format!("{} {{", name));
        self.indent();
        f(self);
        self.unindent();
        self.add_line("}");
    }

    /// Convert to string.
    pub fn as_output(&self) -> String {
        self.lines.join("\n")
    }

    /// Clear the writer.
    pub fn clear(&mut self) {
        self.lines.clear();
        self.indent_level = 0;
    }
}

impl Display for DslWriter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_output())
    }
}

/// Format an element assignment for DSL output.
pub fn format_element_assignment(
    identifier: &str,
    element_type: &str,
    name: &str,
    description: &str,
    technology: Option<&str>,
) -> String {
    let identifier = format_identifier(identifier);
    let name = escape_dsl_string(name);
    let description = escape_dsl_string(description);

    if let Some(tech) = technology {
        let tech = escape_dsl_string(tech);
        format!(
            r#"{} = {} "{}" "{}" "{}""#,
            identifier, element_type, name, description, tech
        )
    } else {
        format!(
            r#"{} = {} "{}" "{}""#,
            identifier, element_type, name, description
        )
    }
}

/// Format a relationship for DSL output.
pub fn format_relationship(
    source: &str,
    target: &str,
    description: &str,
    technology: Option<&str>,
) -> String {
    let source = format_identifier(source);
    let target = format_identifier(target);
    let description = escape_dsl_string(description);

    if let Some(tech) = technology {
        let tech = escape_dsl_string(tech);
        format!(r#"{} -> {} "{}" "{}""#, source, target, description, tech)
    } else {
        format!(r#"{} -> {} "{}""#, source, target, description)
    }
}

/// Format a parent-child relationship for DSL output.
pub fn format_parent_reference(child: &str, parent: &str) -> String {
    let child = format_identifier(child);
    let parent = format_identifier(parent);
    format!("{} <- {}", child, parent)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_dsl_string() {
        assert_eq!(escape_dsl_string("hello"), "hello");
        assert_eq!(escape_dsl_string("hello\"world"), "hello\\\"world");
        assert_eq!(escape_dsl_string("hello\\world"), "hello\\\\world");
    }

    #[test]
    fn test_format_identifier() {
        assert_eq!(format_identifier("user"), "user");
        assert_eq!(format_identifier("my-system"), "my_system");
        assert_eq!(format_identifier("123abc"), "_123abc");
        assert_eq!(format_identifier("api-gateway"), "api_gateway");
    }

    #[test]
    fn test_format_element_assignment() {
        let result = format_element_assignment("user", "person", "User", "A system user", None);
        assert_eq!(result, r#"user = person "User" "A system user""#);

        let result =
            format_element_assignment("api", "softwareSystem", "API", "Backend API", Some("REST"));
        assert_eq!(result, r#"api = softwareSystem "API" "Backend API" "REST""#);
    }

    #[test]
    fn test_format_relationship() {
        let result = format_relationship("user", "api", "Uses", None);
        assert_eq!(result, r#"user -> api "Uses""#);

        let result = format_relationship("api", "db", "Reads from", Some("JDBC"));
        assert_eq!(result, r#"api -> db "Reads from" "JDBC""#);
    }
}
