use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

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
/// use c4rs_core::c4::{Person, Container, Relationship, InteractionStyle};
/// use c4rs_core::c4::value_types::NonEmptyString;
///
/// // Same type relationship (Person → Person)
/// let person1 = Person::builder()
///     .name("Alice".into())
///     .description("User 1".into())
///     .build();
///
/// let person2 = Person::builder()
///     .name("Bob".into())
///     .description("User 2".into())
///     .build();
///
/// let relationship: Relationship<Person, Person> = Relationship::builder()
///     .source(person1)
///     .target(person2)
///     .description("Communicates with".into())
///     .interaction_style(InteractionStyle::Synchronous)
///     .build();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
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
    #[builder(default)]
    technology: Option<NonEmptyString>,
    /// How the elements interact.
    #[builder(default)]
    interaction_style: InteractionStyle,
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

/// Creates a relationship between two elements.
///
/// This is a convenience function that uses the builder internally.
pub fn create_relationship<S: Element, T: Element>(
    source: S,
    target: T,
    description: NonEmptyString,
    technology: Option<NonEmptyString>,
    interaction_style: InteractionStyle,
) -> Relationship<S, T> {
    Relationship::builder()
        .source(source)
        .target(target)
        .description(description)
        .technology(technology)
        .interaction_style(interaction_style)
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
            .name("Alice".into())
            .description("User 1".into())
            .build();

        let person2 = Person::builder()
            .name("Bob".into())
            .description("User 2".into())
            .build();

        let relationship: Relationship<Person, Person> = Relationship::builder()
            .source(person1)
            .target(person2)
            .description("Communicates with".into())
            .interaction_style(InteractionStyle::Synchronous)
            .build();

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
            .build();

        let container = Container::builder()
            .name("Web API".into())
            .description("API".into())
            .container_type(super::super::element::ContainerType::Api)
            .build();

        let relationship: Relationship<Person, Container> = Relationship::builder()
            .source(person)
            .target(container)
            .description("Uses".into())
            .build();

        assert_eq!(relationship.description(), "Uses");
    }
}
