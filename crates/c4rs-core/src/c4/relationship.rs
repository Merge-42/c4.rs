use bon::Builder;
use serde::{Deserialize, Serialize};

use super::element::{Element, InteractionStyle};

use super::code::CodeElement;
use super::component::Component;
use super::container::Container;
use super::context::Person;
use crate::constants::limits::MAX_TECHNOLOGY_LENGTH;

/// Generic relationship between any two C4 elements.
///
/// Relationships connect elements and show how they interact.
/// The generic parameters S and T allow relationships between any Element types,
/// including cross-level relationships (e.g., Person → Container).
///
/// # Examples
///
/// ```
/// use c4rs_core::c4::{Person, Container, Relationship, InteractionStyle};
///
/// // Same type relationship (Person → Person)
/// let person1 = Person::builder().name("Alice".into()).description("User 1".into()).build().unwrap();
/// let person2 = Person::builder().name("Bob".into()).description("User 2".into()).build().unwrap();
///
/// let relationship: Relationship<Person, Person> = Relationship::builder()
///     .source(person1)
///     .target(person2)
///     .description("Communicates with".into())
///     .interaction_style(InteractionStyle::Synchronous)
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Builder)]
#[builder(finish_fn(vis = "", name = build_internal))]
pub struct Relationship<S: Element, T: Element> {
    /// The source element of the relationship.
    #[serde(skip_serializing)]
    source: S,
    /// The target element of the relationship.
    #[serde(skip_serializing)]
    target: T,
    /// Description of the relationship.
    description: String,
    /// Technology used for this relationship, if specified.
    technology: Option<String>,
    /// How the elements interact.
    #[builder(default)]
    interaction_style: InteractionStyle,
}

impl<S: Element, T: Element, State: relationship_builder::IsComplete>
    RelationshipBuilder<S, T, State>
{
    pub fn build(self) -> Result<Relationship<S, T>, RelationshipError> {
        let relationship = self.build_internal();
        if relationship.description.trim().is_empty() {
            return Err(RelationshipError::MissingDescription);
        }
        if let Some(ref tech) = relationship.technology
            && tech.len() > MAX_TECHNOLOGY_LENGTH
        {
            return Err(RelationshipError::TechnologyTooLong {
                max: MAX_TECHNOLOGY_LENGTH,
                actual: tech.len(),
            });
        }
        Ok(relationship)
    }
}

impl<S: Element, T: Element> Relationship<S, T> {
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
        &self.description
    }

    /// Returns the technology used by this relationship.
    pub fn technology(&self) -> Option<&str> {
        self.technology.as_deref()
    }

    /// Returns the interaction style.
    pub fn interaction_style(&self) -> InteractionStyle {
        self.interaction_style.clone()
    }
}

/// Creates a relationship between two elements.
///
/// This is a convenience function that uses the builder internally.
pub fn create_relationship<S: Element, T: Element>(
    source: S,
    target: T,
    description: String,
) -> Result<Relationship<S, T>, RelationshipError> {
    Relationship::builder()
        .source(source)
        .target(target)
        .description(description)
        .build()
}

/// Error type for Relationship construction.
#[derive(Debug, thiserror::Error)]
pub enum RelationshipError {
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
            .name("Alice".into())
            .description("User 1".into())
            .build()
            .unwrap();

        let person2 = Person::builder()
            .name("Bob".into())
            .description("User 2".into())
            .build()
            .unwrap();

        let relationship: Relationship<Person, Person> = Relationship::builder()
            .source(person1)
            .target(person2)
            .description("Communicates with".into())
            .interaction_style(InteractionStyle::Synchronous)
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
            .name("User".into())
            .description("A user".into())
            .build()
            .unwrap();

        let container = Container::builder()
            .name("Web API".into())
            .description("API".into())
            .container_type(super::super::element::ContainerType::Api)
            .build()
            .unwrap();

        let relationship: Relationship<Person, Container> = Relationship::builder()
            .source(person)
            .target(container)
            .description("Uses".into())
            .build()
            .unwrap();

        assert_eq!(relationship.description(), "Uses");
    }
}
