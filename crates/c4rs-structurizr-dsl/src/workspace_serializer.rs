use crate::{
    StylesSerializer, ViewConfiguration, ViewsSerializer,
    error::DslError,
    identifier_generator::IdentifierGenerator,
    styles::{ElementStyle, RelationshipStyle},
    templates::helpers::escape_dsl_string,
    writer::{self, DslWriter},
};
use c4rs_core::c4::{Component, Container, Person, SoftwareSystem};
use std::collections::HashSet;

#[derive(Debug)]
pub struct WorkspaceSerializer {
    writer: DslWriter,
    used_identifiers: HashSet<String>,
    persons: Vec<Person>,
    software_systems: Vec<SoftwareSystem>,
    relationships: Vec<SerializedRelationship>,
    views_serializer: ViewsSerializer,
    styles_serializer: StylesSerializer,
    name: Option<String>,
    description: Option<String>,
}

#[derive(Debug)]
pub(crate) struct SerializedRelationship {
    pub(crate) source_id: String,
    pub(crate) target_id: String,
    pub(crate) description: String,
    pub(crate) technology: Option<String>,
}

impl Default for WorkspaceSerializer {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkspaceSerializer {
    pub fn new() -> Self {
        Self {
            writer: DslWriter::new(),
            used_identifiers: HashSet::new(),
            persons: Vec::new(),
            software_systems: Vec::new(),
            relationships: Vec::new(),
            views_serializer: ViewsSerializer::default(),
            styles_serializer: StylesSerializer::new(),
            name: None,
            description: None,
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn add_person(mut self, person: Person) -> Self {
        self.persons.push(person);
        self
    }

    pub fn add_software_system(mut self, system: SoftwareSystem) -> Self {
        self.software_systems.push(system);
        self
    }

    pub fn add_relationship(
        mut self,
        source_id: &str,
        target_id: &str,
        description: &str,
        technology: Option<&str>,
    ) -> Self {
        self.relationships.push(SerializedRelationship {
            source_id: source_id.to_string(),
            target_id: target_id.to_string(),
            description: description.to_string(),
            technology: technology.map(|s| s.to_string()),
        });
        self
    }

    pub fn add_view(mut self, view: ViewConfiguration) -> Self {
        self.views_serializer.add_view(view);
        self
    }

    pub fn add_element_style(mut self, style: ElementStyle) -> Self {
        self.styles_serializer = self.styles_serializer.add_element_style(style);
        self
    }

    pub fn add_relationship_style(mut self, style: RelationshipStyle) -> Self {
        self.styles_serializer = self.styles_serializer.add_relationship_style(style);
        self
    }

    pub fn add_element_styles(mut self, styles_dsl: &str) -> Self {
        self.styles_serializer = self
            .styles_serializer
            .set_external_output(styles_dsl.to_string());
        self.views_serializer
            .set_styles_output(styles_dsl.to_string());
        self
    }

    pub fn serialize(mut self) -> Result<String, DslError> {
        // Render styles and inject into views if needed
        let styles_dsl = self.styles_serializer.serialize()?;
        if !styles_dsl.is_empty() {
            self.views_serializer
                .set_styles_output(styles_dsl.to_string());
        }

        self.writer.clear();
        self.used_identifiers.clear();
        self.write_workspace_header()?;
        self.write_model_section()?;
        self.writer.unindent();
        self.writer.add_line("}");

        self.write_views_section()?;
        self.writer.unindent();
        self.writer.add_line("}");
        Ok(self.writer.as_output())
    }

    fn write_workspace_header(&mut self) -> Result<(), DslError> {
        let name = escape_dsl_string(self.name.as_deref().unwrap_or("Name"));
        let description = escape_dsl_string(self.description.as_deref().unwrap_or("Description"));
        self.writer
            .add_line(&format!(r#"workspace "{}" "{}" {{"#, name, description));
        self.writer.indent();
        self.writer.add_line("!identifiers hierarchical");
        self.writer.add_empty_line();
        self.writer.add_line("model {");
        self.writer.indent();
        Ok(())
    }

    fn write_model_section(&mut self) -> Result<(), DslError> {
        let person_names: Vec<String> = self.persons.iter().map(|p| p.name().to_string()).collect();
        let system_names: Vec<String> = self
            .software_systems
            .iter()
            .map(|s| s.name().to_string())
            .collect();

        for (person, name) in self.persons.iter().zip(person_names.iter()) {
            let identifier = IdentifierGenerator::generate_unique(name, &self.used_identifiers);
            self.used_identifiers.insert(identifier.clone());
            let dsl = Self::serialize_person(person, &identifier)?;
            self.writer.add_line(&dsl);
        }

        for (system, name) in self.software_systems.iter().zip(system_names.iter()) {
            let system_identifier =
                IdentifierGenerator::generate_unique(name, &self.used_identifiers);
            self.used_identifiers.insert(system_identifier.clone());

            let has_containers = !system.containers().is_empty();

            let dsl = Self::serialize_software_system(system, &system_identifier, has_containers);
            self.writer.add_line(&dsl);

            if has_containers {
                self.writer.indent();
                let containers = system.containers();
                let container_names: Vec<String> =
                    containers.iter().map(|c| c.name().to_string()).collect();
                for (container, cname) in containers.iter().zip(container_names.iter()) {
                    let container_identifier =
                        IdentifierGenerator::generate_unique(cname, &self.used_identifiers);
                    self.used_identifiers.insert(container_identifier.clone());

                    let has_components = !container.components().is_empty();
                    let container_dsl =
                        Self::serialize_container(container, &container_identifier, has_components);
                    self.writer.add_line(&container_dsl);

                    if has_components {
                        self.writer.indent();
                        for component in container.components() {
                            let component_identifier = IdentifierGenerator::generate_unique(
                                component.name(),
                                &self.used_identifiers,
                            );
                            self.used_identifiers.insert(component_identifier.clone());
                            let component_dsl =
                                Self::serialize_component(component, &component_identifier)?;
                            self.writer.add_line(&component_dsl);
                        }
                        self.writer.unindent();
                        self.writer.add_line("}");
                    }
                }
                self.writer.unindent();
                self.writer.add_line("}");
            }
        }

        for rel in &self.relationships {
            let dsl = writer::format_relationship(
                &rel.source_id,
                &rel.target_id,
                &rel.description,
                rel.technology.as_deref(),
            );
            self.writer.add_line(&dsl);
        }

        Ok(())
    }

    fn serialize_person(person: &Person, identifier: &str) -> Result<String, DslError> {
        let base = writer::format_element_assignment(
            identifier,
            "person",
            person.name(),
            person.description(),
            None,
        );
        if person.location() == c4rs_core::c4::Location::External {
            Ok(format!(
                r#"{}" {{
    tags "External"
}}"#,
                base
            ))
        } else {
            Ok(base)
        }
    }

    fn serialize_software_system(
        system: &SoftwareSystem,
        identifier: &str,
        has_containers: bool,
    ) -> String {
        let base = writer::format_element_assignment(
            identifier,
            "softwareSystem",
            system.name(),
            system.description(),
            None,
        );
        if has_containers {
            let external_tag = if system.location() == c4rs_core::c4::Location::External {
                "\n    tags \"External\""
            } else {
                ""
            };
            format!("{} {{{}", base, external_tag)
        } else {
            base
        }
    }

    fn serialize_container(
        container: &Container,
        identifier: &str,
        has_components: bool,
    ) -> String {
        let base = writer::format_element_assignment(
            identifier,
            "container",
            container.name(),
            container.description(),
            None,
        );
        if has_components {
            format!("{} {{", base)
        } else {
            format!("{} {{}}", base)
        }
    }

    fn serialize_component(component: &Component, identifier: &str) -> Result<String, DslError> {
        Ok(writer::format_element_assignment(
            identifier,
            "component",
            component.name(),
            component.description(),
            component.technology(),
        ))
    }

    fn write_views_section(&mut self) -> Result<(), DslError> {
        let views_dsl = self.views_serializer.serialize()?;
        if !views_dsl.is_empty() {
            self.writer.add_empty_line();
            let indented = DslWriter::indent_block(&views_dsl);
            for line in indented.lines() {
                if line.trim().is_empty() {
                    self.writer.add_empty_line();
                } else {
                    self.writer.add_line(line);
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "workspace_serializer_tests.rs"]
mod tests;
