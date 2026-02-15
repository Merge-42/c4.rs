use askama::Template;

#[derive(Template)]
#[template(
    source = r#"    {{ view_type }} {{ identifier }} "{{ title }}" {
{% for inc in include_elements %}        include {{ inc }}
{% endfor %}{% for exc in exclude_elements %}        exclude {{ exc }}
{% endfor %}    }"#,
    ext = "txt"
)]
pub struct ViewTemplate<'a> {
    pub view_type: &'a str,
    pub identifier: &'a str,
    pub title: &'a str,
    pub include_elements: &'a [&'a str],
    pub exclude_elements: &'a [&'a str],
}

#[derive(Template)]
#[template(source = r#"    element "{{ identifier }}" {{ body }}"#, ext = "txt")]
pub struct ElementStyleTemplate<'a> {
    pub identifier: &'a str,
    pub body: &'a str,
}

#[derive(Template)]
#[template(source = r#"    relationship {{ body }}"#, ext = "txt")]
pub struct RelationshipStyleTemplate<'a> {
    pub body: &'a str,
}
