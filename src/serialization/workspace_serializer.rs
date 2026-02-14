use crate::c4::{Container, Person, SoftwareSystem};
use crate::serialization::{
    StylesSerializer, ViewsSerializer, error::StructurizrDslError,
    identifier_generator::IdentifierGenerator, writer::DslWriter,
};
use std::collections::HashSet;

#[derive(Debug)]
pub struct WorkspaceSerializer {
    writer: DslWriter,
    used_identifiers: HashSet<String>,
    persons: Vec<Person>,
    software_systems: Vec<SoftwareSystem>,
    containers: Vec<Container>,
    views_serializer: ViewsSerializer,
    styles_serializer: StylesSerializer,
    has_configuration: bool,
    configuration_scope: Option<String>,
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
            containers: Vec::new(),
            views_serializer: ViewsSerializer::new(),
            styles_serializer: StylesSerializer::new(),
            has_configuration: false,
            configuration_scope: None,
        }
    }

    pub fn add_person(&mut self, person: Person) {
        self.persons.push(person);
    }
    pub fn add_software_system(&mut self, system: SoftwareSystem) {
        self.software_systems.push(system);
    }
    pub fn add_container(&mut self, container: Container) {
        self.containers.push(container);
    }
    pub fn set_scope(&mut self, scope: &str) {
        self.has_configuration = true;
        self.configuration_scope = Some(scope.to_string());
    }

    pub fn set_views_output(&mut self, views_dsl: String) {
        self.views_serializer.set_external_output(views_dsl);
    }

    pub fn set_styles_output(&mut self, styles_dsl: String) {
        self.styles_serializer.set_external_output(styles_dsl);
    }

    pub fn serialize(&mut self) -> Result<String, StructurizrDslError> {
        self.writer.clear();
        self.used_identifiers.clear();
        self.write_workspace_header()?;
        self.write_model_section()?;
        self.write_views_section()?;
        self.write_styles_section()?;
        self.write_configuration_section()?;
        self.writer.unindent();
        self.writer.add_line("}");
        Ok(self.writer.as_output())
    }

    fn write_workspace_header(&mut self) -> Result<(), StructurizrDslError> {
        self.writer.add_line("workspace {");
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
        let container_names: Vec<String> = self
            .containers
            .iter()
            .map(|c| c.name().to_string())
            .collect();

        for (person, name) in self.persons.iter().zip(person_names.iter()) {
            let identifier = IdentifierGenerator::generate_unique(name, &self.used_identifiers);
            self.used_identifiers.insert(identifier.clone());
            let dsl = Self::serialize_person(person, &identifier)?;
            self.writer.add_line(&dsl);
        }

        for (system, name) in self.software_systems.iter().zip(system_names.iter()) {
            let identifier = IdentifierGenerator::generate_unique(name, &self.used_identifiers);
            self.used_identifiers.insert(identifier.clone());
            let dsl = Self::serialize_software_system(system, &identifier)?;
            self.writer.add_line(&dsl);
        }

        for (container, name) in self.containers.iter().zip(container_names.iter()) {
            let identifier = IdentifierGenerator::generate_unique(name, &self.used_identifiers);
            self.used_identifiers.insert(identifier.clone());
            let dsl = Self::serialize_container(container, &identifier)?;
            self.writer.add_line(&dsl);
        }

        Ok(())
    }

    fn serialize_person(person: &Person, identifier: &str) -> Result<String, StructurizrDslError> {
        Ok(format!(
            r#"{} = person "{}" "{}""#,
            identifier,
            person.name(),
            person.description()
        ))
    }

    fn serialize_software_system(
        system: &SoftwareSystem,
        identifier: &str,
    ) -> Result<String, StructurizrDslError> {
        Ok(format!(
            r#"{} = softwareSystem "{}" "{}" {{}}"#,
            identifier,
            system.name(),
            system.description()
        ))
    }

    fn serialize_container(
        container: &Container,
        identifier: &str,
    ) -> Result<String, StructurizrDslError> {
        Ok(format!(
            r#"{} = container "{}" "{}" {{}}"#,
            identifier,
            container.name(),
            container.description()
        ))
    }

    fn write_views_section(&mut self) -> Result<(), StructurizrDslError> {
        let views_dsl = self.views_serializer.serialize();
        if !views_dsl.is_empty() {
            self.writer.unindent();
            self.writer.add_empty_line();
            self.writer.add_line(&views_dsl);
        }
        Ok(())
    }

    fn write_styles_section(&mut self) -> Result<(), StructurizrDslError> {
        let styles_dsl = self.styles_serializer.serialize();
        if !styles_dsl.is_empty() {
            self.writer.unindent();
            self.writer.add_empty_line();
            self.writer.add_line(&styles_dsl);
        }
        Ok(())
    }

    fn write_configuration_section(&mut self) -> Result<(), StructurizrDslError> {
        if self.has_configuration {
            self.writer.unindent();
            self.writer.add_empty_line();
            self.writer.add_line("configuration {");
            self.writer.indent();
            if let Some(scope) = &self.configuration_scope {
                self.writer.add_line(&format!("scope {}", scope));
            }
            self.writer.unindent();
            self.writer.add_line("}");
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
        assert!(result.contains("workspace {"));
        assert!(result.contains("!identifiers"));
        assert!(result.contains("hierarchical"));
        assert!(result.contains("model {"));
    }

    #[test]
    fn test_workspace_serializer_with_person() {
        let person = Person::builder()
            .with_name("User".try_into().unwrap())
            .with_description("A system user".try_into().unwrap())
            .build()
            .unwrap();
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
            .build()
            .unwrap();
        let mut serializer = WorkspaceSerializer::new();
        serializer.add_software_system(system);
        let result = serializer.serialize().unwrap();
        assert!(result.contains("ss = softwareSystem"));
    }

    #[test]
    fn test_workspace_serializer_with_configuration() {
        let mut serializer = WorkspaceSerializer::new();
        serializer.set_scope("softwaresystem");
        let result = serializer.serialize().unwrap();
        assert!(result.contains("configuration {"));
        assert!(result.contains("scope softwaresystem"));
    }

    #[test]
    fn test_identifier_uniqueness() {
        let mut serializer = WorkspaceSerializer::new();
        let person1 = Person::builder()
            .with_name("User".try_into().unwrap())
            .with_description("A user".try_into().unwrap())
            .build()
            .unwrap();
        let person2 = Person::builder()
            .with_name("User".try_into().unwrap())
            .with_description("Another user".try_into().unwrap())
            .build()
            .unwrap();
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
            result.contains("workspace {"),
            "Output should contain workspace block opening"
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
            .build()
            .unwrap();

        let system = SoftwareSystem::builder()
            .with_name("API".try_into().unwrap())
            .with_description("Backend API".try_into().unwrap())
            .build()
            .unwrap();

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
            .build()
            .unwrap();

        let system = SoftwareSystem::builder()
            .with_name("API".try_into().unwrap())
            .with_description("Backend API".try_into().unwrap())
            .build()
            .unwrap();

        let mut serializer = WorkspaceSerializer::new();
        serializer.add_person(person);
        serializer.add_software_system(system);
        let result = serializer.serialize().unwrap();

        assert!(
            result.contains("workspace {"),
            "Should contain workspace opening"
        );
        assert!(result.contains("model {"), "Should contain model opening");
        assert!(result.contains("}"), "Should contain closing braces");
    }
}
