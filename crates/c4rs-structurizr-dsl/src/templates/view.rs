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
#[template(
    source = r#"    element "{{ identifier }}" {
{% if let Some(bg) = background %}        background {{ bg }}
{% endif %}{% if let Some(c) = color %}        color {{ c }}
{% endif %}{% if let Some(s) = shape %}        shape {{ s }}
{% endif %}{% if let Some(sz) = size %}        size {{ sz }}
{% endif %}{% if let Some(st) = stroke %}        stroke {{ st }}
{% endif %}{% if let Some(sw) = stroke_width %}        strokeWidth {{ sw }}
{% endif %}    }"#,
    ext = "txt"
)]
pub struct ElementStyleTemplate<'a> {
    pub identifier: &'a str,
    pub background: Option<&'a str>,
    pub color: Option<&'a str>,
    pub shape: Option<&'a str>,
    pub size: Option<&'a str>,
    pub stroke: Option<&'a str>,
    pub stroke_width: Option<&'a str>,
}

#[derive(Template)]
#[template(
    source = r#"    relationship {
{% if let Some(t) = thickness %}        thickness {{ t }}
{% endif %}{% if let Some(c) = color %}        color {{ c }}
{% endif %}{% if let Some(r) = router %}        router {{ r }}
{% endif %}{% if let Some(d) = dashed %}        dashed {{ d }}
{% endif %}    }"#,
    ext = "txt"
)]
pub struct RelationshipStyleTemplate<'a> {
    pub thickness: Option<&'a str>,
    pub color: Option<&'a str>,
    pub router: Option<&'a str>,
    pub dashed: Option<&'a str>,
}
