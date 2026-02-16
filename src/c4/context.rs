use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

use super::container::Container;
use super::element::{Element, ElementType, Location};
use super::value_types::{ElementIdentifier, NonEmptyString};

pub mod person_builder {
    #[derive(Debug, Clone, Default)]
    pub struct NoName;
    #[derive(Debug, Clone, Default)]
    pub struct HasName;
    #[derive(Debug, Clone, Default)]
    pub struct NoDescription;
    #[derive(Debug, Clone, Default)]
    pub struct HasDescription;
}

/// Represents a user or actor in the system.
///
/// Persons are the people who use the software system being modeled.
/// They can be internal (part of the organization) or external (users, customers, etc.).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Person {
    identifier: ElementIdentifier,
    name: NonEmptyString,
    description: NonEmptyString,
    location: Location,
    technology: Option<NonEmptyString>,
}

impl Person {
    /// Creates a new PersonBuilder for constructing a Person.
    pub fn builder() -> PersonBuilder<person_builder::NoName, person_builder::NoDescription> {
        PersonBuilder::new()
    }

    /// Returns a reference to the person's unique identifier.
    pub fn identifier(&self) -> &ElementIdentifier {
        &self.identifier
    }

    /// Returns the person's name.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the person's description.
    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    /// Returns whether the person is internal or external.
    pub fn location(&self) -> Location {
        self.location.clone()
    }

    /// Returns the technology used by this person, if specified.
    pub fn technology(&self) -> Option<&str> {
        self.technology.as_deref()
    }
}

impl Element for Person {
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
        ElementType::Person
    }

    fn location(&self) -> Location {
        self.location.clone()
    }
}

/// Builder for constructing Person instances with validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonBuilder<N, D> {
    #[serde(skip)]
    _name: PhantomData<N>,
    #[serde(skip)]
    _description: PhantomData<D>,
    identifier: Option<ElementIdentifier>,
    name: Option<NonEmptyString>,
    description: Option<NonEmptyString>,
    #[serde(default)]
    location: Location,
    technology: Option<NonEmptyString>,
}

impl PersonBuilder<person_builder::NoName, person_builder::NoDescription> {
    pub fn new() -> Self {
        PersonBuilder {
            _name: PhantomData,
            _description: PhantomData,
            identifier: None,
            name: None,
            description: None,
            location: Location::Internal,
            technology: None,
        }
    }
}

impl<D> PersonBuilder<person_builder::NoName, D> {
    pub fn with_identifier(mut self, identifier: ElementIdentifier) -> Self {
        self.identifier = Some(identifier);
        self
    }

    pub fn with_name(self, name: NonEmptyString) -> PersonBuilder<person_builder::HasName, D> {
        PersonBuilder {
            _name: PhantomData,
            _description: self._description,
            identifier: self.identifier,
            name: Some(name),
            description: self.description,
            location: self.location,
            technology: self.technology,
        }
    }
}

impl<N> PersonBuilder<N, person_builder::NoDescription> {
    pub fn with_description(
        self,
        description: NonEmptyString,
    ) -> PersonBuilder<N, person_builder::HasDescription> {
        PersonBuilder {
            _name: self._name,
            _description: PhantomData,
            identifier: self.identifier,
            name: self.name,
            description: Some(description),
            location: self.location,
            technology: self.technology,
        }
    }
}

impl<N, D> PersonBuilder<N, D> {
    pub fn with_location(mut self, location: Location) -> Self {
        self.location = location;
        self
    }

    pub fn with_technology(mut self, technology: NonEmptyString) -> Self {
        self.technology = Some(technology);
        self
    }
}

impl PersonBuilder<person_builder::HasName, person_builder::HasDescription> {
    pub fn build(self) -> Person {
        if let Some(ref tech) = self.technology
            && tech.len() > 255
        {
            panic!("technology string exceeds maximum length of 255 characters");
        }
        Person {
            identifier: self.identifier.unwrap_or_default(),
            name: self.name.unwrap(),
            description: self.description.unwrap(),
            location: self.location,
            technology: self.technology,
        }
    }
}

/// Error type for Person construction validation.
#[derive(Debug, thiserror::Error)]
pub enum PersonError {
    #[error("person identifier is required")]
    MissingIdentifier,

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

/// Represents a software system being described.
///
/// A SoftwareSystem is a top-level container that groups related Containers.
/// It represents the overall software that delivers value to users.
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
    /// Creates a new SoftwareSystemBuilder.
    pub fn builder() -> SoftwareSystemBuilder<
        software_system_builder::NoName,
        software_system_builder::NoDescription,
    > {
        SoftwareSystemBuilder::new()
    }

    /// Returns a reference to the system's unique identifier.
    pub fn identifier(&self) -> &ElementIdentifier {
        &self.identifier
    }

    /// Returns the system's name.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the system's description.
    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    /// Returns whether the system is internal or external.
    pub fn location(&self) -> Location {
        self.location.clone()
    }

    /// Returns the containers in this system.
    pub fn containers(&self) -> &[Container] {
        &self.containers
    }

    /// Adds a container to this system.
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

/// Builder for constructing SoftwareSystem instances.
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

/// Error type for SoftwareSystem construction.
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
            .with_name("Alice".try_into().unwrap())
            .with_description("System administrator".try_into().unwrap())
            .with_location(Location::Internal)
            .build();

        assert_eq!(person.name(), "Alice");
        assert_eq!(person.description(), "System administrator");
        assert_eq!(person.location(), Location::Internal);
        assert!(!person.identifier().inner().is_nil());
    }

    #[test]
    fn test_person_with_technology() {
        let person = Person::builder()
            .with_name("Bob".try_into().unwrap())
            .with_description("API consumer".try_into().unwrap())
            .with_location(Location::External)
            .with_technology("Python 3.11".try_into().unwrap())
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
