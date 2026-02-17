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
        .containers(vec![
            Container::builder()
                .name("Web App".into())
                .description("Frontend application".into())
                .container_type(ContainerType::WebApplication)
                .build(),
            Container::builder()
                .name("Database".into())
                .description("PostgreSQL database".into())
                .container_type(ContainerType::Database)
                .technology(Some("PostgreSQL 15".into()))
                .build(),
            Container::builder()
                .name("API Service".into())
                .description("Backend API".into())
                .container_type(ContainerType::Api)
                .build(),
        ])
        .build();

    let web_system = SoftwareSystem::builder()
        .name("Web Portal".into())
        .description("Customer web portal".into())
        .containers(vec![
            Container::builder()
                .name("Frontend".into())
                .description("React frontend".into())
                .container_type(ContainerType::WebApplication)
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

    let ctx_view = c4rs::serialization::views_serializer::ViewConfiguration::builder()
        .view_type(ViewType::SystemContext)
        .element_identifier("a".to_string())
        .title("SystemContext".to_string())
        .include_elements(vec!["*".to_string()])
        .build();
    serializer.add_view(ctx_view);

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
