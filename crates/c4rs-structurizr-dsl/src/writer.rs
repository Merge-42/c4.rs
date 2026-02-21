use crate::templates::helpers::{escape_dsl_string, format_identifier};
use std::fmt::{Display, Formatter};

#[derive(Debug, Default)]
pub struct DslWriter {
    lines: Vec<String>,
    indent_level: usize,
}

impl DslWriter {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            indent_level: 0,
        }
    }

    pub fn add_line(&mut self, line: &str) {
        let indent = "    ".repeat(self.indent_level);
        self.lines.push(format!("{}{}", indent, line));
    }

    pub fn add_empty_line(&mut self) {
        self.lines.push(String::new());
    }

    pub fn indent(&mut self) {
        self.indent_level += 1;
    }

    pub fn unindent(&mut self) {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }
    }

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

    pub fn as_output(&self) -> String {
        self.lines.join("\n")
    }

    pub fn clear(&mut self) {
        self.lines.clear();
        self.indent_level = 0;
    }

    /// Re-indents a DSL block based on brace depth.
    /// Useful for normalizing indentation of pre-rendered DSL fragments.
    pub fn indent_block(s: &str) -> String {
        let mut lines = Vec::new();
        let mut brace_depth = 0;
        for line in s.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                lines.push(line.to_string());
                continue;
            }
            let closing = trimmed.matches('}').count();
            let opening = trimmed.matches('{').count();
            if closing > 0 && brace_depth > 0 {
                brace_depth = (brace_depth as i32 - closing as i32).max(0) as usize;
            }
            let indent = "    ".repeat(brace_depth);
            lines.push(format!("{}{}", indent, trimmed));
            if opening > 0 {
                brace_depth += opening;
            }
        }
        lines.join("\n")
    }
}

impl Display for DslWriter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_output())
    }
}

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
