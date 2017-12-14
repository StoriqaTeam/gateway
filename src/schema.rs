use juniper;
use juniper::{EmptyMutation, FieldResult};

use super::context::Context;

pub struct Query;

#[derive(GraphQLEnum)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct Human {
    id: String,
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

// There is also a custom derive for mapping GraphQL input objects.

// #[derive(GraphQLInputObject)]
// #[graphql(description="A humanoid creature in the Star Wars universe")]
// struct NewHuman {
//     name: String,
//     appears_in: Vec<Episode>,
//     home_planet: String,
// }

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

// struct Mutation;

// graphql_object!(Mutation: Context |&self| {
// field createHuman(&executor, new_human: NewHuman) -> FieldResult<Human> {
//     let db = executor.context().pool.get_connection()?;
//     let human: Human = db.insert_human(&new_human)?;
//     Ok(human)
// }
// });

// A root schema consists of a query and a mutation.
// Request queries can be executed against a RootNode.
pub type Schema = juniper::RootNode<'static, Query, EmptyMutation<Context>>;

pub fn create() -> Schema {
    let query = Query {};
    let mutation = EmptyMutation::new();
    Schema::new(query, mutation)
}
