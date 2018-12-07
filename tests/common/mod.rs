use std::ops::Drop;

use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub mod create_store;
pub mod create_user;
pub mod get_jwt_by_email;
pub mod get_jwt_by_provider;
pub mod microservice;

use self::microservice::UsersMicroservice;

pub const GRAPHQL_URL: &'static str = "http://gateway:8000/graphql";

pub struct TestContext {
    bearer: Option<String>,
    client: Client,
    users_microservice: UsersMicroservice,
}

impl TestContext {
    pub fn new() -> TestContext {
        let config = gateway_lib::config::Config::new().expect("Can't load gateway configs. Please check your /config folder.");
        let client = Client::new();

        let context = TestContext {
            bearer: None,
            client: client.clone(),
            users_microservice: UsersMicroservice {
                database_url: "".to_string(),
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

    pub fn set_bearer(&mut self, bearer: String) {
        self.bearer = Some(bearer);
    }

    pub fn create_user(&self, input: create_user::CreateUserInput) -> Result<Option<create_user::ResponseData>, FailureError> {
        let request_body = create_user::CreateUserMutation::build_query(create_user::Variables { input });
        let response_body: Response<create_user::ResponseData> = self.graphql_request(request_body)?;
        Ok(response_body.data)
    }

    pub fn create_user_jwt(
        &self,
        input: get_jwt_by_provider::CreateJWTProviderInput,
    ) -> Result<Option<get_jwt_by_provider::ResponseData>, FailureError> {
        let request_body = get_jwt_by_provider::GetJwtByProviderMutation::build_query(get_jwt_by_provider::Variables { input });
        let response_body: Response<get_jwt_by_provider::ResponseData> = self.graphql_request(request_body)?;
        Ok(response_body.data)
    }

    pub fn get_jwt_by_email(
        &self,
        input: get_jwt_by_email::CreateJWTEmailInput,
    ) -> Result<Option<get_jwt_by_email::ResponseData>, FailureError> {
        let request_body = get_jwt_by_email::GetJwtByEmailMutation::build_query(get_jwt_by_email::Variables { input });
        let response_body: Response<get_jwt_by_email::ResponseData> = self.graphql_request(request_body)?;
        Ok(response_body.data)
    }

    pub fn create_store(&self, input: create_store::CreateStoreInput) -> Result<Option<create_store::ResponseData>, FailureError> {
        let request_body = create_store::CreateStoreMutation::build_query(create_store::Variables { input });
        let response_body: Response<create_store::ResponseData> = self.graphql_request(request_body)?;
        Ok(response_body.data)
    }

    fn graphql_request<T: Serialize, S: DeserializeOwned>(&self, data: T) -> Result<S, FailureError> {
        let mut request = self.client.post(GRAPHQL_URL).header("CURRENCY", "STQ");
        if let Some(ref bearer) = self.bearer {
            request = request.bearer_auth(bearer);
        }
        let mut res = request.json(&data).send()?;
        let result = res.json()?;
        Ok(result)
    }
}

pub fn default_create_store_input() -> create_store::CreateStoreInput {
    create_store::CreateStoreInput {
        client_mutation_id: "".to_string(),
        user_id: 1,
        slug: "default_store".to_string(),
        cover: None,
        logo: None,
        phone: None,
        email: None,
        slogan: None,
        long_description: None,
        instagram_url: None,
        twitter_url: None,
        facebook_url: None,
        default_language: create_store::Language::EN,
        short_description: vec![create_store::TranslationInput {
            lang: create_store::Language::EN,
            text: "short_description".to_string(),
        }],
        name: vec![create_store::TranslationInput {
            lang: create_store::Language::EN,
            text: "name".to_string(),
        }],
        address_full: create_store::AddressInput {
            value: None,
            country: None,
            country_code: None,
            administrative_area_level1: None,
            administrative_area_level2: None,
            locality: None,
            political: None,
            postal_code: None,
            route: None,
            street_number: None,
            place_id: None,
        },
    }
}

pub fn default_create_jwt_email_input() -> get_jwt_by_email::CreateJWTEmailInput {
    get_jwt_by_email::CreateJWTEmailInput {
        client_mutation_id: "".to_string(),
        email: "user@mail.com".to_string(),
        password: "Qwerty123".to_string(),
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
