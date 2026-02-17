use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
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

pub mod software_system_builder {
    #[derive(Debug, Clone, Default)]
    pub struct NoName;
    #[derive(Debug, Clone, Default)]
    pub struct HasName;
    #[derive(Debug, Clone, Default)]
    pub struct NoDescription;
    #[derive(Debug, Clone, Default)]
    pub struct HasDescription;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SoftwareSystem {
    identifier: ElementIdentifier,
    name: NonEmptyString,
    description: NonEmptyString,
    location: Location,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    containers: Vec<Container>,
}

impl SoftwareSystem {
    pub fn builder() -> SoftwareSystemBuilder<
        software_system_builder::NoName,
        software_system_builder::NoDescription,
    > {
        SoftwareSystemBuilder::new()
    }

    pub fn identifier(&self) -> &ElementIdentifier {
        &self.identifier
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    pub fn location(&self) -> Location {
        self.location.clone()
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
        &self.identifier
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
        self.location.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SoftwareSystemBuilder<N, D> {
    #[serde(skip)]
    _name: PhantomData<N>,
    #[serde(skip)]
    _description: PhantomData<D>,
    identifier: Option<ElementIdentifier>,
    name: Option<NonEmptyString>,
    description: Option<NonEmptyString>,
    location: Location,
    containers: Vec<Container>,
}

impl Default
    for SoftwareSystemBuilder<
        software_system_builder::NoName,
        software_system_builder::NoDescription,
    >
{
    fn default() -> Self {
        Self::new()
    }
}

impl
    SoftwareSystemBuilder<software_system_builder::NoName, software_system_builder::NoDescription>
{
    pub fn new() -> Self {
        SoftwareSystemBuilder {
            _name: PhantomData,
            _description: PhantomData,
            identifier: None,
            name: None,
            description: None,
            location: Location::Internal,
            containers: Vec::new(),
        }
    }
}

impl<D> SoftwareSystemBuilder<software_system_builder::NoName, D> {
    pub fn with_identifier(mut self, identifier: ElementIdentifier) -> Self {
        self.identifier = Some(identifier);
        self
    }

    pub fn with_name(
        self,
        name: NonEmptyString,
    ) -> SoftwareSystemBuilder<software_system_builder::HasName, D> {
        SoftwareSystemBuilder {
            _name: PhantomData,
            _description: self._description,
            identifier: self.identifier,
            name: Some(name),
            description: self.description,
            location: self.location,
            containers: self.containers,
        }
    }
}

impl<N> SoftwareSystemBuilder<N, software_system_builder::NoDescription> {
    pub fn with_description(
        self,
        description: NonEmptyString,
    ) -> SoftwareSystemBuilder<N, software_system_builder::HasDescription> {
        SoftwareSystemBuilder {
            _name: self._name,
            _description: PhantomData,
            identifier: self.identifier,
            name: self.name,
            description: Some(description),
            location: self.location,
            containers: self.containers,
        }
    }
}

impl<N, D> SoftwareSystemBuilder<N, D> {
    pub fn with_location(mut self, location: Location) -> Self {
        self.location = location;
        self
    }

    pub fn add_container(mut self, container: Container) -> Self {
        self.containers.push(container);
        self
    }
}

impl
    SoftwareSystemBuilder<software_system_builder::HasName, software_system_builder::HasDescription>
{
    pub fn build(self) -> SoftwareSystem {
        SoftwareSystem {
            identifier: self.identifier.unwrap_or_default(),
            name: self.name.unwrap(),
            description: self.description.unwrap(),
            location: self.location,
            containers: self.containers,
        }
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
            .with_name("E-Commerce Platform".try_into().unwrap())
            .with_description("Online shopping system".try_into().unwrap())
            .build();

        assert_eq!(system.name(), "E-Commerce Platform");
        assert!(system.containers().is_empty());
    }
}
