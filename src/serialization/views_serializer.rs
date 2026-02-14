//! Views serialization for Structurizr DSL.

use crate::c4::ElementType;
use crate::serialization::writer::DslWriter;

/// Represents a Structurizr view configuration.
#[derive(Debug, Clone)]
pub struct ViewConfiguration {
    pub name: String,
    pub title: String,
    pub element_type: ElementType,
    pub include_elements: Vec<String>,
    pub exclude_elements: Vec<String>,
}

impl ViewConfiguration {
    /// Create a new view configuration.
    pub fn new(name: &str, title: &str, element_type: ElementType) -> Self {
        Self {
            name: name.to_string(),
            title: title.to_string(),
            element_type,
            include_elements: Vec::new(),
            exclude_elements: Vec::new(),
        }
    }

    /// Add an element to include in the view.
    pub fn include_element(&mut self, identifier: &str) {
        self.include_elements.push(identifier.to_string());
    }

    /// Add an element to exclude from the view.
    pub fn exclude_element(&mut self, identifier: &str) {
        self.exclude_elements.push(identifier.to_string());
    }
}

/// Serializes Structurizr views to DSL format.
#[derive(Debug, Default)]
pub struct ViewsSerializer {
    views: Vec<ViewConfiguration>,
    external_output: Option<String>,
}

impl ViewsSerializer {
    /// Create a new views serializer.
    pub fn new() -> Self {
        Self {
            views: Vec::new(),
            external_output: None,
        }
    }

    /// Add a view configuration.
    pub fn add_view(&mut self, view: ViewConfiguration) {
        self.views.push(view);
    }

    /// Set external pre-serialized output (for integration with WorkspaceSerializer).
    pub fn set_external_output(&mut self, output: String) {
        self.external_output = Some(output);
    }

    /// Serialize all views to DSL format.
    pub fn serialize(&self) -> String {
        if let Some(ref output) = self.external_output
            && !output.is_empty()
        {
            return output.clone();
        }

        if self.views.is_empty() {
            return String::new();
        }

        let mut writer = DslWriter::new();
        writer.add_line("views {");
        writer.indent();

        for view in &self.views {
            writer.add_line(&format!(
                "    {} {} {{",
                view.element_type.to_string().to_lowercase(),
                view.name
            ));
            writer.indent();
            writer.add_line(&format!("        title \"{}\"", view.title));

            for element in &view.include_elements {
                writer.add_line(&format!("            include {}", element));
            }

            for element in &view.exclude_elements {
                writer.add_line(&format!("            exclude {}", element));
            }

            writer.unindent();
            writer.add_line("    }");
        }

        writer.unindent();
        writer.add_line("}");

        writer.as_output()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::c4::ElementType;

    #[test]
    fn test_system_context_view() {
        let mut views = ViewsSerializer::new();
        let mut view =
            ViewConfiguration::new("context", "System Context", ElementType::SoftwareSystem);
        view.include_element("User");
        view.include_element("API");
        views.add_view(view);

        let dsl = views.serialize();
        assert!(dsl.contains("views {"));
        assert!(dsl.contains("softwaresystem context {"));
        assert!(dsl.contains("include User"));
    }

    #[test]
    fn test_container_view() {
        let mut views = ViewsSerializer::new();
        let mut view =
            ViewConfiguration::new("containers", "Container Diagram", ElementType::Container);
        view.include_element("Web_App");
        view.include_element("API");
        view.exclude_element("Database");
        views.add_view(view);

        let dsl = views.serialize();
        assert!(dsl.contains("container containers {"));
        assert!(dsl.contains("exclude Database"));
    }

    #[test]
    fn test_empty_views() {
        let views = ViewsSerializer::new();
        let dsl = views.serialize();
        assert!(dsl.is_empty());
    }
}
