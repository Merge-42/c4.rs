use bon::Builder;
use serde::{Deserialize, Serialize};

use super::container::Container;
use super::element::{Element, ElementType, Location};
use super::value_types::ElementIdentifier;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Builder)]
#[builder(finish_fn(vis = "", name = build_internal))]
pub struct Person {
    #[serde(skip)]
    identifier: Option<ElementIdentifier>,
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
            identifier: None,
            name,
            description,
            location: None,
            technology: None,
        })
    }

    pub fn identifier(&self) -> &ElementIdentifier {
        self.identifier.as_ref().unwrap_or_else(|| {
            static DEFAULT: std::sync::LazyLock<ElementIdentifier> =
                std::sync::LazyLock::new(ElementIdentifier::default);
            &DEFAULT
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn location(&self) -> Location {
        self.location.clone().unwrap_or(Location::Internal)
    }

    pub fn technology(&self) -> Option<&str> {
        self.technology.as_deref()
    }
}

impl Element for Person {
    fn identifier(&self) -> &ElementIdentifier {
        self.identifier.as_ref().unwrap_or_else(|| {
            static DEFAULT: std::sync::LazyLock<ElementIdentifier> =
                std::sync::LazyLock::new(ElementIdentifier::default);
            &DEFAULT
        })
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn element_type(&self) -> ElementType {
        ElementType::Person
    }

    fn location(&self) -> Location {
        self.location.clone().unwrap_or(Location::Internal)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PersonError {
    #[error("person name is required and cannot be empty")]
    MissingName,

    #[error("person description is required and cannot be empty")]
    MissingDescription,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Builder)]
#[builder(finish_fn(vis = "", name = build_internal))]
pub struct SoftwareSystem {
    #[builder(field)]
    containers: Vec<Container>,
    #[serde(skip)]
    identifier: Option<ElementIdentifier>,
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
            identifier: None,
            name,
            description,
            location: None,
            containers: Vec::new(),
        })
    }

    pub fn identifier(&self) -> &ElementIdentifier {
        self.identifier.as_ref().unwrap_or_else(|| {
            static DEFAULT: std::sync::LazyLock<ElementIdentifier> =
                std::sync::LazyLock::new(ElementIdentifier::default);
            &DEFAULT
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn location(&self) -> Location {
        self.location.clone().unwrap_or(Location::Internal)
    }

    pub fn containers(&self) -> &[Container] {
        &self.containers
    }

    pub fn add_container(&mut self, container: Container) {
        self.containers.push(container);
    }
}

impl Element for SoftwareSystem {
    fn identifier(&self) -> &ElementIdentifier {
        self.identifier.as_ref().unwrap_or_else(|| {
            static DEFAULT: std::sync::LazyLock<ElementIdentifier> =
                std::sync::LazyLock::new(ElementIdentifier::default);
            &DEFAULT
        })
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn element_type(&self) -> ElementType {
        ElementType::SoftwareSystem
    }

    fn location(&self) -> Location {
        self.location.clone().unwrap_or(Location::Internal)
    }
}

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
        let person = Person::new("Alice", "System administrator").unwrap();

        assert_eq!(person.name(), "Alice");
        assert_eq!(person.description(), "System administrator");
        assert_eq!(person.location(), Location::Internal);
        assert!(!person.identifier().inner().is_nil());
    }

    #[test]
    fn test_person_empty_name() {
        let result = Person::new("", "description");
        assert!(result.is_err());
    }

    #[test]
    fn test_person_empty_description() {
        let result = Person::new("name", "");
        assert!(result.is_err());
    }

    #[test]
    fn test_person_builder() {
        let person = Person::builder()
            .name("Alice".into())
            .description("Admin".into())
            .build()
            .unwrap();

        assert_eq!(person.name(), "Alice");
    }

    #[test]
    fn test_software_system() {
        let system = SoftwareSystem::builder()
            .name("E-Commerce Platform".into())
            .description("Online shopping system".into())
            .build()
            .unwrap();

        assert_eq!(system.name(), "E-Commerce Platform");
        assert!(system.containers().is_empty());
    }
}
