#![feature(plugin)]
#![plugin(rocket_codegen)]

pub mod graphql;

extern crate rocket;
#[macro_use] extern crate juniper;
extern crate juniper_rocket;

use rocket::response::content;
use rocket::State;

use graphql::context::Context;
use graphql::schema::{Schema};

#[get("/ping")]
fn ping() -> &'static str {
    "pong"
}

#[get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
fn get_graphql_handler(
    context: State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}



fn main() {
    let context = Context {};
    let schema = graphql::schema::create();
    rocket::ignite()
        .manage(context)
        .manage(schema)
        .mount("/", routes![ping, graphiql, get_graphql_handler, post_graphql_handler]).launch();
}
