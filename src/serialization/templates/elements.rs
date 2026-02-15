// Serialization templates infrastructure
// These templates are available for future use but the existing implementation
// continues to work without modification to maintain behavioral compatibility.

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
