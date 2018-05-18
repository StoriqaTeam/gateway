use chrono::NaiveDate;
use juniper::ID as GraphqlID;
use juniper::{FieldError, FieldResult};

use super::Gender;
use super::Provider;

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
    pub avatar: Option<String>,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
#[graphql(description = "Update user input object")]
pub struct UpdateUserInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Id of a user.")]
    #[serde(skip_serializing)]
    pub id: GraphqlID,
    #[graphql(description = "Activate/deactivate user.")]
    pub is_active: Option<bool>,
    #[graphql(description = "New phone of a user")]
    pub phone: Option<String>,
    #[graphql(description = "New first name of a user")]
    pub first_name: Option<String>,
    #[graphql(description = "New last name of a user")]
    pub last_name: Option<String>,
    #[graphql(description = "New middle name of a user")]
    pub middle_name: Option<String>,
    #[graphql(description = "Gender of a user")]
    pub gender: Option<Gender>,
    #[graphql(description = "Birthdate of a user")]
    pub birthdate: Option<String>,
    #[graphql(description = "Avatar link of a user")]
    pub avatar: Option<String>,
}

impl UpdateUserInput {
    pub fn is_none(&self) -> bool {
        Self {
            client_mutation_id: self.client_mutation_id.clone(),
            id: self.id.clone(),
            is_active: None,
            phone: None,
            first_name: None,
            last_name: None,
            middle_name: None,
            gender: None,
            birthdate: None,
            avatar: None,
        } == self.clone()
    }

    pub fn validate(&self) -> FieldResult<()> {
        if let Some(birthdate) = self.birthdate.clone() {
            NaiveDate::parse_from_str(&birthdate, "%Y-%m-%d")
                .map(|_| ())
                .map_err(|_| {
                    FieldError::new(
                        "Error response from microservice",
                        graphql_value!({ "code": 100, "details": {
                            "status": "400 Bad Request",
                            "code": "400",
                            "message":
                                "{\"birthdate\":[{\"code\":\"birthdate\",\"message\":\"Incorrect birthdate format\",\"params\":{\"value\":\"\"}}]}"
                            ,
                        }}),
                    )
                })
        } else {
            Ok(())
        }
    }
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Create user input object")]
pub struct CreateUserInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Email of a user.")]
    pub email: String,
    #[graphql(description = "Password of a user.")]
    pub password: String,
    #[graphql(description = "First name of a user")]
    pub first_name: String,
    #[graphql(description = "Last name of a user")]
    pub last_name: String,
}

#[derive(Serialize, Clone)]
pub struct NewUser {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

/// Payload for creating identity
#[derive(Serialize, Clone)]
pub struct NewIdentity {
    pub email: String,
    pub password: String,
    pub provider: Provider,
    pub saga_id: String,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Change user password input object")]
pub struct ChangePasswordInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Previous password of a user.")]
    pub old_password: String,
    #[graphql(description = "New password of a user.")]
    pub new_password: String,
}

/// Payload for creating identity
#[derive(Serialize, Clone)]
pub struct SagaCreateProfile {
    pub identity: NewIdentity,
    pub user: Option<NewUser>,
}

#[derive(GraphQLInputObject, Debug, Clone)]
#[graphql(description = "Deactivate user input object")]
pub struct DeactivateUserInput {
    #[graphql(description = "Client mutation id.")]
    pub client_mutation_id: String,
    #[graphql(description = "id of a user.")]
    pub id: GraphqlID,
}
