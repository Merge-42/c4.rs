use askama::Template;

#[derive(Template)]
#[template(
    source = r#"{{ identifier }} = person "{{ name }}" "{{ description }}""#,
    ext = "txt"
)]
pub struct PersonTemplate<'a> {
    pub identifier: &'a str,
    pub name: &'a str,
    pub description: &'a str,
}

#[derive(Template)]
#[template(
    source = r#"{{ identifier }} = softwareSystem "{{ name }}" "{{ description }}""#,
    ext = "txt"
)]
pub struct SoftwareSystemTemplate<'a> {
    pub identifier: &'a str,
    pub name: &'a str,
    pub description: &'a str,
}

#[derive(Template)]
#[template(
    source = r#"{{ identifier }} = container "{{ name }}" "{{ description }}"{% if let Some(t) = technology %} "{{ t }}"{% else %} ""{% endif %}"#,
    ext = "txt"
)]
pub struct ContainerTemplate<'a> {
    pub identifier: &'a str,
    pub name: &'a str,
    pub description: &'a str,
    pub technology: Option<&'a str>,
}

#[derive(Template)]
#[template(
    source = r#"{{ identifier }} = component "{{ name }}" "{{ description }}"{% if let Some(t) = technology %} "{{ t }}"{% else %} ""{% endif %}"#,
    ext = "txt"
)]
pub struct ComponentTemplate<'a> {
    pub identifier: &'a str,
    pub name: &'a str,
    pub description: &'a str,
    pub technology: Option<&'a str>,
}

#[derive(Template)]
#[template(
    source = r#"{{ source }} -> {{ target }} "{{ description }}"{% if let Some(t) = technology %} "{{ t }}"{% endif %}"#,
    ext = "txt"
)]
pub struct RelationshipTemplate<'a> {
    pub source: &'a str,
    pub target: &'a str,
    pub description: &'a str,
    pub technology: Option<&'a str>,
}
