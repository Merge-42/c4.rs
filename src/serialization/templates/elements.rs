use askama::Template;

#[derive(Template)]
#[template(
    source = r#"{{ identifier }} = person "{{ name }}" "{{ description }}""#,
    ext = "txt",
    escape = "none"
)]
pub struct PersonTemplate {
    pub identifier: String,
    pub name: String,
    pub description: String,
}

#[derive(Template)]
#[template(
    source = r#"{{ identifier }} = softwareSystem "{{ name }}" "{{ description }}""#,
    ext = "txt",
    escape = "none"
)]
pub struct SoftwareSystemTemplate {
    pub identifier: String,
    pub name: String,
    pub description: String,
}

#[derive(Template)]
#[template(
    source = "{{ identifier }} = container \"{{ name }}\" \"{{ description }}\"{% if let Some(t) = technology %} \"{{ t }}\"{% else %} \"\"{% endif %}",
    ext = "txt",
    escape = "none"
)]
pub struct ContainerTemplate {
    pub identifier: String,
    pub name: String,
    pub description: String,
    pub technology: Option<String>,
}

#[derive(Template)]
#[template(
    source = "{{ identifier }} = component \"{{ name }}\" \"{{ description }}\"{% if let Some(t) = technology %} \"{{ t }}\"{% else %} \"\"{% endif %}",
    ext = "txt",
    escape = "none"
)]
pub struct ComponentTemplate {
    pub identifier: String,
    pub name: String,
    pub description: String,
    pub technology: Option<String>,
}

#[derive(Template)]
#[template(
    source = "{{ source }} -> {{ target }} \"{{ description }}\"{% if let Some(t) = technology %} \"{{ t }}\"{% endif %}",
    ext = "txt",
    escape = "none"
)]
pub struct RelationshipTemplate {
    pub source: String,
    pub target: String,
    pub description: String,
    pub technology: Option<String>,
}
