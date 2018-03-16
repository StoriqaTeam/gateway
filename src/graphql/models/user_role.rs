#[derive(GraphQLEnum, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[graphql(description = "User roles enum")]
pub enum Role {
    Superuser,
    User,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[graphql(description = "New user role input object")]
pub struct NewUserRoleInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "User id")]
    pub user_id: i32,
    #[graphql(description = "New user Role")]
    pub role: Role,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash )]
pub struct UserRoles {
    pub user_id: i32,
    pub roles: Vec<Role>,
}