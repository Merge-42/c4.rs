use crate::error::DslError;
use crate::styles::{ElementStyle, RelationshipStyle};
use crate::views_serializer::ViewConfiguration;
use crate::workspace_serializer::WorkspaceSerializer;
use c4rs_core::c4::{Person, SoftwareSystem};

/// Convenience facade over `WorkspaceSerializer` that provides a
/// consuming builder API for constructing and serializing a complete
/// Structurizr DSL workspace.
#[derive(Debug, Default)]
pub struct DslSerializer {
    inner: WorkspaceSerializer,
}

impl DslSerializer {
    pub fn new() -> Self {
        Self {
            inner: WorkspaceSerializer::new(),
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.inner = self.inner.name(name);
        self
    }

    pub fn with_description(mut self, description: &str) -> Self {
        self.inner = self.inner.description(description);
        self
    }

    pub fn add_person(mut self, person: Person) -> Self {
        self.inner = self.inner.add_person(person);
        self
    }

    pub fn add_software_system(mut self, system: SoftwareSystem) -> Self {
        self.inner = self.inner.add_software_system(system);
        self
    }

    pub fn add_view(mut self, view: ViewConfiguration) -> Self {
        self.inner = self.inner.add_view(view);
        self
    }

    pub fn add_element_style(mut self, style: ElementStyle) -> Self {
        self.inner = self.inner.add_element_style(style);
        self
    }

    pub fn add_relationship_style(mut self, style: RelationshipStyle) -> Self {
        self.inner = self.inner.add_relationship_style(style);
        self
    }

    pub fn add_relationship(
        mut self,
        source_id: &str,
        target_id: &str,
        description: &str,
        technology: Option<&str>,
    ) -> Self {
        self.inner = self
            .inner
            .add_relationship(source_id, target_id, description, technology);
        self
    }

    pub fn serialize(self) -> Result<String, DslError> {
        self.inner.serialize()
    }
}

#[cfg(test)]
#[path = "structurizr_dsl_tests.rs"]
mod tests;
