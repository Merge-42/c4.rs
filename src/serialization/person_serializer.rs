//! Person serialization to Structurizr DSL format.

use crate::c4::Person;
use crate::serialization::error::StructurizrDslError;
use crate::serialization::templates::elements::PersonTemplate;
use crate::serialization::templates::helpers::{escape_dsl_string, format_identifier};
use crate::serialization::traits::ElementSerializer;
use askama::Template;

/// Serializes a Person element to Structurizr DSL format.
///
/// Person format: `person = person "name" "description"`
impl ElementSerializer for Person {
    fn serialize_structurizr_dsl(&self) -> Result<String, StructurizrDslError> {
        let identifier = format_identifier(self.name());
        let template = PersonTemplate {
            identifier,
            name: escape_dsl_string(self.name()),
            description: escape_dsl_string(self.description()),
        };
        Ok(template.render()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::c4::Person;

    #[test]
    fn test_person_serialization() {
        let person = Person::builder()
            .with_name("User".try_into().unwrap())
            .with_description("A system user".try_into().unwrap())
            .build();

        let dsl = person.serialize_structurizr_dsl().unwrap();
        assert_eq!(dsl, r#"User = person "User" "A system user""#);
    }

    #[test]
    fn test_person_serialization_special_chars() {
        let person = Person::builder()
            .with_name("User\"Name".try_into().unwrap())
            .with_description("A \"test\" user".try_into().unwrap())
            .build();

        let dsl = person.serialize_structurizr_dsl().unwrap();
        assert_eq!(dsl, r#"User_Name = person "User\"Name" "A \"test\" user""#);
    }

    #[test]
    fn test_person_serialization_spaces() {
        let person = Person::builder()
            .with_name("System User".try_into().unwrap())
            .with_description("A system user".try_into().unwrap())
            .build();

        let dsl = person.serialize_structurizr_dsl().unwrap();
        assert_eq!(dsl, r#"System_User = person "System User" "A system user""#);
    }
}
