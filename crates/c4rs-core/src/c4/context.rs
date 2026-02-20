use bon::Builder;
use serde::{Deserialize, Serialize};

use super::container::Container;
use super::element::{ElementType, Location};
use super::macros::impl_element;
use crate::constants::limits::{MAX_DESCRIPTION_LENGTH, MAX_NAME_LENGTH, MAX_TECHNOLOGY_LENGTH};
use crate::validation::{validate_max_length, validate_non_empty};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Builder)]
#[builder(finish_fn(vis = "", name = build_internal))]
pub struct Person {
    name: String,
    description: String,
    location: Option<Location>,
    technology: Option<String>,
}

impl<S: person_builder::IsComplete> PersonBuilder<S> {
    pub fn build(self) -> Result<Person, PersonError> {
        let person = self.build_internal();
        validate_non_empty(&person.name, "name")?;
        validate_max_length(&person.name, MAX_NAME_LENGTH, "name")?;
        validate_non_empty(&person.description, "description")?;
        validate_max_length(&person.description, MAX_DESCRIPTION_LENGTH, "description")?;
        validate_max_length(&person.technology, MAX_TECHNOLOGY_LENGTH, "technology")?;
        Ok(person)
    }
}

impl_element!(Person, ElementType::Person, optional);

#[derive(Debug, thiserror::Error)]
pub enum PersonError {
    #[error("person name is required and cannot be empty")]
    MissingName,
    #[error("person description is required and cannot be empty")]
    MissingDescription,
    #[error("technology string exceeds maximum length of {max} characters (actual: {actual})")]
    TechnologyTooLong { max: usize, actual: usize },
    #[error("validation error: {0}")]
    Validation(#[from] crate::validation::ValidationError),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Builder)]
#[builder(finish_fn(vis = "", name = build_internal))]
pub struct SoftwareSystem {
    #[builder(field)]
    containers: Vec<Container>,
    name: String,
    description: String,
    location: Option<Location>,
}

impl<S: software_system_builder::IsComplete> SoftwareSystemBuilder<S> {
    pub fn add_container(mut self, container: Container) -> Self {
        self.containers.push(container);
        self
    }
    pub fn build(self) -> Result<SoftwareSystem, SoftwareSystemError> {
        let system = self.build_internal();
        validate_non_empty(&system.name, "name")?;
        validate_max_length(&system.name, MAX_NAME_LENGTH, "name")?;
        validate_non_empty(&system.description, "description")?;
        validate_max_length(&system.description, MAX_DESCRIPTION_LENGTH, "description")?;
        Ok(system)
    }
}

impl SoftwareSystem {
    pub fn containers(&self) -> &[Container] {
        &self.containers
    }
    pub fn add_container(&mut self, container: Container) {
        self.containers.push(container);
    }
}

impl_element!(SoftwareSystem, ElementType::SoftwareSystem, optional);

#[derive(Debug, thiserror::Error)]
pub enum SoftwareSystemError {
    #[error("system name is required and cannot be empty")]
    MissingName,
    #[error("system description is required and cannot be empty")]
    MissingDescription,
    #[error("validation error: {0}")]
    Validation(#[from] crate::validation::ValidationError),
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_person_builder() {
        let p = Person::builder()
            .name("A".into())
            .description("B".into())
            .build()
            .unwrap();
        assert_eq!(p.name(), "A");
    }
    #[test]
    fn test_person_empty_name() {
        assert!(
            Person::builder()
                .name("".into())
                .description("d".into())
                .build()
                .is_err()
        );
    }
    #[test]
    fn test_person_empty_desc() {
        assert!(
            Person::builder()
                .name("n".into())
                .description("".into())
                .build()
                .is_err()
        );
    }
    #[test]
    fn test_software_system() {
        let s = SoftwareSystem::builder()
            .name("E".into())
            .description("D".into())
            .build()
            .unwrap();
        assert_eq!(s.name(), "E");
    }
}
