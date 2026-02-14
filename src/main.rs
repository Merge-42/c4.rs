use c4rs::c4::{Component, Container, ContainerType, Person, SoftwareSystem};
use c4rs::serialization::StructurizrDslSerializer;

fn main() {
    // Create a C4 model
    let person = Person::builder()
        .with_name("User".into())
        .with_description("A user of the system".into())
        .build()
        .unwrap();

    let system = SoftwareSystem::builder()
        .with_name("API".into())
        .with_description("Backend API service".into())
        .build()
        .unwrap();

    let container = Container::builder()
        .with_name("Web App".into())
        .with_description("Frontend application".into())
        .with_container_type(ContainerType::WebApplication)
        .build()
        .unwrap();

    let _component = Component::builder()
        .with_name("UserController".into())
        .with_description("Handles user requests".into())
        .with_technology("Rust".into())
        .build()
        .unwrap();

    // Serialize to Structurizr DSL
    let mut serializer = StructurizrDslSerializer::new();
    serializer.add_person(person);
    serializer.add_software_system(system);
    serializer.add_container("API", container);
    // Note: Component is not directly added - it belongs to a Container
    let dsl = serializer.serialize().unwrap();

    println!("Structurizr DSL Output:\n");
    println!("{}", dsl);
}
