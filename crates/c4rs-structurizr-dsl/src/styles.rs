use bon::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct ElementStyle {
    pub identifier: String,
    pub background: Option<String>,
    pub color: Option<String>,
    pub shape: Option<String>,
    pub size: Option<String>,
    pub stroke: Option<String>,
    pub stroke_width: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
pub struct RelationshipStyle {
    pub thickness: Option<String>,
    pub color: Option<String>,
    pub router: Option<String>,
    pub dashed: Option<bool>,
}
