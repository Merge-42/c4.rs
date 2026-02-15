//! Styles serialization for Structurizr DSL.

use crate::serialization::templates::view::{ElementStyleTemplate, RelationshipStyleTemplate};
use askama::Template;
use serde::{Deserialize, Serialize};

/// Represents a style for elements in Structurizr DSL.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementStyle {
    pub identifier: String,
    pub background: Option<String>,
    pub color: Option<String>,
    pub shape: Option<String>,
    pub size: Option<String>,
    pub stroke: Option<String>,
    pub stroke_width: Option<String>,
}

impl ElementStyle {
    /// Create a new element style.
    pub fn new(identifier: &str) -> Self {
        Self {
            identifier: identifier.to_string(),
            background: None,
            color: None,
            shape: None,
            size: None,
            stroke: None,
            stroke_width: None,
        }
    }

    /// Set the background color.
    pub fn with_background(mut self, color: &str) -> Self {
        self.background = Some(color.to_string());
        self
    }

    /// Set the text color.
    pub fn with_color(mut self, color: &str) -> Self {
        self.color = Some(color.to_string());
        self
    }

    /// Set the shape (Rectangle, Circle, Ellipse, etc.)
    pub fn with_shape(mut self, shape: &str) -> Self {
        self.shape = Some(shape.to_string());
        self
    }

    /// Set the size (small, medium, large).
    pub fn with_size(mut self, size: &str) -> Self {
        self.size = Some(size.to_string());
        self
    }

    /// Set the border color.
    pub fn with_stroke(mut self, color: &str) -> Self {
        self.stroke = Some(color.to_string());
        self
    }

    /// Set the border width.
    pub fn with_stroke_width(mut self, width: &str) -> Self {
        self.stroke_width = Some(width.to_string());
        self
    }
}

/// Represents a relationship style in Structurizr DSL.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RelationshipStyle {
    pub thickness: Option<String>,
    pub color: Option<String>,
    pub router: Option<String>,
    pub dashed: Option<bool>,
}

impl RelationshipStyle {
    /// Create a new relationship style.
    pub fn new() -> Self {
        Self {
            thickness: None,
            color: None,
            router: None,
            dashed: None,
        }
    }

    /// Set the line thickness.
    pub fn with_thickness(mut self, thickness: &str) -> Self {
        self.thickness = Some(thickness.to_string());
        self
    }

    /// Set the line color.
    pub fn with_color(mut self, color: &str) -> Self {
        self.color = Some(color.to_string());
        self
    }

    /// Set the router type (Direct, Orthogonal, curvilinear).
    pub fn with_router(mut self, router: &str) -> Self {
        self.router = Some(router.to_string());
        self
    }

    /// Set whether the line is dashed.
    pub fn with_dashed(mut self, dashed: bool) -> Self {
        self.dashed = Some(dashed);
        self
    }
}

/// Serializes Structurizr styles to DSL format.
#[derive(Debug, Default)]
pub struct StylesSerializer {
    element_styles: Vec<ElementStyle>,
    relationship_styles: Vec<RelationshipStyle>,
    external_output: Option<String>,
}

impl StylesSerializer {
    /// Create a new styles serializer.
    pub fn new() -> Self {
        Self {
            element_styles: Vec::new(),
            relationship_styles: Vec::new(),
            external_output: None,
        }
    }

    /// Add an element style.
    pub fn add_element_style(&mut self, style: ElementStyle) {
        self.element_styles.push(style);
    }

    /// Add a relationship style.
    pub fn add_relationship_style(&mut self, style: RelationshipStyle) {
        self.relationship_styles.push(style);
    }

    /// Set external pre-serialized output (for integration with WorkspaceSerializer).
    pub fn set_external_output(&mut self, output: String) {
        self.external_output = Some(output);
    }

    /// Add element styles from serialized DSL string.
    pub fn add_element_styles_from_string(&mut self, dsl: &str) {
        self.external_output = Some(dsl.to_string());
    }

    /// Serialize styles to DSL format.
    pub fn serialize(&self) -> String {
        if let Some(ref output) = self.external_output
            && !output.is_empty()
        {
            return output.clone();
        }

        if self.element_styles.is_empty() && self.relationship_styles.is_empty() {
            return String::new();
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
            lines.push(template.render().unwrap());
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
                thickness: style.thickness.as_deref(),
                color: style.color.as_deref(),
                router: style.router.as_deref(),
                dashed: dashed_str.as_deref(),
            };
            lines.push(template.render().unwrap());
        }

        lines.push("}".to_string());
        lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_style() {
        let mut styles = StylesSerializer::new();
        styles.add_element_style(
            ElementStyle::new("Person")
                .with_background("#ffcc00")
                .with_color("#000000")
                .with_shape("Person"),
        );

        let dsl = styles.serialize();
        assert!(dsl.contains("styles {"));
        assert!(dsl.contains(r#"element "Person""#));
        assert!(dsl.contains("background #ffcc00"));
        assert!(dsl.contains("shape Person"));
    }

    #[test]
    fn test_relationship_style() {
        let mut styles = StylesSerializer::new();
        styles.add_relationship_style(
            RelationshipStyle::new()
                .with_thickness("2")
                .with_color("#999999")
                .with_router("curvilinear")
                .with_dashed(true),
        );

        let dsl = styles.serialize();
        assert!(dsl.contains("relationship {"));
        assert!(dsl.contains("thickness 2"));
        assert!(dsl.contains("dashed true"));
    }

    #[test]
    fn test_empty_styles() {
        let styles = StylesSerializer::new();
        let dsl = styles.serialize();
        assert!(dsl.is_empty());
    }

    #[test]
    fn test_container_style() {
        let mut styles = StylesSerializer::new();
        styles.add_element_style(
            ElementStyle::new("Database")
                .with_background("#ffffff")
                .with_shape("cylinder"),
        );

        let dsl = styles.serialize();
        assert!(dsl.contains(r#"element "Database""#));
        assert!(dsl.contains("shape cylinder"));
    }

    #[test]
    fn test_us5_element_styles_from_spec() {
        let mut styles = StylesSerializer::new();
        styles.add_element_style(
            ElementStyle::new("Element")
                .with_color("#9a28f8")
                .with_stroke("#9a28f8")
                .with_stroke_width("7")
                .with_shape("roundedbox"),
        );
        styles.add_element_style(ElementStyle::new("Person").with_shape("person"));
        styles.add_element_style(ElementStyle::new("Database").with_shape("cylinder"));
        styles.add_element_style(ElementStyle::new("Boundary").with_stroke_width("5"));

        let dsl = styles.serialize();
        assert!(dsl.contains(r#"element "Element""#));
        assert!(dsl.contains("color #9a28f8"));
        assert!(dsl.contains("stroke #9a28f8"));
        assert!(dsl.contains("strokeWidth 7"));
        assert!(dsl.contains("shape roundedbox"));
        assert!(dsl.contains(r#"element "Person""#));
        assert!(dsl.contains("shape person"));
        assert!(dsl.contains(r#"element "Database""#));
        assert!(dsl.contains("shape cylinder"));
        assert!(dsl.contains(r#"element "Boundary""#));
        assert!(dsl.contains("strokeWidth 5"));
    }

    #[test]
    fn test_us5_relationship_style_from_spec() {
        let mut styles = StylesSerializer::new();
        styles.add_relationship_style(RelationshipStyle::new().with_thickness("4"));

        let dsl = styles.serialize();
        assert!(dsl.contains("relationship {"));
        assert!(dsl.contains("thickness 4"));
    }
}
