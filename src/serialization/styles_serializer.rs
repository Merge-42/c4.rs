//! Styles serialization for Structurizr DSL.

use crate::c4::ElementType;
use crate::serialization::writer::DslWriter;
use serde::{Deserialize, Serialize};

/// Represents a style for elements in Structurizr DSL.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementStyle {
    pub identifier: String,
    pub element_type: ElementType,
    pub background: Option<String>,
    pub color: Option<String>,
    pub shape: Option<String>,
    pub size: Option<String>,
    pub stroke: Option<String>,
    pub stroke_width: Option<String>,
}

impl ElementStyle {
    /// Create a new element style.
    pub fn new(identifier: &str, element_type: ElementType) -> Self {
        Self {
            identifier: identifier.to_string(),
            element_type,
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

        let mut writer = DslWriter::new();
        writer.add_line("styles {");
        writer.indent();

        for style in &self.element_styles {
            writer.add_line(&format!(
                "    {} {{",
                style.element_type.to_string().to_lowercase()
            ));
            writer.indent();

            if let Some(bg) = &style.background {
                writer.add_line(&format!("        background {}", bg));
            }
            if let Some(color) = &style.color {
                writer.add_line(&format!("        color {}", color));
            }
            if let Some(shape) = &style.shape {
                writer.add_line(&format!("        shape {}", shape));
            }
            if let Some(size) = &style.size {
                writer.add_line(&format!("        size {}", size));
            }
            if let Some(stroke) = &style.stroke {
                writer.add_line(&format!("        stroke {}", stroke));
            }
            if let Some(stroke_width) = &style.stroke_width {
                writer.add_line(&format!("        strokeWidth {}", stroke_width));
            }

            writer.unindent();
            writer.add_line("    }");
        }

        for style in &self.relationship_styles {
            writer.add_line("    relationship {");
            writer.indent();

            if let Some(thickness) = &style.thickness {
                writer.add_line(&format!("        thickness {}", thickness));
            }
            if let Some(color) = &style.color {
                writer.add_line(&format!("        color {}", color));
            }
            if let Some(router) = &style.router {
                writer.add_line(&format!("        router {}", router));
            }
            if let Some(dashed) = &style.dashed {
                writer.add_line(&format!(
                    "        dashed {}",
                    if *dashed { "true" } else { "false" }
                ));
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

    #[test]
    fn test_element_style() {
        let mut styles = StylesSerializer::new();
        styles.add_element_style(
            ElementStyle::new("person", ElementType::Person)
                .with_background("#ffcc00")
                .with_color("#000000")
                .with_shape("Person"),
        );

        let dsl = styles.serialize();
        assert!(dsl.contains("styles {"));
        assert!(dsl.contains("person {"));
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
            ElementStyle::new("container", ElementType::Container)
                .with_background("#ffffff")
                .with_shape("Rectangle"),
        );

        let dsl = styles.serialize();
        assert!(dsl.contains("container {"));
        assert!(dsl.contains("shape Rectangle"));
    }
}
