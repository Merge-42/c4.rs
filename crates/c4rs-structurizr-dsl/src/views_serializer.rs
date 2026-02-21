use crate::error::DslError;
use crate::templates::view::ViewTemplate;
use crate::writer;
use askama::Template;
use bon::Builder;

#[non_exhaustive]
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

#[derive(Debug, Clone, Builder)]
pub struct ViewConfiguration {
    pub view_type: ViewType,
    pub element_identifier: String,
    pub title: String,
    #[builder(default)]
    pub include_elements: Vec<String>,
    #[builder(default)]
    pub exclude_elements: Vec<String>,
}

#[derive(Debug, Default, Builder)]
pub struct ViewsSerializer {
    #[builder(default)]
    views: Vec<ViewConfiguration>,
    external_output: Option<String>,
    styles_output: Option<String>,
    configuration_output: Option<String>,
}

impl ViewsSerializer {
    pub fn add_view(&mut self, view: ViewConfiguration) {
        self.views.push(view);
    }

    pub fn set_external_output(&mut self, output: String) {
        self.external_output = Some(output);
    }

    pub fn set_styles_output(&mut self, output: String) {
        self.styles_output = Some(output);
    }

    pub fn set_configuration_output(&mut self, output: String) {
        self.configuration_output = Some(output);
    }

    pub fn styles_output(&self) -> Option<&String> {
        self.styles_output.as_ref()
    }

    pub fn configuration_output(&self) -> Option<&String> {
        self.configuration_output.as_ref()
    }

    pub fn serialize(&self) -> Result<String, DslError> {
        if let Some(output) = writer::try_external_output(&self.external_output) {
            return Ok(output);
        }

        if self.views.is_empty()
            && self.styles_output.is_none()
            && self.configuration_output.is_none()
        {
            return Ok(String::new());
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
            lines.push(template.render()?);
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
        Ok(lines.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_context_view() {
        let mut views = ViewsSerializer::builder().build();
        let view = ViewConfiguration::builder()
            .view_type(ViewType::SystemContext)
            .element_identifier("a".to_string())
            .title("System Context".to_string())
            .include_elements(vec!["*".to_string()])
            .build();
        views.add_view(view);

        let dsl = views.serialize().unwrap();
        assert!(dsl.contains("views {"));
        assert!(dsl.contains("systemContext a \"System Context\" {"));
        assert!(dsl.contains("include *"));
    }

    #[test]
    fn test_container_view() {
        let mut views = ViewsSerializer::builder().build();
        let view = ViewConfiguration::builder()
            .view_type(ViewType::Container)
            .element_identifier("api".to_string())
            .title("Container Diagram".to_string())
            .include_elements(vec!["Web_App".to_string(), "API".to_string()])
            .exclude_elements(vec!["Database".to_string()])
            .build();
        views.add_view(view);

        let dsl = views.serialize().unwrap();
        assert!(dsl.contains("container api \"Container Diagram\" {"));
        assert!(dsl.contains("exclude Database"));
    }

    #[test]
    fn test_empty_views() {
        let views = ViewsSerializer::builder().build();
        let dsl = views.serialize().unwrap();
        assert!(dsl.is_empty());
    }
}
