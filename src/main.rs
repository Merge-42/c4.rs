use c4rs::c4::{Container, ContainerType, Person, SoftwareSystem};
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
        .add_container(
            Container::builder()
                .name("Web App".into())
                .description("Frontend application".into())
                .container_type(ContainerType::WebApplication)
                .build(),
        )
        .add_container(
            Container::builder()
                .name("Database".into())
                .description("PostgreSQL database".into())
                .container_type(ContainerType::Database)
                .technology(Some("PostgreSQL 15".into()))
                .build(),
        )
        .add_container(
            Container::builder()
                .name("API Service".into())
                .description("Backend API".into())
                .container_type(ContainerType::Api)
                .build(),
        )
        .build();

    let web_system = SoftwareSystem::builder()
        .name("Web Portal".into())
        .description("Customer web portal".into())
        .add_container(
            Container::builder()
                .name("Frontend".into())
                .description("React frontend".into())
                .container_type(ContainerType::WebApplication)
                .build(),
        )
        .build();

    use c4rs::serialization::views_serializer::ViewType;

    let ctx_view = c4rs::serialization::views_serializer::ViewConfiguration::builder()
        .view_type(ViewType::SystemContext)
        .element_identifier("a".to_string())
        .title("SystemContext".to_string())
        .include_elements(vec!["*".to_string()])
        .build();

    let dsl = StructurizrDslSerializer::new()
        .with_name("Example System")
        .with_description("An example C4 model")
        .add_person(person)
        .add_software_system(api_system)
        .add_software_system(web_system)
        .add_relationship("u", "a", "Uses", None)
        .add_view(ctx_view)
        .add_element_style(
            c4rs::serialization::styles_serializer::ElementStyle::new("Person")
                .with_shape("person"),
        )
        .add_element_style(
            c4rs::serialization::styles_serializer::ElementStyle::new("Database")
                .with_shape("cylinder"),
        )
        .serialize()?;

    println!("Structurizr DSL Output:\n");
    println!("{}", dsl);

    Ok(())
}
