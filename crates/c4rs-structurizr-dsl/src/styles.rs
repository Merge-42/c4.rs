use serde::{Deserialize, Serialize};

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

    pub fn with_background(mut self, color: &str) -> Self {
        self.background = Some(color.to_string());
        self
    }

    pub fn with_color(mut self, color: &str) -> Self {
        self.color = Some(color.to_string());
        self
    }

    pub fn with_shape(mut self, shape: &str) -> Self {
        self.shape = Some(shape.to_string());
        self
    }

    pub fn with_size(mut self, size: &str) -> Self {
        self.size = Some(size.to_string());
        self
    }

    pub fn with_stroke(mut self, color: &str) -> Self {
        self.stroke = Some(color.to_string());
        self
    }

    pub fn with_stroke_width(mut self, width: &str) -> Self {
        self.stroke_width = Some(width.to_string());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RelationshipStyle {
    pub thickness: Option<String>,
    pub color: Option<String>,
    pub router: Option<String>,
    pub dashed: Option<bool>,
}

impl RelationshipStyle {
    pub fn new() -> Self {
        Self {
            thickness: None,
            color: None,
            router: None,
            dashed: None,
        }
    }

    pub fn with_thickness(mut self, thickness: &str) -> Self {
        self.thickness = Some(thickness.to_string());
        self
    }

    pub fn with_color(mut self, color: &str) -> Self {
        self.color = Some(color.to_string());
        self
    }

    pub fn with_router(mut self, router: &str) -> Self {
        self.router = Some(router.to_string());
        self
    }

    pub fn with_dashed(mut self, dashed: bool) -> Self {
        self.dashed = Some(dashed);
        self
    }
}
