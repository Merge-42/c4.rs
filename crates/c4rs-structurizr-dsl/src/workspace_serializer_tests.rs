use super::*;
use c4rs_core::c4::{Person, SoftwareSystem};

#[test]
fn test_workspace_serializer_empty() {
    let result = WorkspaceSerializer::new().serialize().unwrap();
    assert!(result.starts_with("workspace "));
    assert!(result.contains("!identifiers"));
    assert!(result.contains("hierarchical"));
    assert!(result.contains("model {"));
}

#[test]
fn test_workspace_serializer_with_person() {
    let person = Person::builder()
        .name("User".into())
        .description("A system user".into())
        .build()
        .unwrap();
    let result = WorkspaceSerializer::new()
        .add_person(&person)
        .serialize()
        .unwrap();
    assert!(result.contains("u = person"));
}

#[test]
fn test_workspace_serializer_with_software_system() {
    let system = SoftwareSystem::builder()
        .name("Software System".into())
        .description("Backend system".into())
        .build()
        .unwrap();
    let result = WorkspaceSerializer::new()
        .add_software_system(&system)
        .serialize()
        .unwrap();
    assert!(result.contains("ss = softwareSystem"));
}

#[test]
fn test_identifier_uniqueness() {
    let person1 = Person::builder()
        .name("User".into())
        .description("A user".into())
        .build()
        .unwrap();
    let person2 = Person::builder()
        .name("User".into())
        .description("Another user".into())
        .build()
        .unwrap();
    let result = WorkspaceSerializer::new()
        .add_person(&person1)
        .add_person(&person2)
        .serialize()
        .unwrap();
    assert!(result.contains("u = person"));
    assert!(result.contains("u1 = person"));
}

#[test]
fn test_us1_workspace_declaration_structure() {
    let result = WorkspaceSerializer::new().serialize().unwrap();

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
    let result = WorkspaceSerializer::new().serialize().unwrap();

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
        .name("User".into())
        .description("A system user".into())
        .build()
        .unwrap();

    let system = SoftwareSystem::builder()
        .name("API".into())
        .description("Backend API".into())
        .build()
        .unwrap();

    let result = WorkspaceSerializer::new()
        .add_person(&person)
        .add_software_system(&system)
        .serialize()
        .unwrap();

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
        .name("User".into())
        .description("A system user".into())
        .build()
        .unwrap();

    let system = SoftwareSystem::builder()
        .name("API".into())
        .description("Backend API".into())
        .build()
        .unwrap();

    let result = WorkspaceSerializer::new()
        .add_person(&person)
        .add_software_system(&system)
        .serialize()
        .unwrap();

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
        .name("User".into())
        .description("A system user".into())
        .build()
        .unwrap();

    let system = SoftwareSystem::builder()
        .name("API".into())
        .description("Backend API".into())
        .build()
        .unwrap();

    let result = WorkspaceSerializer::new()
        .add_person(&person)
        .add_software_system(&system)
        .serialize()
        .unwrap();

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
        .name("Database".into())
        .description("Data store".into())
        .build()
        .unwrap();

    let person2 = Person::builder()
        .name("Developer".into())
        .description("Software developer".into())
        .build()
        .unwrap();

    let result = WorkspaceSerializer::new()
        .add_person(&person1)
        .add_person(&person2)
        .serialize()
        .unwrap();

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
        .name("API".into())
        .description("Backend API".into())
        .build()
        .unwrap();

    let result = WorkspaceSerializer::new()
        .add_software_system(&system)
        .serialize()
        .unwrap();

    assert!(
        result.contains("a = softwareSystem"),
        "SoftwareSystem should have 'a' identifier"
    );
}

#[test]
fn test_us2_multiple_software_systems() {
    let system1 = SoftwareSystem::builder()
        .name("API".into())
        .description("Backend API".into())
        .build()
        .unwrap();

    let system2 = SoftwareSystem::builder()
        .name("API".into())
        .description("Another API".into())
        .build()
        .unwrap();

    let result = WorkspaceSerializer::new()
        .add_software_system(&system1)
        .add_software_system(&system2)
        .serialize()
        .unwrap();

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
    let user = Person::builder()
        .name("User".into())
        .description("A user".into())
        .build()
        .unwrap();
    let system = SoftwareSystem::builder()
        .name("Software System".into())
        .description("A system".into())
        .build()
        .unwrap();
    let result = WorkspaceSerializer::new()
        .add_person(&user)
        .add_software_system(&system)
        .add_relationship(&user, &system, "Uses", None)
        .serialize()
        .unwrap();

    assert!(
        result.contains("u -> ss \"Uses\""),
        "Relationship should have correct syntax: source -> target \"description\""
    );
}

