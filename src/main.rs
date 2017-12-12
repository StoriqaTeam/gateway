#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate juniper;
extern crate juniper_rocket;

use rocket::response::content;
use rocket::State;

use juniper::{FieldResult};

#[derive(GraphQLEnum)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

#[derive(GraphQLObject)]
#[graphql(description="A humanoid creature in the Star Wars universe")]
struct Human {
    id: String,
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

// There is also a custom derive for mapping GraphQL input objects.

#[derive(GraphQLInputObject)]
#[graphql(description="A humanoid creature in the Star Wars universe")]
struct NewHuman {
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

// Now, we create our root Query and Mutation types with resolvers by using the
// graphql_object! macro.
// Objects can have contexts that allow accessing shared state like a database
// pool.

// Graphql context
struct Context;

// To make our context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for Context {}

struct Query;

graphql_object!(Query: Context |&self| {

    field apiVersion() -> &str {
        "1.0"
    }

    field human(&executor, id: String) -> FieldResult<Human> {
        let human = Human {
            id: String::from("1"),
            name: String::from("Luke"),
            appears_in: vec![Episode::NewHope],
            home_planet: String::from("Tatuin")
        };
        Ok(human)
    }
});

struct Mutation;

graphql_object!(Mutation: Context |&self| {
    // field createHuman(&executor, new_human: NewHuman) -> FieldResult<Human> {
    //     let db = executor.context().pool.get_connection()?;
    //     let human: Human = db.insert_human(&new_human)?;
    //     Ok(human)
    // }
});

// A root schema consists of a query and a mutation.
// Request queries can be executed against a RootNode.
type Schema = juniper::RootNode<'static, Query, Mutation>;

#[get("/ping")]
fn ping() -> &'static str {
    "Pong!"
}

#[get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

struct Database;

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
    let query = Query {};
    let mutation = Mutation {};
    rocket::ignite()
        .manage(context)
        .manage(Schema::new(
            query,
            mutation
        ))
        .mount("/", routes![ping, graphiql, get_graphql_handler, post_graphql_handler]).launch();
}
