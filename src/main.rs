use c4rs::c4::{Component, Container, ContainerType, Person, SoftwareSystem};
use c4rs::serialization::StructurizrDslSerializer;

fn main() {
    // Create a C4 model
    let person = Person::builder()
        .with_name("User".into())
        .with_description("A user of the system".into())
        .build()
        .unwrap();

    // First software system with containers
    let api_system = SoftwareSystem::builder()
        .with_name("API".into())
        .with_description("Backend API service".into())
        .build()
        .unwrap();

    // Container in the API system
    let api_container = Container::builder()
        .with_name("Web App".into())
        .with_description("Frontend application".into())
        .with_container_type(ContainerType::WebApplication)
        .build()
        .unwrap();

    // Another container in the API system
    let db_container = Container::builder()
        .with_name("Database".into())
        .with_description("PostgreSQL database".into())
        .with_container_type(ContainerType::Database)
        .with_technology("PostgreSQL 15".into())
        .build()
        .unwrap();

    // Second software system (separate from API)
    let web_system = SoftwareSystem::builder()
        .with_name("Web Portal".into())
        .with_description("Customer web portal".into())
        .build()
        .unwrap();

    // Container in the Web Portal system
    let portal_frontend = Container::builder()
        .with_name("Frontend".into())
        .with_description("React frontend".into())
        .with_container_type(ContainerType::WebApplication)
        .build()
        .unwrap();

    // Container WITH components (using builder pattern)
    let api_container_with_components = Container::builder()
        .with_name("API Service".into())
        .with_description("Backend API".into())
        .with_container_type(ContainerType::Api)
        .add_component(
            Component::builder()
                .with_name("UserController".into())
                .with_description("User handling".into())
                .with_technology("Rust".into())
                .build()
                .unwrap(),
        )
        .add_component(
            Component::builder()
                .with_name("OrderController".into())
                .with_description("Order handling".into())
                .with_technology("Rust".into())
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    // Serialize to Structurizr DSL
    let mut serializer = StructurizrDslSerializer::new();
    serializer.add_person(person);

    // Add first software system with its containers
    serializer.add_software_system(api_system);
    serializer.add_container("API", api_container);
    serializer.add_container("API", db_container);
    serializer.add_container("API", api_container_with_components);

    // Add second software system with its container
    serializer.add_software_system(web_system);
    serializer.add_container("Web Portal", portal_frontend);

    let dsl = serializer.serialize().unwrap();

    println!("Structurizr DSL Output:\n");
    println!("{}", dsl);
}
