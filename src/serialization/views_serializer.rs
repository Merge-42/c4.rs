//! Views serialization for Structurizr DSL.

use crate::serialization::templates::view::ViewTemplate;
use askama::Template;

/// Represents a view type in Structurizr DSL.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ViewType {
    #[default]
    SystemContext,
    Container,
    Component,
    SystemLandscape,
    Filtered,
    Dynamic,
    Deployment,
    Custom,
}

impl std::fmt::Display for ViewType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ViewType::SystemContext => write!(f, "systemContext"),
            ViewType::Container => write!(f, "container"),
            ViewType::Component => write!(f, "component"),
            ViewType::SystemLandscape => write!(f, "systemLandscape"),
            ViewType::Filtered => write!(f, "filtered"),
            ViewType::Dynamic => write!(f, "dynamic"),
            ViewType::Deployment => write!(f, "deployment"),
            ViewType::Custom => write!(f, "custom"),
        }
    }
}

/// Represents a Structurizr view configuration.
#[derive(Debug, Clone)]
pub struct ViewConfiguration {
    pub view_type: ViewType,
    pub element_identifier: String,
    pub title: String,
    pub include_elements: Vec<String>,
    pub exclude_elements: Vec<String>,
}

impl ViewConfiguration {
    /// Create a new view configuration.
    pub fn new(view_type: ViewType, element_identifier: &str, title: &str) -> Self {
        Self {
            view_type,
            element_identifier: element_identifier.to_string(),
            title: title.to_string(),
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
    styles_output: Option<String>,
    configuration_output: Option<String>,
}

impl ViewsSerializer {
    /// Create a new views serializer.
    pub fn new() -> Self {
        Self {
            views: Vec::new(),
            external_output: None,
            styles_output: None,
            configuration_output: None,
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

    /// Set styles output to be included inside views.
    pub fn set_styles_output(&mut self, output: String) {
        self.styles_output = Some(output);
    }

    /// Set configuration output to be included inside views.
    pub fn set_configuration_output(&mut self, output: String) {
        self.configuration_output = Some(output);
    }

    /// Check if styles output is set.
    pub fn styles_output(&self) -> Option<&String> {
        self.styles_output.as_ref()
    }

    /// Check if configuration output is set.
    pub fn configuration_output(&self) -> Option<&String> {
        self.configuration_output.as_ref()
    }

    /// Serialize all views to DSL format.
    pub fn serialize(&self) -> String {
        if let Some(ref output) = self.external_output
            && !output.is_empty()
        {
            return output.clone();
        }

        if self.views.is_empty()
            && self.styles_output.is_none()
            && self.configuration_output.is_none()
        {
            return String::new();
        }

        let mut lines = Vec::new();
        lines.push("views {".to_string());

        for view in &self.views {
            let include_refs: Vec<&str> =
                view.include_elements.iter().map(|s| s.as_str()).collect();
            let exclude_refs: Vec<&str> =
                view.exclude_elements.iter().map(|s| s.as_str()).collect();

            let template = ViewTemplate {
                view_type: &view.view_type.to_string(),
                identifier: &view.element_identifier,
                title: &view.title,
                include_elements: &include_refs,
                exclude_elements: &exclude_refs,
            };
            lines.push(template.render().unwrap());
        }

        if let Some(ref styles) = self.styles_output {
            lines.push(String::new());
            for line in styles.lines() {
                lines.push(line.to_string());
            }
        }

        if let Some(ref config) = self.configuration_output {
            lines.push(String::new());
            for line in config.lines() {
                lines.push(line.to_string());
            }
        }

        lines.push("}".to_string());
        lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_context_view() {
        let mut views = ViewsSerializer::new();
        let mut view = ViewConfiguration::new(ViewType::SystemContext, "a", "System Context");
        view.include_element("*");
        views.add_view(view);

        let dsl = views.serialize();
        assert!(dsl.contains("views {"));
        assert!(dsl.contains("systemContext a \"System Context\" {"));
        assert!(dsl.contains("include *"));
    }

    #[test]
    fn test_container_view() {
        let mut views = ViewsSerializer::new();
        let mut view = ViewConfiguration::new(ViewType::Container, "api", "Container Diagram");
        view.include_element("Web_App");
        view.include_element("API");
        view.exclude_element("Database");
        views.add_view(view);

        let dsl = views.serialize();
        assert!(dsl.contains("container api \"Container Diagram\" {"));
        assert!(dsl.contains("exclude Database"));
    }

    #[test]
    fn test_empty_views() {
        let views = ViewsSerializer::new();
        let dsl = views.serialize();
        assert!(dsl.is_empty());
    }
}
