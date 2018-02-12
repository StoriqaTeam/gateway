use super::Provider;

#[derive(GraphQLObject, Deserialize, Debug)]
#[graphql(description = "JWT Token")]
pub struct JWT {
    #[graphql(description = "Token")]
    pub token: String,
}

/// Payload for creating JWT token by provider
#[derive(Serialize, Deserialize)]
pub struct ProviderOauth {
    pub token: String,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Debug, Clone)]
#[graphql(description = "Create jwt by email input object")]
pub struct CreateJWTEmailInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Email of a user.")]
    pub email: String,
    #[graphql(description = "Password of a user.")]
    pub password: String,
}

#[derive(GraphQLInputObject, Debug, Clone)]
#[graphql(description = "Create jwt by provider input object")]
pub struct CreateJWTProviderInput {
    #[graphql(description = "Client mutation id.")]
    pub client_mutation_id: String,
    #[graphql(description = "Token Provider.")]
    pub provider: Provider,
    #[graphql(description = "Token recevied from provider.")]
    pub token: String,
}
