use crate::styles::{ElementStyle, RelationshipStyle};
use crate::templates::view::{ElementStyleTemplate, RelationshipStyleTemplate};
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

    pub fn add_element_style(&mut self, style: ElementStyle) {
        self.element_styles.push(style);
    }

    pub fn add_relationship_style(&mut self, style: RelationshipStyle) {
        self.relationship_styles.push(style);
    }

    pub fn set_external_output(&mut self, output: String) {
        self.external_output = Some(output);
    }

    pub fn add_element_styles_from_string(&mut self, dsl: &str) {
        self.external_output = Some(dsl.to_string());
    }

    pub fn serialize(&self) -> Result<String, askama::Error> {
        if let Some(ref output) = self.external_output
            && !output.is_empty()
        {
            return Ok(output.clone());
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
mod tests {
    use super::*;

    #[test]
    fn test_element_style() {
        let mut styles = StylesSerializer::new();
        styles.add_element_style(
            ElementStyle::builder()
                .identifier("Person".into())
                .background("ffcc00".into())
                .color("#000000".into())
                .shape("Person".into())
                .build(),
        );

        let dsl = styles.serialize().unwrap();
        assert!(dsl.contains("styles {"));
        assert!(dsl.contains(r#"element "Person""#));
        assert!(dsl.contains("background ffcc00"));
        assert!(dsl.contains("shape Person"));
    }

    #[test]
    fn test_relationship_style() {
        let mut styles = StylesSerializer::new();
        styles.add_relationship_style(
            RelationshipStyle::builder()
                .thickness("2".into())
                .color("#999999".into())
                .router("curvilinear".into())
                .dashed(true)
                .build(),
        );

        let dsl = styles.serialize().unwrap();
        assert!(dsl.contains("relationship {"));
        assert!(dsl.contains("thickness 2"));
        assert!(dsl.contains("dashed true"));
    }

    #[test]
    fn test_empty_styles() {
        let styles = StylesSerializer::new();
        let dsl = styles.serialize().unwrap();
        assert!(dsl.is_empty());
    }

    #[test]
    fn test_container_style() {
        let mut styles = StylesSerializer::new();
        styles.add_element_style(
            ElementStyle::builder()
                .identifier("Database".into())
                .background("#ffffff".into())
                .shape("cylinder".into())
                .build(),
        );

        let dsl = styles.serialize().unwrap();
        assert!(dsl.contains(r#"element "Database""#));
        assert!(dsl.contains("shape cylinder"));
    }

    #[test]
    fn test_us5_element_styles_from_spec() {
        let mut styles = StylesSerializer::new();
        styles.add_element_style(
            ElementStyle::builder()
                .identifier("Element".into())
                .color("#9a28f8".into())
                .stroke("#9a28f8".into())
                .stroke_width("7".into())
                .shape("roundedbox".into())
                .build(),
        );
        styles.add_element_style(
            ElementStyle::builder()
                .identifier("Person".into())
                .shape("person".into())
                .build(),
        );
        styles.add_element_style(
            ElementStyle::builder()
                .identifier("Database".into())
                .shape("cylinder".into())
                .build(),
        );
        styles.add_element_style(
            ElementStyle::builder()
                .identifier("Boundary".into())
                .stroke_width("5".into())
                .build(),
        );

        let dsl = styles.serialize().unwrap();
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
        styles.add_relationship_style(RelationshipStyle::builder().thickness("4".into()).build());

        let dsl = styles.serialize().unwrap();
        assert!(dsl.contains("relationship {"));
        assert!(dsl.contains("thickness 4"));
    }
}
