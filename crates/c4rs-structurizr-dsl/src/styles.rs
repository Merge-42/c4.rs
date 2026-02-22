use bon::Builder;
#[derive(Debug, Clone, Builder)]
pub struct ElementStyle {
    pub identifier: String,
    pub background: Option<String>,
    pub color: Option<String>,
    pub shape: Option<String>,
    pub size: Option<String>,
    pub stroke: Option<String>,
    pub stroke_width: Option<String>,
}

#[derive(Debug, Clone, Builder)]
pub struct RelationshipStyle {
    pub identifier: String,
    pub thickness: Option<String>,
    pub color: Option<String>,
    pub router: Option<String>,
    pub dashed: Option<bool>,
}

impl Default for RelationshipStyle {
    fn default() -> Self {
        Self {
            identifier: "Relationship".to_string(),
            thickness: None,
            color: None,
            router: None,
            dashed: None,
        }
    }
}
