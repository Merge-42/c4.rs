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
    /// The tag this style applies to. Defaults to `"Relationship"` which
    /// targets all relationships (the implicit tag in Structurizr DSL).
    #[builder(default = "Relationship".to_string())]
    pub identifier: String,
    pub thickness: Option<String>,
    pub color: Option<String>,
    pub router: Option<String>,
    pub dashed: Option<bool>,
}
