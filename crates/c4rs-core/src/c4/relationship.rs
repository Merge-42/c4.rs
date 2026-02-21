use bon::Builder;
use serde::{Deserialize, Serialize};

use super::element::{Element, InteractionStyle};

use super::code::CodeElement;
use super::component::Component;
use super::container::Container;
use super::context::Person;
use crate::constants::limits::MAX_TECHNOLOGY_LENGTH;
use crate::validation::{validate_max_length, validate_non_empty};

/// Relationship between two C4 elements.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Builder)]
#[builder(finish_fn(vis = "", name = build_internal))]
pub struct Relationship<S: Element, T: Element> {
    #[serde(skip_serializing)]
    source: S,
    #[serde(skip_serializing)]
    target: T,
    description: String,
    technology: Option<String>,
    #[builder(default)]
    interaction_style: InteractionStyle,
}

impl<S: Element, T: Element, State: relationship_builder::IsComplete>
    RelationshipBuilder<S, T, State>
{
    pub fn build(self) -> Result<Relationship<S, T>, RelationshipError> {
        let relationship = self.build_internal();
        validate_non_empty(&relationship.description, "description")?;
        validate_max_length(
            &relationship.technology,
            MAX_TECHNOLOGY_LENGTH,
            "technology",
        )?;
        Ok(relationship)
    }
}

impl<S: Element, T: Element> Relationship<S, T> {
    pub fn source(&self) -> &S {
        &self.source
    }

    pub fn target(&self) -> &T {
        &self.target
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn technology(&self) -> Option<&str> {
        self.technology.as_deref()
    }

    pub fn interaction_style(&self) -> InteractionStyle {
        self.interaction_style.clone()
    }
}

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

#[derive(Debug, thiserror::Error)]
pub enum RelationshipError {
    #[error("relationship description is required and cannot be empty")]
    MissingDescription,
    #[error("technology string exceeds maximum length of {max} characters (actual: {actual})")]
    TechnologyTooLong { max: usize, actual: usize },
    #[error("validation error: {0}")]
    Validation(#[from] crate::validation::ValidationError),
}

pub type PersonRelationship = Relationship<Person, Person>;
pub type PersonToContainerRelationship = Relationship<Person, Container>;
pub type ContainerRelationship = Relationship<Container, Container>;
pub type ComponentRelationship = Relationship<Component, Component>;
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
