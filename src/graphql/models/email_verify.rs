#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Password reset request input object")]
pub struct VerifyEmailResend {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Email of a user.")]
    pub email: String,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Password reset apply input object")]
pub struct VerifyEmailApply {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Reset token.")]
    pub token: String,
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
pub struct VerifyEmailOutput {
    pub success: bool,
}
