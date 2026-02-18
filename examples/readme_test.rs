// This file tests the README example for create_relationship
// It should FAIL to compile

use c4rs::c4::{Container, InteractionStyle, Person, create_relationship};

fn main() {
    let user = Person::builder()
        .name("User".into())
        .description("A user".into())
        .build();

    let web_api = Container::builder()
        .name("Web API".into())
        .description("API".into())
        .container_type(c4rs::c4::ContainerType::Api)
        .build();

    // This is what the README shows (lines 116-121):
    // let relationship = create_relationship(
    //     &user,
    //     &web_api,
    //     "Uses".into(),
    //     Some(InteractionStyle::Synchronous),
    // );
    //
    // It should fail because:
    // 1. Takes 5 args, not 4
    // 2. interaction_style is Option in README but required in actual API

    // The correct call is:
    let _relationship = create_relationship(
        user,
        web_api,
        "Uses".into(),
        None,
        InteractionStyle::Synchronous,
    );
}
