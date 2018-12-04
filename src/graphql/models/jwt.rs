use std::fmt;
use std::fmt::Display;

use juniper::Value;

use graphql::models::{NewUserAdditionalData, NewUserAdditionalDataInput};
use stq_static_resources::Provider;
use stq_types::UserId;

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

    from_input_value(_v: &InputValue) -> Option<UserStatus> {
        None
    }
});

#[derive(Serialize, Deserialize, Debug)]
pub enum UserStatus {
    New(UserId),
    Exists,
}

/// Payload for creating JWT token by provider
#[derive(Serialize, Deserialize)]
pub struct ProviderOauth {
    pub token: String,
    pub additional_data: Option<NewUserAdditionalData>,
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
    #[graphql(description = "Additional data containing referal, referer, country, utm_marks, etc... .")]
    pub additional_data: Option<NewUserAdditionalDataInput>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JWTPayload {
    pub user_id: UserId,
    pub exp: i64,
    pub provider: Provider,
}

impl Display for JWTPayload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.user_id.fmt(f)
    }
}
