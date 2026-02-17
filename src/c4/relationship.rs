use serde::{Deserialize, Serialize};

use super::element::{Element, InteractionStyle};
use super::value_types::{ElementIdentifier, NonEmptyString};

use super::code::CodeElement;
use super::component::Component;
use super::container::Container;
use super::context::Person;

/// Generic relationship between any two C4 elements.
///
/// Relationships connect elements and show how they interact.
/// The generic parameters S and T allow relationships between any Element types,
/// including cross-level relationships (e.g., Person → Container).
///
/// # Examples
///
/// ```
/// use c4rs::c4::{Person, Container, Relationship, InteractionStyle};
/// use c4rs::c4::value_types::NonEmptyString;
///
/// // Same type relationship (Person → Person)
/// let person1 = Person::builder()
///     .name("Alice".try_into().unwrap())
///     .description("User 1".try_into().unwrap())
///     .build();
///
/// let person2 = Person::builder()
///     .name("Bob".try_into().unwrap())
///     .description("User 2".try_into().unwrap())
///     .build();
///
/// let relationship: Relationship<Person, Person> = Relationship::builder()
///     .with_source(person1)
///     .with_target(person2)
///     .with_description("Communicates with".try_into().unwrap())
///     .with_interaction_style(InteractionStyle::Synchronous)
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Relationship<S: Element, T: Element> {
    /// The source element of the relationship.
    #[serde(skip_serializing)]
    source: S,
    /// The target element of the relationship.
    #[serde(skip_serializing)]
    target: T,
    /// Description of the relationship.
    description: NonEmptyString,
    /// Technology used for this relationship, if specified.
    technology: Option<NonEmptyString>,
    /// How the elements interact.
    interaction_style: InteractionStyle,
}

impl<S: Element, T: Element> Relationship<S, T> {
    /// Creates a new RelationshipBuilder.
    pub fn builder() -> RelationshipBuilder<S, T> {
        RelationshipBuilder::new()
    }

    /// Returns a reference to the source element.
    pub fn source(&self) -> &S {
        &self.source
    }

    /// Returns a reference to the target element.
    pub fn target(&self) -> &T {
        &self.target
    }

    /// Returns the relationship description.
    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    /// Returns the technology used by this relationship.
    pub fn technology(&self) -> Option<&str> {
        self.technology.as_deref()
    }

    /// Returns the interaction style.
    pub fn interaction_style(&self) -> InteractionStyle {
        self.interaction_style.clone()
    }

    /// Returns identifiers of source and target for serialization.
    pub fn endpoints(&self) -> (ElementIdentifier, ElementIdentifier) {
        (
            self.source.identifier().clone(),
            self.target.identifier().clone(),
        )
    }
}

/// Builder for constructing Relationship instances.
#[derive(Debug, Clone)]
pub struct RelationshipBuilder<S: Element, T: Element> {
    source: Option<S>,
    target: Option<T>,
    description: Option<NonEmptyString>,
    technology: Option<NonEmptyString>,
    interaction_style: InteractionStyle,
}

impl<S: Element, T: Element> Default for RelationshipBuilder<S, T> {
    fn default() -> Self {
        Self {
            source: None,
            target: None,
            description: None,
            technology: None,
            interaction_style: InteractionStyle::Synchronous,
        }
    }
}

impl<S: Element, T: Element> RelationshipBuilder<S, T> {
    /// Creates a new RelationshipBuilder.
    pub fn new() -> Self {
        Self {
            source: None,
            target: None,
            description: None,
            technology: None,
            interaction_style: InteractionStyle::Synchronous,
        }
    }

    /// Sets the source element.
    pub fn with_source(mut self, source: S) -> Self {
        self.source = Some(source);
        self
    }

    /// Sets the target element.
    pub fn with_target(mut self, target: T) -> Self {
        self.target = Some(target);
        self
    }

    /// Sets the relationship description.
    pub fn with_description(mut self, description: NonEmptyString) -> Self {
        self.description = Some(description);
        self
    }

    /// Sets the technology.
    pub fn with_technology(mut self, technology: Option<NonEmptyString>) -> Self {
        self.technology = technology;
        self
    }

