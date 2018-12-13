use std::time::SystemTime;

use uuid::Uuid;

use stq_static_resources::TokenType;

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Get existing reset token request input object")]
pub struct ExistingResetTokenInput {
    #[graphql(description = "user")]
    pub user_id: i32,
    #[graphql(description = "token type")]
    pub token_type: TokenTypeInput,
}

#[derive(GraphQLEnum, Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[graphql(description = "Token type input object")]
pub enum TokenTypeInput {
    EmailVerify,
    PasswordReset,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResetToken {
    pub token: String,
    pub email: String,
    pub created_at: SystemTime,
    pub token_type: TokenType,
    pub uuid: Uuid,
    pub updated_at: SystemTime,
}
