use super::Provider;
use juniper::Value;

#[derive(GraphQLObject, Deserialize, Debug)]
#[graphql(description = "JWT Token")]
pub struct JWT {
    #[graphql(description = "Token")]
    pub token: String,

    #[graphql(deprecation = "Not in use")]
    pub status: UserStatus,
}

graphql_scalar!(UserStatus {
    description: "DEPRACATED"

    resolve(&self) -> Value {
        Value::string("deprecated!!!!")
    }

    from_input_value(v: &InputValue) -> Option<UserStatus> {
        None
    }
});

#[derive(Serialize, Deserialize, Debug)]
pub enum UserStatus {
    New(i32),
    Exists,
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
