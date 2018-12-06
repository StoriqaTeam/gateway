#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "tests/graphql/schema.json",
    query_path = "tests/graphql/queries/create_user.graphql",
)]
pub struct CreateUserMutation;

pub use self::create_user_mutation::*;
