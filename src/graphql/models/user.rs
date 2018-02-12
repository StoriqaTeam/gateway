use super::gender::Gender;
use juniper::ID as GraphqlID;

#[derive(Deserialize, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub is_active: bool,
    pub phone: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub middle_name: Option<String>,
    pub gender: Gender,
    pub birthdate: Option<String>,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Debug, Clone)]
pub struct UpdateUser {
    #[graphql(description="Activate/deactivate user.")]
    pub is_active: Option<bool>,
    #[graphql(description="New phone of a user")]
    pub phone: Option<String>,
    #[graphql(description="New first name of a user")]
    pub first_name: Option<String>,
    #[graphql(description="New last name of a user")]
    pub last_name: Option<String>,
    #[graphql(description="New middle name of a user")]
    pub middle_name: Option<String>,
    #[graphql(description="Gender of a user")]
    pub gender: Option<Gender>,
    #[graphql(description="Birthdate of a user")]
    pub birthdate: Option<String>,
}


#[derive(GraphQLInputObject, Debug, Clone)]
#[graphql(description="Update user input object")]
pub struct UpdateUserInput {
    #[graphql(description="Client mutation id.")]
    pub client_mutation_id: String,
    #[graphql(description="Input field update user.")]
    pub input_fields: UpdateUserWithIdInput
}

#[derive(GraphQLInputObject, Debug, Clone)]
#[graphql(description="Update user with id input object")]
pub struct UpdateUserWithIdInput {
    #[graphql(description="Id of a user.")]
    pub id: GraphqlID,
    #[graphql(description="Input field update user.")]
    pub update_user: UpdateUser
}


#[derive(GraphQLInputObject, Serialize, Deserialize, Debug, Clone)]
#[graphql(description="New user")]
pub struct NewUser {
    #[graphql(description="Email of a user.")]
    pub email: String,
    #[graphql(description="Password of a user.")]
    pub password: String
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Debug, Clone)]
#[graphql(description="Create user input object")]
pub struct CreateUserInput {
    #[graphql(description="Client mutation id.")]
    pub client_mutation_id: String,
    #[graphql(description="Input field new user.")]
    pub input_fields: NewUser
}

#[derive(GraphQLInputObject, Debug, Clone)]
#[graphql(description="Delete user")]
pub struct DeleteUser {
    #[graphql(description="Email of a user.")]
    pub id: GraphqlID,
}

#[derive(GraphQLInputObject, Debug, Clone)]
#[graphql(description="Deactivate user input object")]
pub struct DeactivateUserInput {
    #[graphql(description="Client mutation id.")]
    pub client_mutation_id: String,
    #[graphql(description="Input field Delete user.")]
    pub input_fields: DeleteUser
}