#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "tests/graphql/schema.json",
    query_path = "tests/graphql/queries/get_jwt_by_provider.graphql",
)]
pub struct GetJwtByProviderMutation;

pub use self::get_jwt_by_provider_mutation::*;