#[test]
fn test_us3_relationship_with_technology() {
    let user = Person::builder()
        .name("User".into())
        .description("A user".into())
        .build()
        .unwrap();
    let system = SoftwareSystem::builder()
        .name("Software System".into())
        .description("A system".into())
        .build()
        .unwrap();
    let result = WorkspaceSerializer::new()
        .add_person(&user)
        .add_software_system(&system)
        .add_relationship(&user, &system, "Uses", Some("HTTPS"))
        .serialize()
        .unwrap();

    assert!(
        result.contains("u -> ss \"Uses\" \"HTTPS\""),
        "Relationship with technology should include technology in output"
    );
}

#[test]
fn test_us3_multiple_relationships() {
    let user = Person::builder()
        .name("User".into())
        .description("A user".into())
        .build()
        .unwrap();
    let api = SoftwareSystem::builder()
        .name("API".into())
        .description("API service".into())
        .build()
        .unwrap();
    let db = SoftwareSystem::builder()
        .name("Database".into())
        .description("Data store".into())
        .build()
        .unwrap();
    let result = WorkspaceSerializer::new()
        .add_person(&user)
        .add_software_system(&api)
        .add_software_system(&db)
        .add_relationship(&user, &api, "Uses", Some("HTTPS"))
        .add_relationship(&api, &db, "Queries", Some("TCP"))
        .serialize()
        .unwrap();

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
    let alpha = SoftwareSystem::builder()
        .name("Alpha".into())
        .description("First".into())
        .build()
        .unwrap();
    let bravo = SoftwareSystem::builder()
        .name("Bravo".into())
        .description("Second".into())
        .build()
        .unwrap();
    let charlie = SoftwareSystem::builder()
        .name("Charlie".into())
        .description("Third".into())
        .build()
        .unwrap();
    let result = WorkspaceSerializer::new()
        .add_software_system(&alpha)
        .add_software_system(&bravo)
        .add_software_system(&charlie)
        .add_relationship(&bravo, &alpha, "Depends on", None)
        .add_relationship(&alpha, &charlie, "Calls", None)
        .serialize()
        .unwrap();

    let pos1 = result.find("b -> a").unwrap();
    let pos2 = result.find("a -> c").unwrap();
    assert!(pos1 < pos2, "Relationships should appear in order added");
}

#[test]
fn test_us7_brace_balance() {
    let person = Person::builder()
        .name("User".into())
        .description("A system user".into())
        .build()
        .unwrap();

    let system = SoftwareSystem::builder()
        .name("API".into())
        .description("Backend API".into())
        .build()
        .unwrap();

    let result = WorkspaceSerializer::new()
        .add_person(&person)
        .add_software_system(&system)
        .add_relationship(&person, &system, "Uses", None)
        .serialize()
        .unwrap();

    let opens = result.matches('{').count();
    let closes = result.matches('}').count();
    assert_eq!(
        opens, closes,
        "Braces should be balanced: {} opens, {} closes",
        opens, closes
    );
}

#[test]
fn test_special_characters_in_person_name() {
    let person = Person::builder()
        .name("User \"Admin\"".into())
        .description("A special user".into())
        .build()
        .unwrap();
    let result = WorkspaceSerializer::new()
        .add_person(&person)
        .serialize()
        .unwrap();
    assert!(result.contains(r#"User \"Admin\""#));
}

#[test]
fn test_special_characters_in_description() {
    let person = Person::builder()
        .name("User".into())
        .description("A \"test\" user & <admin>".into())
        .build()
        .unwrap();
    let result = WorkspaceSerializer::new()
        .add_person(&person)
        .serialize()
        .unwrap();
    assert!(result.contains(r#"\"test\""#));
}

#[test]
fn test_backslash_in_name() {
    let system = SoftwareSystem::builder()
        .name("API\\Backend".into())
        .description("Backend API".into())
        .build()
        .unwrap();
    let result = WorkspaceSerializer::new()
        .add_software_system(&system)
        .serialize()
        .unwrap();
    assert!(result.contains(r#"API\\Backend"#));
}

#[test]
fn test_relationship_with_special_chars() {
    let user = Person::builder()
        .name("User".into())
        .description("A user".into())
        .build()
        .unwrap();
    let api = SoftwareSystem::builder()
        .name("API".into())
        .description("API".into())
        .build()
        .unwrap();
    let result = WorkspaceSerializer::new()
        .add_person(&user)
        .add_software_system(&api)
        .add_relationship(&user, &api, "Uses \"HTTPS\"", Some("JSON\\API"))
        .serialize()
        .unwrap();
    assert!(result.contains(r#""Uses \"HTTPS\""#));
    assert!(result.contains(r#""JSON\\API""#));
}
