//! Relationship serialization to Structurizr DSL format.

use crate::c4::{Element, Relationship};
use crate::serialization::error::StructurizrDslError;
use crate::serialization::traits::{ElementSerializer, format_identifier};
use crate::serialization::writer::format_relationship;

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
        let source = format_identifier(self.source().name());
        let target = format_identifier(self.target().name());
        let description = self.description();
        let technology = self.technology();

        Ok(format_relationship(
            &source,
            &target,
            description,
            technology,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::c4::{Person, Relationship};

    #[test]
    fn test_relationship_serialization() {
        let source = Person::builder()
            .with_name("User".try_into().unwrap())
            .with_description("A user".try_into().unwrap())
            .build()
            .unwrap();

        let target = Person::builder()
            .with_name("Admin".try_into().unwrap())
            .with_description("An admin".try_into().unwrap())
            .build()
            .unwrap();

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
            .with_name("User".try_into().unwrap())
            .with_description("A user".try_into().unwrap())
            .build()
            .unwrap();

        let target = Person::builder()
            .with_name("API".try_into().unwrap())
            .with_description("Backend".try_into().unwrap())
            .build()
            .unwrap();

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
