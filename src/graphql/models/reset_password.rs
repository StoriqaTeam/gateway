#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Password reset request input object")]
pub struct ResetRequest {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Email of a user.")]
    pub email: String,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Password reset apply input object")]
pub struct ResetApply {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Reset token.")]
    pub token: String,
    #[graphql(description = "Password of a user.")]
    pub password: String,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Debug, Clone)]
pub struct ResetActionResp {
    pub success: bool
}
