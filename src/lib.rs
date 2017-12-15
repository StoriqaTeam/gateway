#![feature(plugin)]
#![plugin(rocket_codegen)]

pub mod api;
pub mod schema;
pub mod helper;

#[macro_use]
extern crate juniper;
extern crate juniper_rocket;
extern crate rocket;




pub fn rocket_factory(config: String) -> Result<rocket::Rocket, String> {
    let mut context = helper::microservices::Microservices::new();
    context.apply(config);
    let schema = schema::create();
    let rocket = rocket::ignite().manage(context).manage(schema).mount(
        "/",
        routes![
            api::graph::ping,
            api::graph::graphql,
            api::graph::get_graphql_handler,
            api::graph::post_graphql_handler
        ],
    );

    Ok(rocket)
}
