//! Person serialization to Structurizr DSL format.

use crate::c4::Person;
use crate::serialization::error::StructurizrDslError;
use crate::serialization::traits::{ElementSerializer, format_identifier};
use crate::serialization::writer::format_element_assignment;

/// Serializes a Person element to Structurizr DSL format.
///
/// Person format: `person = person "name" "description"`
impl ElementSerializer for Person {
    fn serialize_structurizr_dsl(&self) -> Result<String, StructurizrDslError> {
        let identifier = format_identifier(self.name());
        let name = self.name();
        let description = self.description();

        Ok(format_element_assignment(
            &identifier,
            "person",
            name,
            description,
            None,
        ))
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
            .build()
            .unwrap();

        let dsl = person.serialize_structurizr_dsl().unwrap();
        assert_eq!(dsl, r#"User = person "User" "A system user""#);
    }

    #[test]
    fn test_person_serialization_special_chars() {
        let person = Person::builder()
            .with_name("User\"Name".try_into().unwrap())
            .with_description("A \"test\" user".try_into().unwrap())
            .build()
            .unwrap();

        let dsl = person.serialize_structurizr_dsl().unwrap();
        assert_eq!(dsl, r#"User_Name = person "User\"Name" "A \"test\" user""#);
    }

    #[test]
    fn test_person_serialization_spaces() {
        let person = Person::builder()
            .with_name("System User".try_into().unwrap())
            .with_description("A system user".try_into().unwrap())
            .build()
            .unwrap();

        let dsl = person.serialize_structurizr_dsl().unwrap();
        assert_eq!(dsl, r#"System_User = person "System User" "A system user""#);
    }
}
