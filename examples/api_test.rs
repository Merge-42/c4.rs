use c4rs::c4::create_relationship;
use c4rs::c4::value_types::NonEmptyString;
use c4rs::c4::{
    CodeElement, CodeType, Component, Container, ContainerType, InteractionStyle, Location, Person,
    Relationship, SoftwareSystem,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== Testing NonEmptyString ergonomics ===\n");

    let name = NonEmptyString::from("Alice");
    println!("NonEmptyString::from(\"Alice\") = {:?}", name);

    let name2: NonEmptyString = "Bob".into();
    println!("\"Bob\".into() = {:?}", name2);

    let name3 = NonEmptyString::new("Charlie").unwrap();
    println!("NonEmptyString::new(\"Charlie\").unwrap() = {:?}", name3);

    println!("\nTesting empty string with .into()...");
    let result = std::panic::catch_unwind(|| {
        let _: NonEmptyString = "".into();
    });
    match result {
        Ok(_) => println!("ERROR: Should have panicked!"),
        Err(_) => println!("Confirmed: .into() panics on empty string"),
    }

    println!("\n=== Testing create_relationship signature ===\n");

    let person = Person::builder()
        .name("User".into())
        .description("A user".into())
        .build();

    let container = Container::builder()
        .name("API".into())
        .description("REST API".into())
        .container_type(ContainerType::Api)
        .build();

    let rel = create_relationship(
        person.clone(),
        container.clone(),
        "Uses".into(),
        Some("HTTP".into()),
        InteractionStyle::Synchronous,
    );
    println!("create_relationship with all 5 args: OK");
    println!("  description: {}", rel.description());
    println!("  technology: {:?}", rel.technology());
    println!("  interaction_style: {:?}", rel.interaction_style());

    println!("\n=== Testing Relationship builder ===\n");

    let rel3 = Relationship::<Person, Container>::builder()
        .source(person.clone())
        .target(container.clone())
        .description("Calls".into())
        .interaction_style(InteractionStyle::Asynchronous)
        .build();
    println!("Relationship builder: OK");
    println!("  description: {}", rel3.description());
    println!("  interaction_style: {:?}", rel3.interaction_style());

    println!("\n=== Testing Person builder ===\n");

    let person2 = Person::builder()
        .name("Alice".into())
        .description("Admin".into())
        .location(Location::Internal)
        .build();
    println!(
        "Person: name={}, description={}, location={}",
        person2.name(),
        person2.description(),
        person2.location()
    );

    println!("\n=== Testing SoftwareSystem with containers ===\n");

    let system = SoftwareSystem::builder()
        .name("E-Commerce".into())
        .description("Online store".into())
        .add_container(
            Container::builder()
                .name("Web".into())
                .description("Frontend".into())
                .container_type(ContainerType::WebApplication)
                .build(),
        )
        .build();
    println!(
        "SoftwareSystem: name={}, containers={}",
        system.name(),
        system.containers().len()
    );

    println!("\n=== Testing Component ===\n");

    let component = Component::builder()
        .name("OrderHandler".into())
        .description("Handles orders".into())
        .responsibilities(vec!["Create order".into(), "Cancel order".into()])
        .technology("Rust".into())
        .build();
    println!(
        "Component: name={}, responsibilities={}",
        component.name(),
        component.responsibilities().len()
    );

    println!("\n=== Testing CodeElement (optional fields) ===\n");

    let code = CodeElement::builder()
        .name("calculateTotal".into())
        .description("Calculates total".into())
        .code_type(CodeType::Function)
        .language("Rust".into())
        .file_path("src/orders.rs".into())
        .build();
    println!(
        "CodeElement: name={}, language={:?}",
        code.name(),
        code.language()
    );

    println!("\n=== Testing cross-level relationships ===\n");

    let _rel_pc: Relationship<Person, Container> = Relationship::builder()
        .source(person.clone())
        .target(container.clone())
        .description("Uses".into())
        .build();
    println!("Person -> Container: OK");

    let container2 = Container::builder()
        .name("Service".into())
        .description("Service".into())
        .container_type(ContainerType::Api)
        .build();
    let component2 = Component::builder()
        .name("Handler".into())
        .description("Handler".into())
        .build();
    let _rel_cc: Relationship<Container, Component> = Relationship::builder()
        .source(container2)
        .target(component2)
        .description("Contains".into())
        .build();
    println!("Container -> Component: OK");

    println!("\n=== All tests passed! ===\n");

    Ok(())
}
