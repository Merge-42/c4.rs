//! Relationship serialization to Structurizr DSL format.

use crate::error::StructurizrDslError;
use crate::templates::elements::RelationshipTemplate;
use crate::templates::helpers::format_identifier;
use crate::traits::ElementSerializer;
use askama::Template;
use c4rs_core::c4::{Element, Relationship};

/// Serializes a Relationship to Structurizr DSL format.
///
/// Relationship format: `source -> target "description" "technology"`
///
/// # Type Parameters
///
/// - `S`: Source element type
/// - `T`: Target element type
impl<S: Element, T: Element> ElementSerializer for Relationship<S, T> {
    fn serialize_structurizr_dsl(&self) -> Result<String, StructurizrDslError> {
        let template = RelationshipTemplate {
            source: format_identifier(self.source().name()),
            target: format_identifier(self.target().name()),
            description: self.description().to_string(),
            technology: self.technology().map(|s| s.to_string()),
        };
        Ok(template.render()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use c4rs_core::c4::{Person, Relationship};

    #[test]
    fn test_relationship_serialization() {
        let source = Person::builder()
            .name("User".try_into().unwrap())
            .description("A user".try_into().unwrap())
            .build();

        let target = Person::builder()
            .name("Admin".try_into().unwrap())
            .description("An admin".try_into().unwrap())
            .build();

        let relationship = Relationship::builder()
            .source(source)
            .target(target)
            .description("Reports to".try_into().unwrap())
            .build();

        let dsl = relationship.serialize_structurizr_dsl().unwrap();
        assert_eq!(dsl, r#"User -> Admin "Reports to""#);
    }

    #[test]
    fn test_relationship_with_technology() {
        let source = Person::builder()
            .name("User".try_into().unwrap())
            .description("A user".try_into().unwrap())
            .build();

        let target = Person::builder()
            .name("API".try_into().unwrap())
            .description("Backend".try_into().unwrap())
            .build();

        let relationship = Relationship::builder()
            .source(source)
            .target(target)
            .description("Uses".try_into().unwrap())
            .technology(Some("HTTPS".try_into().unwrap()))
            .build();

        let dsl = relationship.serialize_structurizr_dsl().unwrap();
        assert_eq!(dsl, r#"User -> API "Uses" "HTTPS""#);
    }
}
