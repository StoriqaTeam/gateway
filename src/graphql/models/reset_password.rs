use uuid::Uuid;

use stq_static_resources::{Device, Project};

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Password reset request input object")]
pub struct ResetRequest {
    #[graphql(name = "clientMutationId", description = "Client mutation id.")]
    pub uuid: String,
    #[graphql(description = "Email of a user.")]
    pub email: String,
    #[graphql(description = "Device type")]
    pub device: Option<Device>,
    #[graphql(description = "Project")]
    pub project: Option<Project>,
}

impl ResetRequest {
    pub fn fill_uuid(mut self) -> Self {
        self.uuid = Some(self.uuid)
            .filter(|id| !id.is_empty())
            .unwrap_or_else(|| Uuid::new_v4().hyphenated().to_string());
        self
    }
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
    #[graphql(description = "Project")]
    pub project: Option<Project>,
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
pub struct ResetActionOutput {
    pub success: bool,
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
pub struct ResetApplyActionOutput {
    pub success: bool,
    #[graphql(description = "Reset token.")]
    pub token: String,
}
