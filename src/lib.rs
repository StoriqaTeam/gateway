#![feature(plugin)]
#![plugin(rocket_codegen)]

pub mod api;
pub mod schema;
pub mod context;

#[macro_use]
extern crate juniper;
extern crate juniper_rocket;
extern crate rocket;




/// Constructs a new Rocket instance.
///
/// This function takes care of attaching all routes and handlers of the application.
pub fn rocket_factory() -> Result<rocket::Rocket, String> {
    let context = context::Context {};
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
