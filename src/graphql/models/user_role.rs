use stq_types::{RoleId, StoreId, UserId};

#[derive(GraphQLEnum, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[graphql(description = "Users microservice role")]
pub enum UserMicroserviceRole {
    Superuser,
    Moderator,
    User,
}

#[derive(GraphQLEnum, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[graphql(description = "Stores microservice role")]
pub enum StoresMicroserviceRole {
    Superuser,
    Moderator,
    PlatformAdmin,
    User,
}

#[derive(GraphQLEnum, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[graphql(description = "Billing microservice role")]
pub enum BillingMicroserviceRole {
    Superuser,
    User,
    StoreManager,
    FinancialManager,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[graphql(description = "New role input object")]
pub struct NewUsersRoleInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "User id")]
    pub user_id: i32,
    #[graphql(description = "New Role")]
    pub name: UserMicroserviceRole,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[graphql(description = "Remove role input object")]
pub struct RemoveUsersRoleInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "User id")]
    pub user_id: i32,
    #[graphql(description = "Removed Role")]
    pub name: UserMicroserviceRole,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[graphql(description = "New role input object")]
pub struct NewStoresRoleInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "User id")]
    pub user_id: i32,
    #[graphql(description = "New Role")]
    pub name: StoresMicroserviceRole,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[graphql(description = "Remove role input object")]
pub struct RemoveStoresRoleInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "User id")]
    pub user_id: i32,
    #[graphql(description = "Removed Role")]
    pub name: StoresMicroserviceRole,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[graphql(description = "New role input object")]
pub struct NewBillingRoleInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "User id")]
    pub user_id: i32,
    #[graphql(description = "New Role")]
    pub name: BillingMicroserviceRole,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[graphql(description = "Remove role input object")]
pub struct RemoveBillingRoleInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "User id")]
    pub user_id: i32,
    #[graphql(description = "Removed Role")]
    pub name: BillingMicroserviceRole,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewRole<Role> {
    pub id: RoleId,
    pub user_id: UserId,
    pub name: Role,
    pub data: Option<StoreId>,
}
