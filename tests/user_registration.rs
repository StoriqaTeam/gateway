extern crate failure;
extern crate gateway_lib;
#[macro_use]
extern crate graphql_client;
extern crate reqwest;
extern crate serde_derive;
#[macro_use]
extern crate serde;
extern crate serde_json;

mod common;

use common::create_user;
use common::get_jwt_by_provider;

#[test]
pub fn creates_user() {
    //setup
    let context = common::TestContext::new();
    //given
    let new_user = common::default_create_user_input();
    //when
    let user = context.create_user(new_user).unwrap().unwrap().create_user;
    //then
    assert_eq!(user.email, common::default_create_user_input().email);
}

#[test]
pub fn create_user_with_additional_data() {
    //setup
    let context = common::TestContext::new();
    //given
    let referal = context
        .create_user(create_user::CreateUserInput {
            email: "referral@email.net".to_string(),
            ..common::default_create_user_input()
        }).unwrap()
        .unwrap();

    let new_user = create_user::CreateUserInput {
        additional_data: Some(create_user::NewUserAdditionalDataInput {
            country: Some("MMR".to_string()),
            referal: Some(referal.create_user.raw_id),
            referer: Some("localhost".to_string()),
            utm_marks_keys: Some(vec!["source".to_string()]),
            utm_marks_values: Some(vec!["word_of_mouth".to_string()]),
        }),
        ..common::default_create_user_input()
    };
    //when
    let user = context.create_user(new_user).unwrap().unwrap().create_user;
    //then
    assert_eq!(user.email, common::default_create_user_input().email);
}

#[test]
#[ignore]
pub fn create_user_via_facebook() {
    //setup
    let context = common::TestContext::new();
    //given
    let facebook_jwt = common::facebook_create_jwt_provider_input();
    //when
    let user = context.create_user_jwt(facebook_jwt);
    //then
    assert!(user.is_ok());
    assert!(user.unwrap().is_some());
}

#[test]
#[ignore]
pub fn create_user_via_google() {
    //setup
    let context = common::TestContext::new();
    //given
    let google_jwt = common::google_create_jwt_provider_input();
    //when
    let user = context.create_user_jwt(google_jwt);
    //then
    assert!(user.is_ok());
    assert!(user.unwrap().is_some());
}

#[test]
#[ignore]
pub fn create_user_via_facebook_with_additional_data() {
    //setup
    let context = common::TestContext::new();
    //given
    let referal = context
        .create_user(create_user::CreateUserInput {
            email: "referral@email.net".to_string(),
            ..common::default_create_user_input()
        }).unwrap()
        .unwrap();

    let facebook_jwt = get_jwt_by_provider::CreateJWTProviderInput {
        additional_data: Some(get_jwt_by_provider::NewUserAdditionalDataInput {
            country: Some("MMR".to_string()),
            referal: Some(referal.create_user.raw_id),
            referer: Some("localhost".to_string()),
            utm_marks_keys: Some(vec!["source".to_string()]),
            utm_marks_values: Some(vec!["word_of_mouth".to_string()]),
        }),
        ..common::facebook_create_jwt_provider_input()
    };
    //when
    let user = context.create_user_jwt(facebook_jwt);
    //then
    assert!(user.is_ok());
    assert!(user.unwrap().is_some());
}

#[test]
#[ignore]
pub fn create_user_via_google_with_additional_data() {
    //setup
    let context = common::TestContext::new();
    //given
    let referal = context
        .create_user(create_user::CreateUserInput {
            email: "referral@email.net".to_string(),
            ..common::default_create_user_input()
        }).unwrap()
        .unwrap();

    let google_jwt = get_jwt_by_provider::CreateJWTProviderInput {
        additional_data: Some(get_jwt_by_provider::NewUserAdditionalDataInput {
            country: Some("MMR".to_string()),
            referal: Some(referal.create_user.raw_id),
            referer: Some("localhost".to_string()),
            utm_marks_keys: Some(vec!["source".to_string()]),
            utm_marks_values: Some(vec!["word_of_mouth".to_string()]),
        }),
        ..common::google_create_jwt_provider_input()
    };
    //when
    let user = context.create_user_jwt(google_jwt);
    //then
    assert!(user.is_ok());
    assert!(user.unwrap().is_some());
}
