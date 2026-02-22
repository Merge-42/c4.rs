use crate::styles::{ElementStyle, RelationshipStyle};
use crate::templates::view::{ElementStyleTemplate, RelationshipStyleTemplate};
use crate::writer;
use askama::Template;

#[derive(Debug, Default)]
pub struct StylesSerializer {
    element_styles: Vec<ElementStyle>,
    relationship_styles: Vec<RelationshipStyle>,
    external_output: Option<String>,
}

impl StylesSerializer {
    pub fn new() -> Self {
        Self {
            element_styles: Vec::new(),
            relationship_styles: Vec::new(),
            external_output: None,
        }
    }

    pub fn add_element_style(mut self, style: ElementStyle) -> Self {
        self.element_styles.push(style);
        self
    }

    pub fn add_relationship_style(mut self, style: RelationshipStyle) -> Self {
        self.relationship_styles.push(style);
        self
    }

    pub fn set_external_output(mut self, output: String) -> Self {
        self.external_output = Some(output);
        self
    }

    pub fn add_element_styles_from_string(mut self, dsl: &str) -> Self {
        self.external_output = Some(dsl.to_string());
        self
    }

    pub fn serialize(&self) -> Result<String, askama::Error> {
        if let Some(output) = writer::try_external_output(&self.external_output) {
            return Ok(output);
        }

        if self.element_styles.is_empty() && self.relationship_styles.is_empty() {
            return Ok(String::new());
        }

        let mut lines = Vec::new();
        lines.push("styles {".to_string());

        for style in &self.element_styles {
            let template = ElementStyleTemplate {
                identifier: &style.identifier,
                background: style.background.as_deref(),
                color: style.color.as_deref(),
                shape: style.shape.as_deref(),
                size: style.size.as_deref(),
                stroke: style.stroke.as_deref(),
                stroke_width: style.stroke_width.as_deref(),
            };
            lines.push(template.render()?);
        }

        for style in &self.relationship_styles {
            let dashed_str = style.dashed.map(|d| {
                if d {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            });
            let template = RelationshipStyleTemplate {
                identifier: &style.identifier,
                thickness: style.thickness.as_deref(),
                color: style.color.as_deref(),
                router: style.router.as_deref(),
                dashed: dashed_str.as_deref(),
            };
            lines.push(template.render()?);
        }

        lines.push("}".to_string());
        Ok(lines.join("\n"))
    }
}

#[cfg(test)]
#[path = "styles_serializer_tests.rs"]
mod tests;
