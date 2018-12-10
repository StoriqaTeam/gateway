use std::time::{Duration, SystemTime};

use chrono::prelude::*;
use juniper::ID as GraphqlID;
use juniper::{FieldError, FieldResult};

use stq_types::{BaseProductId, CouponCode, CouponId, StoreId};

/// Payload for coupon
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Coupon {
    pub id: CouponId,
    pub code: CouponCode,
    pub title: String,
    pub store_id: StoreId,
    pub scope: CouponScope,
    pub percent: i32,
    pub quantity: i32,
    pub expired_at: Option<SystemTime>,
    pub is_active: bool,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

impl Coupon {
    pub fn scope_support(&self) -> FieldResult<bool> {
        match self.scope {
            CouponScope::Store => Err(FieldError::new(
                "Error response from microservice",
                graphql_value!({ "code": 100, "details": {
                    "status": "400 Bad Request",
                    "code": "400",
                    "message":
                        "{ Using a coupon for a store is not supported }"
                    ,
                }}),
            )),
            CouponScope::BaseProducts => Ok(true),
            CouponScope::Categories => Err(FieldError::new(
                "Error response from microservice",
                graphql_value!({ "code": 100, "details": {
                    "status": "400 Bad Request",
                    "code": "400",
                    "message":
                        "{ Using a coupon for a categories is not supported }"
                    ,
                }}),
            )),
        }
    }
}

/// Input Object for creating coupon
#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Create coupon input object")]
pub struct NewCouponInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "code")]
    pub code: String,
    #[graphql(description = "title")]
    pub title: String,
    #[graphql(description = "store id")]
    pub store_id: i32,
    #[graphql(description = "scope")]
    pub scope: CouponScope,
    #[graphql(description = "percent")]
    pub percent: i32,
    #[graphql(description = "quantity")]
    pub quantity: i32,
    #[graphql(description = "expired at")]
    pub expired_at: Option<DateTime<Utc>>,
}

/// Input Object for updating coupon
#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq, Eq)]
#[graphql(description = "Update coupon input object")]
pub struct UpdateCouponInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Id of a coupon.")]
    #[serde(skip_serializing)]
    pub id: GraphqlID,
    #[graphql(description = "percent")]
    pub percent: Option<i32>,
    #[graphql(description = "quantity")]
    pub quantity: Option<i32>,
    #[graphql(description = "quantity")]
    pub expired_at: Option<DateTime<Utc>>,
    #[graphql(description = "is active")]
    pub is_active: Option<bool>,
}

#[derive(GraphQLInputObject, Debug, Clone)]
#[graphql(description = "Change base_product in coupon")]
pub struct ChangeBaseProductsInCoupon {
    #[graphql(description = "Client mutation id.")]
    pub client_mutation_id: String,
    #[graphql(description = "Id of a coupon.")]
    pub raw_id: i32,
    #[graphql(description = "Id of a base_product.")]
    pub raw_base_product_id: i32,
}

/// Payload for creating coupon
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewCoupon {
    pub code: CouponCode,
    pub title: String,
    pub store_id: i32,
    pub scope: CouponScope,
    pub percent: i32,
    pub quantity: i32,
    pub expired_at: Option<SystemTime>,
}

/// Payload for updating coupon
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateCoupon {
    pub percent: Option<i32>,
    pub quantity: Option<i32>,
    pub expired_at: Option<SystemTime>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CouponScopeBaseProducts {
    pub id: i32,
    pub coupon_id: CouponId,
    pub base_product_id: BaseProductId,
}

#[derive(GraphQLEnum, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[graphql(description = "Coupon application scope")]
pub enum CouponScope {
    Store,
    Categories,
    BaseProducts,
}

impl UpdateCouponInput {
    pub fn is_none(&self) -> bool {
        &Self {
            client_mutation_id: self.client_mutation_id.clone(),
            id: self.id.clone(),
            percent: None,
            quantity: None,
            expired_at: None,
            is_active: None,
        } == self
    }
}

impl From<NewCouponInput> for NewCoupon {
    fn from(input: NewCouponInput) -> Self {
        Self {
            code: CouponCode(input.code),
            title: input.title,
            store_id: input.store_id,
            scope: input.scope,
            percent: input.percent,
            quantity: input.quantity,
            expired_at: input.expired_at.map(into_system_time),
        }
    }
}

impl From<UpdateCouponInput> for UpdateCoupon {
    fn from(input: UpdateCouponInput) -> Self {
        Self {
            percent: input.percent,
            quantity: input.quantity,
            expired_at: input.expired_at.map(into_system_time),
            is_active: input.is_active,
        }
    }
}

fn into_system_time(datetime: DateTime<Utc>) -> SystemTime {
    SystemTime::UNIX_EPOCH + Duration::new(datetime.timestamp() as u64, 0)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CouponsSearchCodePayload {
    pub code: CouponCode,
    pub store_id: StoreId,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum CouponValidate {
    NotActive,
    HasExpired,
    NoActivationsAvailable,
    AlreadyActivated,
    Valid,
}

impl CouponValidate {
    pub fn validate(&self) -> FieldResult<()> {
        match *self {
            CouponValidate::NotActive => Err(FieldError::new(
                "Error response from microservice",
                graphql_value!({ "code": 100, "details": {
                    "status": "400 Bad Request",
                    "code": "400",
                    "message":
                        "{ Coupon is not active }"
                    ,
                }}),
            )),

            CouponValidate::AlreadyActivated => Err(FieldError::new(
                "Error response from microservice",
                graphql_value!({ "code": 100, "details": {
                    "status": "400 Bad Request",
                    "code": "400",
                    "message":
                        "{ Coupon is already activated }"
                    ,
                }}),
            )),
            CouponValidate::HasExpired => Err(FieldError::new(
                "Error response from microservice",
                graphql_value!({ "code": 100, "details": {
                    "status": "400 Bad Request",
                    "code": "400",
                    "message":
                        "{ Coupon has expired }"
                    ,
                }}),
            )),
            CouponValidate::NoActivationsAvailable => Err(FieldError::new(
                "Error response from microservice",
                graphql_value!({ "code": 100, "details": {
                    "status": "400 Bad Request",
                    "code": "400",
                    "message":
                        "{ No activations available for the coupon }"
                    ,
                }}),
            )),
            CouponValidate::Valid => Ok(()),
        }
    }
}
