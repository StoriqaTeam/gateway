#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "tests/graphql/schema.json",
    query_path = "tests/graphql/queries/create_store.graphql",
)]
pub struct CreateStoreMutation;

pub use self::create_store_mutation::*;
