#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "tests/graphql/schema.json",
    query_path = "tests/graphql/queries/get_jwt_by_email.graphql",
)]
pub struct GetJwtByEmailMutation;

pub use self::get_jwt_by_email_mutation::*;
