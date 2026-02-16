use crate::c4::{Component, Container, Person, SoftwareSystem};
use crate::serialization::{
    StylesSerializer, ViewConfiguration, ViewsSerializer, error::StructurizrDslError,
    identifier_generator::IdentifierGenerator, writer::DslWriter,
};
use std::collections::HashSet;

/// Workspace serializer - handles all serialization for the Structurizr DSL.
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
pub struct SerializedRelationship {
    pub source_id: String,
    pub target_id: String,
    pub description: String,
    pub technology: Option<String>,
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
            views_serializer: ViewsSerializer::new(),
            styles_serializer: StylesSerializer::new(),
            name: None,
            description: None,
        }
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = Some(name.to_string());
    }

    pub fn set_description(&mut self, description: &str) {
        self.description = Some(description.to_string());
    }

    pub fn add_person(&mut self, person: Person) {
        self.persons.push(person);
    }
    pub fn add_software_system(&mut self, system: SoftwareSystem) {
        self.software_systems.push(system);
    }
    pub fn add_relationship(
        &mut self,
        source_id: &str,
        target_id: &str,
        description: &str,
        technology: Option<&str>,
    ) {
        self.relationships.push(SerializedRelationship {
            source_id: source_id.to_string(),
            target_id: target_id.to_string(),
            description: description.to_string(),
            technology: technology.map(|s| s.to_string()),
        });
    }

    pub fn set_views_output(&mut self, views_dsl: String) {
        self.views_serializer.set_external_output(views_dsl);
    }

    pub fn add_view(&mut self, view: &ViewConfiguration) {
        self.views_serializer.add_view(view.clone());
    }

    pub fn set_styles_output(&mut self, styles_dsl: &str) {
        self.styles_serializer
            .set_external_output(styles_dsl.to_string());
        self.views_serializer
            .set_styles_output(styles_dsl.to_string());
    }

    pub fn set_views_styles_output(&mut self, styles_dsl: String) {
        self.views_serializer.set_styles_output(styles_dsl);
    }

    pub fn add_element_styles(&mut self, styles_dsl: &str) {
        self.styles_serializer
            .set_external_output(styles_dsl.to_string());
        self.views_serializer
            .set_styles_output(styles_dsl.to_string());
    }

    pub fn serialize(&mut self) -> Result<String, StructurizrDslError> {
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

    fn write_workspace_header(&mut self) -> Result<(), StructurizrDslError> {
        let name = self.name.as_deref().unwrap_or("Name");
        let description = self.description.as_deref().unwrap_or("Description");
        self.writer
            .add_line(&format!(r#"workspace "{}" "{}" {{"#, name, description));
        self.writer.indent();
        self.writer.add_line("!identifiers hierarchical");
        self.writer.add_empty_line();
        self.writer.add_line("model {");
        self.writer.indent();
        Ok(())
    }

    fn write_model_section(&mut self) -> Result<(), StructurizrDslError> {
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
            let dsl = Self::serialize_relationship(
                &rel.source_id,
                &rel.target_id,
                &rel.description,
                rel.technology.as_deref(),
            );
            self.writer.add_line(&dsl);
        }

        Ok(())
    }

    fn serialize_relationship(
        source_id: &str,
        target_id: &str,
        description: &str,
        technology: Option<&str>,
    ) -> String {
        if let Some(tech) = technology {
            format!(
                r#"{} -> {} "{}" "{}""#,
                source_id, target_id, description, tech
            )
        } else {
            format!(r#"{} -> {} "{}""#, source_id, target_id, description)
        }
    }

    fn serialize_person(person: &Person, identifier: &str) -> Result<String, StructurizrDslError> {
        let tags = if person.location() == crate::c4::Location::External {
            r#" {
    tags "External"
}"#
        } else {
            ""
        };
        if tags.is_empty() {
            Ok(format!(
                r#"{} = person "{}" "{}""#,
                identifier,
                person.name(),
                person.description()
            ))
        } else {
            Ok(format!(
                r#"{} = person "{}" "{}""{}"#,
                identifier,
                person.name(),
                person.description(),
                tags
            ))
        }
    }

    fn serialize_software_system(
        system: &SoftwareSystem,
        identifier: &str,
        has_containers: bool,
    ) -> String {
        let external_tag = if system.location() == crate::c4::Location::External {
            "\n    tags \"External\""
        } else {
            ""
        };
        if has_containers {
            format!(
                r#"{} = softwareSystem "{}" "{}" {{{}"#,
                identifier,
                system.name(),
                system.description(),
                external_tag
            )
        } else {
            format!(
                r#"{} = softwareSystem "{}" "{}" {{}}"#,
                identifier,
                system.name(),
                system.description()
            )
        }
    }

    fn serialize_container(
        container: &Container,
        identifier: &str,
        has_components: bool,
    ) -> String {
        if has_components {
            format!(
                r#"{} = container "{}" "{}" {{"#,
                identifier,
                container.name(),
                container.description()
            )
        } else {
            format!(
                r#"{} = container "{}" "{}" {{}}"#,
                identifier,
                container.name(),
                container.description()
            )
        }
    }

    fn serialize_component(
        component: &Component,
        identifier: &str,
    ) -> Result<String, StructurizrDslError> {
        let technology = component.technology().unwrap_or("");
        Ok(format!(
            r#"{} = component "{}" "{}" "{}""#,
            identifier,
            component.name(),
            component.description(),
            technology
        ))
    }

    fn write_views_section(&mut self) -> Result<(), StructurizrDslError> {
        let views_dsl = self.views_serializer.serialize();
        if !views_dsl.is_empty() {
            self.writer.unindent();
            self.writer.add_empty_line();
            for line in views_dsl.lines() {
                self.writer.add_line(line);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::c4::{Person, SoftwareSystem};

    #[test]
    fn test_workspace_serializer_empty() {
        let mut serializer = WorkspaceSerializer::new();
        let result = serializer.serialize().unwrap();
        println!("=== OUTPUT ===");
        println!("{}", result);
        println!("=== END ===");
        assert!(result.starts_with("workspace "));
        assert!(result.contains("!identifiers"));
        assert!(result.contains("hierarchical"));
        assert!(result.contains("model {"));
    }

    #[test]
    fn test_workspace_serializer_with_person() {
        let person = Person::builder()
            .with_name("User".try_into().unwrap())
            .with_description("A system user".try_into().unwrap())
            .build();
        let mut serializer = WorkspaceSerializer::new();
        serializer.add_person(person);
        let result = serializer.serialize().unwrap();
        assert!(result.contains("u = person"));
    }

    #[test]
    fn test_workspace_serializer_with_software_system() {
        let system = SoftwareSystem::builder()
            .with_name("Software System".try_into().unwrap())
            .with_description("Backend system".try_into().unwrap())
            .build();
        let mut serializer = WorkspaceSerializer::new();
        serializer.add_software_system(system);
        let result = serializer.serialize().unwrap();
        assert!(result.contains("ss = softwareSystem"));
    }

    #[test]
    fn test_identifier_uniqueness() {
        let mut serializer = WorkspaceSerializer::new();
        let person1 = Person::builder()
            .with_name("User".try_into().unwrap())
            .with_description("A user".try_into().unwrap())
            .build();
        let person2 = Person::builder()
            .with_name("User".try_into().unwrap())
            .with_description("Another user".try_into().unwrap())
            .build();
        serializer.add_person(person1);
        serializer.add_person(person2);
        let result = serializer.serialize().unwrap();
        assert!(result.contains("u = person"));
        assert!(result.contains("u1 = person"));
    }

    #[test]
    fn test_us1_workspace_declaration_structure() {
        let mut serializer = WorkspaceSerializer::new();
        let result = serializer.serialize().unwrap();

        assert!(
            result.starts_with("workspace "),
            "Output should start with workspace declaration"
        );
        assert!(
            result.contains("!identifiers"),
            "Output should contain !identifiers directive"
        );
        assert!(
            result.contains("hierarchical"),
            "Output should specify hierarchical identifier strategy"
        );
        assert!(
            result.contains("model {"),
            "Output should contain model section opening"
        );
    }

    #[test]
    fn test_us1_identifiers_directive_present() {
        let mut serializer = WorkspaceSerializer::new();
        let result = serializer.serialize().unwrap();

        assert!(
            result.contains("!identifiers"),
            "Output should contain !identifiers directive"
        );
        assert!(
            result.contains("hierarchical"),
            "Output should specify hierarchical identifier strategy"
        );
    }

    #[test]
    fn test_us1_workspace_with_multiple_elements() {
        let person = Person::builder()
            .with_name("User".try_into().unwrap())
            .with_description("A system user".try_into().unwrap())
            .build();

        let system = SoftwareSystem::builder()
            .with_name("API".try_into().unwrap())
            .with_description("Backend API".try_into().unwrap())
            .build();

        let mut serializer = WorkspaceSerializer::new();
        serializer.add_person(person);
        serializer.add_software_system(system);
        let result = serializer.serialize().unwrap();

        assert!(
            result.contains("u = person"),
            "First person should have 'u' identifier"
        );
        assert!(
            result.contains("a = softwareSystem"),
            "First software system should have 'a' identifier"
        );
    }

    #[test]
    fn test_us1_workspace_blocks_properly_formed() {
        let person = Person::builder()
            .with_name("User".try_into().unwrap())
            .with_description("A system user".try_into().unwrap())
            .build();

        let system = SoftwareSystem::builder()
            .with_name("API".try_into().unwrap())
            .with_description("Backend API".try_into().unwrap())
            .build();

        let mut serializer = WorkspaceSerializer::new();
        serializer.add_person(person);
        serializer.add_software_system(system);
        let result = serializer.serialize().unwrap();

        assert!(
            result.starts_with("workspace "),
            "Should contain workspace opening"
        );
        assert!(result.contains("model {"), "Should contain model opening");
        assert!(result.contains("}"), "Should contain closing braces");
    }

    #[test]
    fn test_us2_element_syntax() {
        let person = Person::builder()
            .with_name("User".try_into().unwrap())
            .with_description("A system user".try_into().unwrap())
            .build();

        let system = SoftwareSystem::builder()
            .with_name("API".try_into().unwrap())
            .with_description("Backend API".try_into().unwrap())
            .build();

        let mut serializer = WorkspaceSerializer::new();
        serializer.add_person(person);
        serializer.add_software_system(system);
        let result = serializer.serialize().unwrap();

        assert!(
            result.contains("u = person"),
            "Person should have 'u' identifier"
        );
        assert!(
            result.contains("a = softwareSystem"),
            "SoftwareSystem should have 'a' identifier"
        );
        assert!(result.contains("\"API\""), "Should contain API name");
    }

    #[test]
    fn test_us2_identifier_generation_collision() {
        let person1 = Person::builder()
            .with_name("Database".try_into().unwrap())
            .with_description("Data store".try_into().unwrap())
            .build();

        let person2 = Person::builder()
            .with_name("Developer".try_into().unwrap())
            .with_description("Software developer".try_into().unwrap())
            .build();

        let mut serializer = WorkspaceSerializer::new();
        serializer.add_person(person1);
        serializer.add_person(person2);
        let result = serializer.serialize().unwrap();

        assert!(
            result.contains("d = person \"Database\""),
            "First person 'Database' should have 'd' identifier"
        );
        assert!(
            result.contains("d1 = person \"Developer\""),
            "Second person 'Developer' should have 'd1' identifier (collision resolved)"
        );
    }

    #[test]
    fn test_us2_software_system_identifier() {
        let system = SoftwareSystem::builder()
            .with_name("API".try_into().unwrap())
            .with_description("Backend API".try_into().unwrap())
            .build();

        let mut serializer = WorkspaceSerializer::new();
        serializer.add_software_system(system);
        let result = serializer.serialize().unwrap();

        assert!(
            result.contains("a = softwareSystem"),
            "SoftwareSystem should have 'a' identifier"
        );
    }

    #[test]
    fn test_us2_multiple_software_systems() {
        let system1 = SoftwareSystem::builder()
            .with_name("API".try_into().unwrap())
            .with_description("Backend API".try_into().unwrap())
            .build();

        let system2 = SoftwareSystem::builder()
            .with_name("API".try_into().unwrap())
            .with_description("Another API".try_into().unwrap())
            .build();

        let mut serializer = WorkspaceSerializer::new();
        serializer.add_software_system(system1);
        serializer.add_software_system(system2);
        let result = serializer.serialize().unwrap();

        assert!(
            result.contains("a = softwareSystem"),
            "First system should have 'a' identifier"
        );
        assert!(
            result.contains("a1 = softwareSystem"),
            "Second system should have 'a1' identifier"
        );
    }

    #[test]
    fn test_us3_relationship_syntax() {
        let mut serializer = WorkspaceSerializer::new();
        serializer.add_relationship("u", "ss", "Uses", None);
        let result = serializer.serialize().unwrap();

        assert!(
            result.contains("u -> ss \"Uses\""),
            "Relationship should have correct syntax: source -> target \"description\""
        );
    }

    #[test]
    fn test_us3_relationship_with_technology() {
        let mut serializer = WorkspaceSerializer::new();
        serializer.add_relationship("u", "ss", "Uses", Some("HTTPS"));
        let result = serializer.serialize().unwrap();

        assert!(
            result.contains("u -> ss \"Uses\" \"HTTPS\""),
            "Relationship with technology should include technology in output"
        );
    }

    #[test]
    fn test_us3_multiple_relationships() {
        let mut serializer = WorkspaceSerializer::new();
        serializer.add_relationship("u", "a", "Uses", Some("HTTPS"));
        serializer.add_relationship("a", "d", "Queries", Some("TCP"));
        let result = serializer.serialize().unwrap();

        assert!(
            result.contains("u -> a \"Uses\" \"HTTPS\""),
            "First relationship should be present"
        );
        assert!(
            result.contains("a -> d \"Queries\" \"TCP\""),
            "Second relationship should be present"
        );
    }

    #[test]
    fn test_us3_relationship_order() {
        let mut serializer = WorkspaceSerializer::new();
        serializer.add_relationship("s2", "s1", "Depends on", None);
        serializer.add_relationship("s1", "s3", "Calls", None);
        let result = serializer.serialize().unwrap();

        let pos1 = result.find("s2 -> s1").unwrap();
        let pos2 = result.find("s1 -> s3").unwrap();
        assert!(pos1 < pos2, "Relationships should appear in order added");
    }

    #[test]
    fn test_us7_brace_balance() {
        let person = Person::builder()
            .with_name("User".try_into().unwrap())
            .with_description("A system user".try_into().unwrap())
            .build();

        let system = SoftwareSystem::builder()
            .with_name("API".try_into().unwrap())
            .with_description("Backend API".try_into().unwrap())
            .build();

        let mut serializer = WorkspaceSerializer::new();
        serializer.add_person(person);
        serializer.add_software_system(system);
        serializer.add_relationship("u", "a", "Uses", None);
        let result = serializer.serialize().unwrap();

        let opens = result.matches('{').count();
        let closes = result.matches('}').count();
        assert_eq!(
            opens, closes,
            "Braces should be balanced: {} opens, {} closes",
            opens, closes
        );
    }
}
