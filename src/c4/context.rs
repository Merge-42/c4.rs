use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::container::Container;
use super::element::{Element, ElementType, Location};
use super::value_types::{ElementIdentifier, NonEmptyString};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
pub struct Person {
    #[builder(default, setter(strip_option))]
    identifier: Option<ElementIdentifier>,

    name: NonEmptyString,
    description: NonEmptyString,

    #[builder(default, setter(strip_option))]
    location: Option<Location>,

    #[builder(default, setter(strip_option))]
    technology: Option<NonEmptyString>,
}

impl Person {
    pub fn identifier(&self) -> &ElementIdentifier {
        self.identifier.as_ref().unwrap_or_else(|| {
            static DEFAULT: std::sync::LazyLock<ElementIdentifier> =
                std::sync::LazyLock::new(ElementIdentifier::default);
            &DEFAULT
        })
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn description(&self) -> &str {
        self.description.as_str()
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
        self.name.as_str()
    }

    fn description(&self) -> &str {
        self.description.as_str()
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

    #[error("technology string exceeds maximum length of {max} characters (actual: {actual})")]
    TechnologyTooLong { max: usize, actual: usize },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
pub struct SoftwareSystem {
    #[builder(default)]
    identifier: Option<ElementIdentifier>,
    name: NonEmptyString,
    description: NonEmptyString,
    #[builder(default)]
    location: Option<Location>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    containers: Vec<Container>,
}

impl SoftwareSystem {
    pub fn identifier(&self) -> &ElementIdentifier {
        self.identifier.as_ref().unwrap_or_else(|| {
            static DEFAULT: std::sync::LazyLock<ElementIdentifier> =
                std::sync::LazyLock::new(ElementIdentifier::default);
            &DEFAULT
        })
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn description(&self) -> &str {
        self.description.as_str()
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

    pub fn build(self) -> SoftwareSystem {
        SoftwareSystem {
            identifier: self.identifier,
            name: self.name,
            description: self.description,
            location: self.location,
            containers: self.containers,
        }
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
        self.name.as_str()
    }

    fn description(&self) -> &str {
        self.description.as_str()
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
    #[error("system identifier is required")]
    MissingIdentifier,

    #[error("system name is required and cannot be empty")]
    MissingName,

    #[error("system description is required and cannot be empty")]
    MissingDescription,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_person_builder() {
        let person = Person::builder()
            .name("Alice".try_into().unwrap())
            .description("System administrator".try_into().unwrap())
            .location(Location::Internal)
            .build();

        assert_eq!(person.name(), "Alice");
        assert_eq!(person.description(), "System administrator");
        assert_eq!(person.location(), Location::Internal);
        assert!(!person.identifier().inner().is_nil());
    }

    #[test]
    fn test_person_with_technology() {
        let person = Person::builder()
            .name("Bob".try_into().unwrap())
            .description("API consumer".try_into().unwrap())
            .location(Location::External)
            .technology("Python 3.11".try_into().unwrap())
            .build();

        assert_eq!(person.technology(), Some("Python 3.11"));
    }

    #[test]
    fn test_software_system() {
        let system = SoftwareSystem::builder()
            .name("E-Commerce Platform".try_into().unwrap())
            .description("Online shopping system".try_into().unwrap())
            .build();

        assert_eq!(system.name(), "E-Commerce Platform");
        assert!(system.containers().is_empty());
    }
}
