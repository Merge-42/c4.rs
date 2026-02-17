//! Relationship serialization to Structurizr DSL format.

use crate::c4::{Element, Relationship};
use crate::serialization::error::StructurizrDslError;
use crate::serialization::templates::elements::RelationshipTemplate;
use crate::serialization::templates::helpers::format_identifier;
use crate::serialization::traits::ElementSerializer;
use askama::Template;

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
    use crate::c4::{Person, Relationship};

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
            .with_source(source)
            .with_target(target)
            .with_description("Reports to".try_into().unwrap())
            .build()
            .unwrap();

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
            .with_source(source)
            .with_target(target)
            .with_description("Uses".try_into().unwrap())
            .with_technology(Some("HTTPS".try_into().unwrap()))
            .build()
            .unwrap();

        let dsl = relationship.serialize_structurizr_dsl().unwrap();
        assert_eq!(dsl, r#"User -> API "Uses" "HTTPS""#);
    }
}
