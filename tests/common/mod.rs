use std::ops::Drop;

use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;
use reqwest::Client;

pub mod create_user;
pub mod get_jwt_by_provider;
pub mod microservice;

use self::microservice::UsersMicroservice;

pub struct TestContext {
    client: Client,
    users_microservice: UsersMicroservice,
}

impl TestContext {
    pub fn new() -> TestContext {
        let config = gateway_lib::config::Config::new().expect("Can't load gateway configs. Please check your /config folder.");
        let client = Client::new();

        let context = TestContext {
            client: client.clone(),
            users_microservice: UsersMicroservice {
                client: client.clone(),
                config: config.clone(),
            },
        };

        context.clear_all_data().unwrap();
        context
    }

    pub fn clear_all_data(&self) -> Result<(), FailureError> {
        self.users_microservice.clear_all_data()?;
        Ok(())
    }

    pub fn create_user(&self, input: create_user::CreateUserInput) -> Result<Option<create_user::ResponseData>, FailureError> {
        let request_body = create_user::CreateUserMutation::build_query(create_user::Variables { input });

        let mut res = self.client.post("http://gateway:8000/graphql").json(&request_body).send()?;
        let response_body: Response<create_user::ResponseData> = res.json()?;
        Ok(response_body.data)
    }

    pub fn create_user_jwt(
        &self,
        input: get_jwt_by_provider::CreateJWTProviderInput,
    ) -> Result<Option<get_jwt_by_provider::ResponseData>, FailureError> {
        let request_body = get_jwt_by_provider::GetJwtByProviderMutation::build_query(get_jwt_by_provider::Variables { input });

        let mut res = self.client.post("http://gateway:8000/graphql").json(&request_body).send()?;
        let response_body: Response<get_jwt_by_provider::ResponseData> = res.json()?;
        Ok(response_body.data)
    }
}

pub fn default_create_user_input() -> create_user::CreateUserInput {
    create_user::CreateUserInput {
        additional_data: None,
        client_mutation_id: "".to_string(),
        device: None,
        email: "user@mail.com".to_string(),
        first_name: "User".to_string(),
        last_name: "Userovsky".to_string(),
        password: "Qwerty123".to_string(),
        project: None,
    }
}

pub fn facebook_create_jwt_provider_input() -> get_jwt_by_provider::CreateJWTProviderInput {
    get_jwt_by_provider::CreateJWTProviderInput {
        client_mutation_id: "".to_string(),
        provider: get_jwt_by_provider::Provider::FACEBOOK,
        token: "facebook-token".to_string(),
        additional_data: None,
    }
}

pub fn google_create_jwt_provider_input() -> get_jwt_by_provider::CreateJWTProviderInput {
    get_jwt_by_provider::CreateJWTProviderInput {
        client_mutation_id: "".to_string(),
        provider: get_jwt_by_provider::Provider::GOOGLE,
        token: "google-token".to_string(),
        additional_data: None,
    }
}

impl Drop for TestContext {
    fn drop(&mut self) {
        match self.clear_all_data() {
            Ok(()) => {}
            Err(err) => {
                if !std::thread::panicking() {
                    panic!("Failed to clear data after test: {:?}", err);
                }
            }
        }
    }
}
