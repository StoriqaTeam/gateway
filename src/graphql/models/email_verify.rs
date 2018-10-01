#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Email verify input object")]
pub struct VerifyEmailResend {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Email of a user.")]
    pub email: String,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Email verify apply input object")]
pub struct VerifyEmailApply {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Email verification token.")]
    pub token: String,
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
pub struct VerifyEmailResendOutput {
    pub success: bool,
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
pub struct VerifyEmailApplyOutput {
    pub success: bool,
    #[graphql(description = "Email verification token.")]
    pub token: String,
}
