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
    source = r#"{{ identifier }} = container "{{ name }}" "{{ description }}" "{{ technology }}""#,
    ext = "txt"
)]
pub struct ContainerTemplate<'a> {
    pub identifier: &'a str,
    pub name: &'a str,
    pub description: &'a str,
    pub technology: &'a str,
}

#[derive(Template)]
#[template(
    source = r#"{{ identifier }} = component "{{ name }}" "{{ description }}" "{{ technology }}""#,
    ext = "txt"
)]
pub struct ComponentTemplate<'a> {
    pub identifier: &'a str,
    pub name: &'a str,
    pub description: &'a str,
    pub technology: &'a str,
}

#[derive(Template)]
#[template(
    source = r#"{{ source }} -> {{ target }} "{{ description }}" "{{ technology }}""#,
    ext = "txt"
)]
pub struct RelationshipTemplate<'a> {
    pub source: &'a str,
    pub target: &'a str,
    pub description: &'a str,
    pub technology: &'a str,
}

#[derive(Template)]
#[template(
    source = r#"{{ source }} -> {{ target }} "{{ description }}""#,
    ext = "txt"
)]
pub struct RelationshipNoTechTemplate<'a> {
    pub source: &'a str,
    pub target: &'a str,
    pub description: &'a str,
}
