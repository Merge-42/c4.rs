use bon::Builder;
use serde::{Deserialize, Serialize};

use super::container::Container;
use super::element::{ElementType, Location};
use super::macros::impl_element;
use crate::constants::limits::MAX_TECHNOLOGY_LENGTH;

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
        if person.name.trim().is_empty() {
            return Err(PersonError::MissingName);
        }
        if person.description.trim().is_empty() {
            return Err(PersonError::MissingDescription);
        }
        if let Some(ref tech) = person.technology
            && tech.len() > MAX_TECHNOLOGY_LENGTH
        {
            return Err(PersonError::TechnologyTooLong {
                max: MAX_TECHNOLOGY_LENGTH,
                actual: tech.len(),
            });
        }
        Ok(person)
    }
}

impl Person {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Result<Person, PersonError> {
        let name = name.into();
        let description = description.into();
        if name.trim().is_empty() {
            return Err(PersonError::MissingName);
        }
        if description.trim().is_empty() {
            return Err(PersonError::MissingDescription);
        }
        Ok(Person {
            name,
            description,
            location: None,
            technology: None,
        })
    }
    pub fn technology(&self) -> Option<&str> {
        self.technology.as_deref()
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
        if system.name.trim().is_empty() {
            return Err(SoftwareSystemError::MissingName);
        }
        if system.description.trim().is_empty() {
            return Err(SoftwareSystemError::MissingDescription);
        }
        Ok(system)
    }
}

impl SoftwareSystem {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Result<SoftwareSystem, SoftwareSystemError> {
        let name = name.into();
        let description = description.into();
        if name.trim().is_empty() {
            return Err(SoftwareSystemError::MissingName);
        }
        if description.trim().is_empty() {
            return Err(SoftwareSystemError::MissingDescription);
        }
        Ok(SoftwareSystem {
            name,
            description,
            location: None,
            containers: Vec::new(),
        })
    }
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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_person_new() {
        let p = Person::new("A", "B").unwrap();
        assert_eq!(p.name(), "A");
    }
    #[test]
    fn test_person_empty_name() {
        assert!(Person::new("", "d").is_err());
    }
    #[test]
    fn test_person_empty_desc() {
        assert!(Person::new("n", "").is_err());
    }
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
    fn test_software_system() {
        let s = SoftwareSystem::builder()
            .name("E".into())
            .description("D".into())
            .build()
            .unwrap();
        assert_eq!(s.name(), "E");
    }
}
