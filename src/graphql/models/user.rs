use std::collections::HashMap;
use std::time::SystemTime;

use chrono::NaiveDate;
use juniper::ID as GraphqlID;
use juniper::{FieldError, FieldResult};

use stq_static_resources::{Device, Gender, Project, Provider};
use stq_types::{Alpha3, SagaId, UserId};

#[derive(Deserialize, Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub email: String,
    pub is_active: bool,
    pub phone: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub middle_name: Option<String>,
    pub gender: Option<Gender>,
    pub birthdate: Option<String>,
    pub avatar: Option<String>,
    pub is_blocked: bool,
    pub emarsys_id: Option<i32>,
    pub referal: Option<UserId>,
    pub utm_marks: Option<HashMap<String, String>>,
    pub country: Option<Alpha3>,
    pub referer: Option<String>,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UtmMark {
    pub key: String,
    pub value: String,
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
            NaiveDate::parse_from_str(&birthdate, "%Y-%m-%d").map(|_| ()).map_err(|_| {
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
    #[graphql(description = "Device")]
    pub device: Option<Device>,
    #[graphql(description = "Project")]
    pub project: Option<Project>,
    #[graphql(description = "Additional data containing referal, referer, country, utm_marks, etc... .")]
    pub additional_data: Option<NewUserAdditionalDataInput>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewUser {
    pub email: String,
    pub phone: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub middle_name: Option<String>,
    pub gender: Option<Gender>,
    pub birthdate: Option<NaiveDate>,
    pub last_login_at: SystemTime,
    pub saga_id: SagaId,
    pub referal: Option<UserId>,
    pub utm_marks: Option<HashMap<String, String>>,
    pub country: Option<String>,
    pub referer: Option<String>,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone, Default)]
#[graphql(description = "Additional for user creation input object")]
pub struct NewUserAdditionalDataInput {
    #[graphql(description = "Raw user id who advertised the project.")]
    pub referal: Option<i32>,
    #[graphql(description = "Collection of marketing utm marks.")]
    pub utm_marks: Option<Vec<UtmMarkInput>>,
    #[graphql(description = "Alpha 3 country code of a user.")]
    pub country: Option<String>,
    #[graphql(description = "Referer application domain.")]
    pub referer: Option<String>,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone, Default)]
#[graphql(description = "Single utm key-value pair")]
pub struct UtmMarkInput {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NewUserAdditionalData {
    pub referal: Option<i32>,
    pub utm_marks: Option<HashMap<String, String>>,
    pub country: Option<String>,
    pub referer: Option<String>,
}

impl Into<NewUserAdditionalData> for NewUserAdditionalDataInput {
    fn into(self) -> NewUserAdditionalData {
        let utm_marks: HashMap<String, String> = self.utm_marks.into_iter().flatten().map(|mark| (mark.key, mark.value)).collect();
        let utm_marks = Some(utm_marks).filter(|m| !m.is_empty());
        NewUserAdditionalData {
            referal: self.referal,
            utm_marks,
            country: self.country,
            referer: self.referer,
        }
    }
}

/// Payload for creating identity
#[derive(Serialize, Clone)]
pub struct NewIdentity {
    pub email: String,
    pub password: String,
    pub provider: Provider,
    pub saga_id: SagaId,
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
    pub device: Option<Device>,
    pub project: Option<Project>,
}

#[derive(GraphQLInputObject, Debug, Clone)]
#[graphql(description = "Deactivate user input object")]
pub struct DeactivateUserInput {
    #[graphql(description = "Client mutation id.")]
    pub client_mutation_id: String,
    #[graphql(description = "id of a user.")]
    pub id: GraphqlID,
}

/// Payload for searching for user
#[derive(GraphQLInputObject, Debug, Serialize, Deserialize)]
#[graphql(description = "Search user input object")]
pub struct SearchUserInput {
    #[graphql(description = "Email of a user.")]
    pub email: Option<String>,
    #[graphql(description = "Phone of a user.")]
    pub phone: Option<String>,
    #[graphql(description = "first name of a user.")]
    pub first_name: Option<String>,
    #[graphql(description = "last name of a user.")]
    pub last_name: Option<String>,
    #[graphql(description = "Blocked status of a user.")]
    pub is_blocked: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UserSearchResults {
    pub total_count: u32,
    pub users: Vec<User>,
}
