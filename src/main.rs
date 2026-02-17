use c4rs::c4::{Component, Container, ContainerType, Person, SoftwareSystem};
use c4rs::serialization::StructurizrDslSerializer;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let person = Person::builder()
        .name("User".into())
        .description("A user of the system".into())
        .build();

    let api_system = SoftwareSystem::builder()
        .name("API".into())
        .description("Backend API service".into())
        .containers(vec![
            Container::builder()
                .with_name("Web App".into())
                .with_description("Frontend application".into())
                .with_container_type(ContainerType::WebApplication)
                .build(),
            Container::builder()
                .with_name("Database".into())
                .with_description("PostgreSQL database".into())
                .with_container_type(ContainerType::Database)
                .with_technology("PostgreSQL 15".into())
                .build(),
            Container::builder()
                .with_name("API Service".into())
                .with_description("Backend API".into())
                .with_container_type(ContainerType::Api)
                .add_component(
                    Component::builder()
                        .with_name("UserController".into())
                        .with_description("User handling".into())
                        .with_technology("Rust".into())
                        .build(),
                )
                .add_component(
                    Component::builder()
                        .with_name("OrderController".into())
                        .with_description("Order handling".into())
                        .with_technology("Rust".into())
                        .build(),
                )
                .build(),
        ])
        .build();

    let web_system = SoftwareSystem::builder()
        .name("Web Portal".into())
        .description("Customer web portal".into())
        .containers(vec![
            Container::builder()
                .with_name("Frontend".into())
                .with_description("React frontend".into())
                .with_container_type(ContainerType::WebApplication)
                .build(),
        ])
        .build();

    let mut serializer = StructurizrDslSerializer::new()
        .with_name("Example System")
        .with_description("An example C4 model");
    serializer.add_person(person);
    serializer.add_software_system(api_system);
    serializer.add_software_system(web_system);

    serializer.add_relationship("u", "a", "Uses", None);

    use c4rs::serialization::views_serializer::ViewType;

    let mut ctx_view = c4rs::serialization::views_serializer::ViewConfiguration::new(
        ViewType::SystemContext,
        "a",
        "SystemContext",
    );
    ctx_view.include_element("*");
    serializer.add_view(&ctx_view);

    serializer.add_element_style(
        c4rs::serialization::styles_serializer::ElementStyle::new("Person").with_shape("person"),
    );
    serializer.add_element_style(
        c4rs::serialization::styles_serializer::ElementStyle::new("Database")
            .with_shape("cylinder"),
    );

    let dsl = serializer.serialize()?;

    println!("Structurizr DSL Output:\n");
    println!("{}", dsl);

    Ok(())
}
