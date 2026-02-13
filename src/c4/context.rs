use serde::{Deserialize, Serialize};

use super::container::Container;
use super::element::{Element, ElementType, Location};
use super::value_types::{ElementIdentifier, NonEmptyString};

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
    pub fn builder() -> PersonBuilder {
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
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PersonBuilder {
    identifier: Option<ElementIdentifier>,
    name: Option<NonEmptyString>,
    description: Option<NonEmptyString>,
    location: Location,
    technology: Option<NonEmptyString>,
}

impl PersonBuilder {
    /// Creates a new PersonBuilder with default values.
    pub fn new() -> Self {
        Self {
            identifier: None,
            name: None,
            description: None,
            location: Location::Internal,
            technology: None,
        }
    }

    /// Sets the element identifier.
    pub fn with_identifier(mut self, identifier: ElementIdentifier) -> Self {
        self.identifier = Some(identifier);
        self
    }

    /// Sets the person's name.
    pub fn with_name(mut self, name: NonEmptyString) -> Self {
        self.name = Some(name);
        self
    }

    /// Sets the person's description.
    pub fn with_description(mut self, description: NonEmptyString) -> Self {
        self.description = Some(description);
        self
    }

    /// Sets whether the person is internal or external.
    pub fn with_location(mut self, location: Location) -> Self {
        self.location = location;
        self
    }

    /// Sets the technology used by this person.
    pub fn with_technology(mut self, technology: NonEmptyString) -> Self {
        self.technology = Some(technology);
        self
    }

    /// Builds the Person, validating all fields.
    ///
    /// Returns an error if any required field is missing or invalid.
    pub fn build(self) -> Result<Person, PersonError> {
        let identifier = self.identifier.unwrap_or_default();
        let name = self.name.ok_or(PersonError::MissingName)?;
        let description = self.description.ok_or(PersonError::MissingDescription)?;

        if let Some(ref tech) = self.technology
            && tech.len() > 255
        {
            return Err(PersonError::TechnologyTooLong {
                max: 255,
                actual: tech.len(),
            });
        }

        Ok(Person {
            identifier,
            name,
            description,
            location: self.location,
            technology: self.technology,
        })
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
    pub fn builder() -> SoftwareSystemBuilder {
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
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct SoftwareSystemBuilder {
    identifier: Option<ElementIdentifier>,
    name: Option<NonEmptyString>,
    description: Option<NonEmptyString>,
    location: Location,
    containers: Vec<Container>,
}

impl SoftwareSystemBuilder {
    /// Creates a new SoftwareSystemBuilder.
    pub fn new() -> Self {
        Self {
            identifier: None,
            name: None,
            description: None,
            location: Location::Internal,
            containers: Vec::new(),
        }
    }

    /// Sets the element identifier.
    pub fn with_identifier(mut self, identifier: ElementIdentifier) -> Self {
        self.identifier = Some(identifier);
        self
    }

    /// Sets the system's name.
    pub fn with_name(mut self, name: NonEmptyString) -> Self {
        self.name = Some(name);
        self
    }

    /// Sets the system's description.
    pub fn with_description(mut self, description: NonEmptyString) -> Self {
        self.description = Some(description);
        self
    }

    /// Sets whether the system is internal or external.
    pub fn with_location(mut self, location: Location) -> Self {
        self.location = location;
        self
    }

    /// Adds a container to the system.
    pub fn add_container(mut self, container: Container) -> Self {
        self.containers.push(container);
        self
    }

    /// Builds the SoftwareSystem.
    pub fn build(self) -> Result<SoftwareSystem, SoftwareSystemError> {
        let identifier = self.identifier.unwrap_or_default();
        let name = self.name.ok_or(SoftwareSystemError::MissingName)?;
        let description = self
            .description
            .ok_or(SoftwareSystemError::MissingDescription)?;

        Ok(SoftwareSystem {
            identifier,
            name,
            description,
            location: self.location,
            containers: self.containers,
        })
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
            .build()
            .unwrap();

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
            .build()
            .unwrap();

        assert_eq!(person.technology(), Some("Python 3.11"));
    }

    #[test]
    fn test_person_error_missing_name() {
        let result = Person::builder()
            .with_description("No name".try_into().unwrap())
            .build();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "person name is required and cannot be empty"
        );
    }

    #[test]
    fn test_software_system() {
        let system = SoftwareSystem::builder()
            .with_name("E-Commerce Platform".try_into().unwrap())
            .with_description("Online shopping system".try_into().unwrap())
            .build()
            .unwrap();

        assert_eq!(system.name(), "E-Commerce Platform");
        assert!(system.containers().is_empty());
    }
}