    /// Sets the interaction style.
    pub fn with_interaction_style(mut self, interaction_style: InteractionStyle) -> Self {
        self.interaction_style = interaction_style;
        self
    }

    /// Builds the Relationship.
    pub fn build(self) -> Result<Relationship<S, T>, RelationshipError> {
        let source = self.source.ok_or(RelationshipError::MissingSource)?;
        let target = self.target.ok_or(RelationshipError::MissingTarget)?;
        let description = self
            .description
            .ok_or(RelationshipError::MissingDescription)?;

        if let Some(ref tech) = self.technology
            && tech.len() > 255
        {
            return Err(RelationshipError::TechnologyTooLong {
                max: 255,
                actual: tech.len(),
            });
        }

        Ok(Relationship {
            source,
            target,
            description,
            technology: self.technology,
            interaction_style: self.interaction_style,
        })
    }
}

/// Creates a relationship between two elements.
///
/// This is a convenience function that uses the builder internally.
pub fn create_relationship<S: Element, T: Element>(
    source: S,
    target: T,
    description: NonEmptyString,
    technology: Option<NonEmptyString>,
    interaction_style: InteractionStyle,
) -> Result<Relationship<S, T>, RelationshipError> {
    Relationship::builder()
        .with_source(source)
        .with_target(target)
        .with_description(description)
        .with_technology(technology)
        .with_interaction_style(interaction_style)
        .build()
}

/// Error type for Relationship construction.
#[derive(Debug, thiserror::Error)]
pub enum RelationshipError {
    #[error("relationship source element is required")]
    MissingSource,

    #[error("relationship target element is required")]
    MissingTarget,

    #[error("relationship description is required and cannot be empty")]
    MissingDescription,

    #[error("technology string exceeds maximum length of {max} characters (actual: {actual})")]
    TechnologyTooLong { max: usize, actual: usize },
}

/// Type alias for relationships between people.
pub type PersonRelationship = Relationship<Person, Person>;

/// Type alias for relationships between people and containers.
pub type PersonToContainerRelationship = Relationship<Person, Container>;

/// Type alias for relationships between containers.
pub type ContainerRelationship = Relationship<Container, Container>;

/// Type alias for relationships between components.
pub type ComponentRelationship = Relationship<Component, Component>;

/// Type alias for relationships between components and code elements.
pub type ComponentToCodeRelationship = Relationship<Component, CodeElement>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relationship_builder() {
        let person1 = Person::builder()
            .name("Alice".try_into().unwrap())
            .description("User 1".try_into().unwrap())
            .build();

        let person2 = Person::builder()
            .name("Bob".try_into().unwrap())
            .description("User 2".try_into().unwrap())
            .build();

        let relationship: Relationship<Person, Person> = Relationship::builder()
            .with_source(person1)
            .with_target(person2)
            .with_description("Communicates with".try_into().unwrap())
            .with_interaction_style(InteractionStyle::Synchronous)
            .build()
            .unwrap();

        assert_eq!(relationship.description(), "Communicates with");
        assert_eq!(
            relationship.interaction_style(),
            InteractionStyle::Synchronous
        );
    }

    #[test]
    fn test_cross_level_relationship() {
        let person = Person::builder()
            .name("User".try_into().unwrap())
            .description("A user".try_into().unwrap())
            .build();

        let container = Container::builder()
            .name("Web API".try_into().unwrap())
            .description("API".try_into().unwrap())
            .container_type(super::super::element::ContainerType::Api)
            .build();

        let relationship: Relationship<Person, Container> = Relationship::builder()
            .with_source(person)
            .with_target(container)
            .with_description("Uses".try_into().unwrap())
            .build()
            .unwrap();

        assert_eq!(relationship.description(), "Uses");
    }

    #[test]
    fn test_relationship_error_missing_source() {
        let result = Relationship::<Person, Person>::builder()
            .with_target(
                Person::builder()
                    .name("Target".try_into().unwrap())
                    .description("Target".try_into().unwrap())
                    .build(),
            )
            .with_description("Has".try_into().unwrap())
            .build();

        assert!(result.is_err());
    }
}
