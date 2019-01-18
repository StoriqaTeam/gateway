//! File containing user object of graphql schema
use std::cmp;
use std::str::FromStr;

use chrono::prelude::*;
use futures::Future;
use hyper::Method;
use juniper;
use juniper::ID as GraphqlID;
use juniper::{FieldError, FieldResult};

use stq_api::orders::{OrderClient, OrderSearchTerms};
use stq_api::types::ApiFutureExt;
use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_static_resources::{Gender, Provider};
use stq_types::{OrderIdentifier, OrderSlug};
use stq_types::{UserId, WarehouseIdentifier, WarehouseSlug};

use super::*;
use errors::into_graphql;
use graphql::context::Context;
use graphql::models::*;
use graphql::schema::warehouse as warehouse_module;
use schema::order as order_module;

const MIN_ID: i32 = 0;

graphql_object!(User: Context as "User" |&self| {
    description: "User's profile."

    interfaces: [&Node]

    field id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Users, Model::User, self.id.0).to_string().into()
    }

    field raw_id() -> &i32 as "Unique int id"{
        &self.id.0
    }

    field email() -> &str as "Email" {
        &self.email
    }

    field phone() -> &Option<String> as "Phone" {
        &self.phone
    }

    field first_name() -> &Option<String> as "First name" {
        &self.first_name
    }

    field last_name() -> &Option<String> as "Last name" {
        &self.last_name
    }

    field middle_name() -> &Option<String> as "Middle name" {
        &self.middle_name
    }

    field gender() -> &Option<Gender> as "Gender" {
        &self.gender
    }

    field birthdate() -> &Option<String> as "Birth date" {
        &self.birthdate
    }

    field avatar() -> &Option<String> as "Avatar" {
        &self.avatar
    }

    field is_active() -> &bool as "If the user was disabled (deleted), isActive is false" {
        &self.is_active
    }

    field is_blocked() -> &bool as "Block status of a user" {
        &self.is_blocked
    }

    field emarsys_id() -> &Option<i32> as "User Emarsys id" {
        &self.emarsys_id
    }

    field created_at() -> String as "Created at" {
        let datetime: DateTime<Utc> = self.created_at.into();
        datetime.to_rfc3339()
    }

    field updated_at() -> String as "Updated at" {
        let datetime: DateTime<Utc> = self.updated_at.into();
        datetime.to_rfc3339()
    }

    field admin() -> Admin as "Admin routes" {
        Admin{}
    }

    field provider(&executor) -> Option<Provider> as "Provider user has logged in with" {
        let context = executor.context();
        context.user.clone().map(|payload| payload.provider)
    }

    field referal() -> Option<i32> as "Raw user id who advertised the project." {
        self.referal.map(|id| id.0)
    }

    field utm_marks() -> Option<Vec<UtmMark>> as "Single utm key-value pair." {
        if let Some(utm_marks) = self.utm_marks.clone() {
            let res = utm_marks.into_iter()
            .map(|(key, value)| UtmMark {
                key, value
            })
            .collect();
            Some(res)
        } else {
            None
        }
    }

    field country(&executor) -> FieldResult<Option<Country>> as "User country." {
        let context = executor.context();

        if let Some(ref alpha3) = self.country {
            let find_by_alpha3_url = format!(
                "{}/{}/alpha3/{}",
                context.config.service_url(Service::Delivery),
                Model::Country.to_url(),
                alpha3
            );
            context.request::<Option<Country>>(Method::Get, find_by_alpha3_url, None).wait()
        } else {
            Ok(None)
        }
    }

    field referer() -> &Option<String> as "Referer application domain." {
        &self.referer
    }

    field roles_on_user_microservices(&executor) -> Option<Vec<UserMicroserviceRole>> as "Fetches user roles on users microservice." {
        let context = executor.context();

        let url = format!("{}/roles/by-user-id/{}",
            context.config.service_url(Service::Users),
            self.id);

        context.request::<Vec<UserMicroserviceRole>>(Method::Get, url, None)
            .wait().ok()
    }

    field roles_on_stores_microservices(&executor) -> Option<Vec<StoresMicroserviceRole>> as "Fetches user roles on stores microservice." {
        let context = executor.context();

        let url = format!("{}/roles/by-user-id/{}",
            context.config.service_url(Service::Stores),
            self.id);

        context.request::<Vec<StoresMicroserviceRole>>(Method::Get, url, None)
            .wait().ok()
    }

    field my_store(&executor) -> FieldResult<Option<Store>> as "Fetches store of the current user." {
        let context = executor.context();

        let url = format!(
            "{}/{}/by_user_id/{}",
            &context.config.service_url(Service::Stores),
            Model::Store.to_url(),
            self.id.to_string()
        );

        context.request::<Option<Store>>(Method::Get, url, None)
            .wait()
    }

    field stores(&executor,
        first = None : Option<i32> as "First edges",
        after = None : Option<GraphqlID>  as "Id of a store",
        visibility: Option<Visibility> as "Specifies allowed visibility of the stores",
    )
            -> FieldResult<Option<Connection<Store, PageInfo>>> as "Fetches stores using relay connection." {
        let context = executor.context();
        let visibility = visibility.unwrap_or(Visibility::Active);

        let raw_id = match after {
            Some(val) => ID::from_str(&*val)?.raw_id,
            None => MIN_ID
        };

        let records_limit = context.config.gateway.records_limit;
        let first = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let url = format!("{}/{}?offset={}&count={}&visibility={}",
            context.config.service_url(Service::Stores),
            Model::Store.to_url(),
            raw_id,
            first + 1,
            visibility
        );

        context.request::<Vec<Store>>(Method::Get, url, None)
            .map (|stores| {
                let mut store_edges: Vec<Edge<Store>> = stores
                    .into_iter()
                    .map(|store| Edge::new(
                                juniper::ID::from(ID::new(Service::Stores, Model::Store, store.id.0).to_string()),
                                store.clone()
                            ))
                    .collect();
                let has_next_page = store_edges.len() as i32 == first + 1;
                if has_next_page {
                    store_edges.pop();
                };
                let has_previous_page = true;
                let start_cursor =  store_edges.get(0).map(|e| e.cursor.clone());
                let end_cursor = store_edges.iter().last().map(|e| e.cursor.clone());
                let page_info = PageInfo {
                    has_next_page,
                    has_previous_page,
                    start_cursor,
                    end_cursor};
                Connection::new(store_edges, page_info)
            })
            .wait()
            .map(Some)
    }

    field deprecated "use query store" store(&executor, id: i32 as "Int id of a store.") -> FieldResult<Option<Store>> as "Fetches store by id." {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Stores),
            Model::Store.to_url(),
            id.to_string()
        );

        context.request::<Option<Store>>(Method::Get, url, None)
            .wait()
    }

    field product(&executor, id: i32 as "Int id of a product.") -> FieldResult<Option<Product>> as "Fetches product by id." {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Stores),
            Model::Product.to_url(),
            id.to_string()
        );

        context.request::<Option<Product>>(Method::Get, url, None)
            .wait()
    }

    field products(&executor,
        first = None : Option<i32> as "First edges",
        after = None : Option<GraphqlID>  as "Base64 Id of a product")
            -> FieldResult<Option<Connection<Product, PageInfo>>> as "Fetches products using relay connection." {
        let context = executor.context();

        let raw_id = match after {
            Some(val) => ID::from_str(&*val)?.raw_id,
            None => MIN_ID
        };

        let records_limit = context.config.gateway.records_limit;
        let first = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let url = format!("{}/{}?offset={}&count={}",
            context.config.service_url(Service::Stores),
            Model::Product.to_url(),
            raw_id,
            first + 1);

        context.request::<Vec<Product>>(Method::Get, url, None)
            .map (|products| {
                let mut product_edges: Vec<Edge<Product>> = products
                    .into_iter()
                    .map(|product| Edge::new(
                                juniper::ID::from(ID::new(Service::Stores, Model::Product, product.id.0).to_string()),
                                product.clone()
                            ))
                    .collect();
                let has_next_page = product_edges.len() as i32 == first + 1;
                if has_next_page {
                    product_edges.pop();
                };
                let has_previous_page = true;
                let start_cursor =  product_edges.get(0).map(|e| e.cursor.clone());
                let end_cursor = product_edges.iter().last().map(|e| e.cursor.clone());
                let page_info = PageInfo {
                    has_next_page,
                    has_previous_page,
                    start_cursor,
                    end_cursor};
                Connection::new(product_edges, page_info)
            })
            .wait()
            .map(Some)
    }

    field base_product(&executor,
        id: i32 as "Int Id of a base product.",
        visibility: Option<Visibility> as "Specifies allowed visibility of the base product",
    ) -> FieldResult<Option<BaseProduct>> as "Fetches base product by id." {
        let context = executor.context();
        let visibility = visibility.unwrap_or(Visibility::Active);

        let url = format!(
            "{}/{}/{}?visibility={}",
            &context.config.service_url(Service::Stores),
            Model::BaseProduct.to_url(),
            id.to_string(),
            visibility
        );

        context.request::<Option<BaseProduct>>(Method::Get, url, None)
            .wait()
    }

    field base_products(&executor,
        first = None : Option<i32> as "First edges",
        after = None : Option<GraphqlID>  as "Base64 Id of base product",
        visibility: Option<Visibility> as "Specifies allowed visibility of the base products",
    ) -> FieldResult<Option<Connection<BaseProduct, PageInfo>>> as "Fetches base products using relay connection." {
        let context = executor.context();
        let visibility = visibility.unwrap_or(Visibility::Active);

        let raw_id = match after {
            Some(val) => ID::from_str(&*val)?.raw_id,
            None => MIN_ID
        };

        let records_limit = context.config.gateway.records_limit;
        let first = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let url = format!("{}/{}?offset={}&count={}&visibility={}",
            context.config.service_url(Service::Stores),
            Model::BaseProduct.to_url(),
            raw_id,
            first + 1,
            visibility
        );

        context.request::<Vec<BaseProduct>>(Method::Get, url, None)
            .map (|base_products| {
                let mut base_product_edges: Vec<Edge<BaseProduct>> = base_products
                    .into_iter()
                    .map(|base_product| Edge::new(
                                juniper::ID::from(ID::new(Service::Stores, Model::BaseProduct, base_product.id.0).to_string()),
                                base_product.clone()
                            ))
                    .collect();
                let has_next_page = base_product_edges.len() as i32 == first + 1;
                if has_next_page {
                    base_product_edges.pop();
                };
                let has_previous_page = true;
                let start_cursor =  base_product_edges.get(0).map(|e| e.cursor.clone());
                let end_cursor = base_product_edges.iter().last().map(|e| e.cursor.clone());
                let page_info = PageInfo {
                    has_next_page,
                    has_previous_page,
                    start_cursor,
                    end_cursor};
                Connection::new(base_product_edges, page_info)
            })
            .wait()
            .map(Some)
    }

    field wizard_store(&executor) -> FieldResult<Option<WizardStore>> as "Fetches wizard store." {
        let context = executor.context();

        let url = format!("{}/{}",
            &context.config.service_url(Service::Stores),
            Model::WizardStore.to_url(),
            );

        context.request::<Option<WizardStore>>(Method::Get, url, None)
            .wait()
    }

    field delivery_addresses_full(&executor) -> FieldResult<Option<Vec<UserDeliveryAddress>>> as "Fetches delivery addresses for user." {
        let context = executor.context();

        let url = format!("{}/{}/{}/addresses",
            context.config.service_url(Service::Delivery),
            Model::User.to_url(),
            self.id);

        context.request::<Vec<UserDeliveryAddress>>(Method::Get, url, None)
            .wait()
            .map(Some)
    }

    field deprecated "use query delivery_addresses_full" delivery_addresses(&executor) -> FieldResult<Option<Vec<UserDeliveryAddress>>> as "Fetches delivery addresses for user." {
        let context = executor.context();

        let url = format!("{}/{}/delivery_addresses/{}",
            context.config.service_url(Service::Users),
            Model::User.to_url(),
            self.id);

        context.request::<Vec<UserDeliveryAddress>>(Method::Get, url, None)
            .wait()
            .map(Some)
    }

    field orders(&executor,
        current_page : i32 as "Current page",
        items_count : i32 as "Items count",
        search_term_options : SearchOrderOptionInput as "Search options pattern")
            -> FieldResult<Option<Connection<GraphQLOrder, PageInfoOrdersSearch>>> as "Fetches orders using relay connection." {
        let context = executor.context();

        let offset = items_count * (current_page - 1);

        let records_limit = context.config.gateway.records_limit;
        let count = cmp::min(items_count, records_limit as i32);

        let created_from = match search_term_options.created_from.clone() {
            Some(value) => {
                match value.parse() {
                    Ok(v) => Some(v),
                    Err(_) => return Err(FieldError::new(
                        "Parsing created_from error",
                        graphql_value!({ "code": 300, "details": { "created_from has wrong format." }}),
                    )),
                }
            },
            None => None
        };

        let created_to = match search_term_options.created_to.clone() {
            Some(value) => {
                match value.parse() {
                    Ok(v) => Some(v),
                    Err(_) => return Err(FieldError::new(
                        "Parsing created_to error",
                        graphql_value!({ "code": 300, "details": { "created_to has wrong format." }}),
                    )),
                }
            },
            None => None
        };

        let customer = search_term_options.email.clone().and_then(|email| {
            let url = format!("{}/{}/by_email?email={}",
                context.config.service_url(Service::Users),
                Model::User.to_url(),
                email);

            context.request::<Option<User>>(Method::Get, url, None)
                .wait()
                .ok()
                .and_then (|user| user.map(|u|u.id))
        });

        let search_term = OrderSearchTerms {
                slug: search_term_options.slug.map(OrderSlug),
                customer: Some(self.id),
                store: None,
                created_from,
                created_to,
                payment_status: search_term_options.payment_status,
                state: search_term_options.order_status,
                ..OrderSearchTerms::default()
            };

        let rpc_client = context.get_rest_api_client(Service::Orders);
        rpc_client.search(search_term)
            .sync()
            .map_err(into_graphql)
            .map(|res| res.into_iter().map(GraphQLOrder).collect())
            .map (move |orders: Vec<GraphQLOrder>| {
                let total_pages = (orders.iter().count() as f32 / items_count as f32).ceil() as i32;

                let mut orders_edges: Vec<Edge<GraphQLOrder>> = orders
                    .into_iter()
                    .skip(offset as usize)
                    .take(count as usize)
                    .map(|order| Edge::new(
                                juniper::ID::from(order.0.id.to_string()),
                                order.clone()
                            ))
                    .collect();

                let page_info = PageInfoOrdersSearch {
                    total_pages,
                    current_page,
                    page_items_count: items_count,
                    search_term_options: search_term_options.into()
                };
                Connection::new(orders_edges, page_info)
            })
            .map(Some)
    }

    field order(&executor, slug: i32 as "Order slug" ) -> FieldResult<Option<GraphQLOrder>> as "Fetches order." {
        let context = executor.context();

        order_module::try_get_order(context, OrderIdentifier::Slug(OrderSlug(slug)))
    }


    field warehouse(&executor, slug: String as "Slug of a warehouse.") -> FieldResult<Option<GraphQLWarehouse>> as "Fetches warehouse by slug." {
        let context = executor.context();

        warehouse_module::try_get_warehouse(context, WarehouseIdentifier::Slug(WarehouseSlug(slug)))
    }

    field invoice(&executor, id: String as "Invoice id") -> FieldResult<Option<Invoice>> as "Invoice" {
        let context = executor.context();
        let url = format!("{}/invoices/by-id/{}",
            context.config.service_url(Service::Billing),
            id);

        context.request::<Option<Invoice>>(Method::Get, url, None)
            .wait()
    }

    field my_balance(&executor) -> FieldResult<Vec<MerchantBalance>> as "Fetches balance of current user." {
        let context = executor.context();

        let url = format!("{}/merchants/user/{}/balance",
            &context.config.service_url(Service::Billing),
            self.id,
            );

        context.request::<Vec<MerchantBalance>>(Method::Get, url, None)
            .wait()
    }

    field store_balance(&executor, id: i32 as "Store id") -> FieldResult<Vec<MerchantBalance>> as "Fetches balance by store id." {
        let context = executor.context();

        let url = format!("{}/merchants/store/{}/balance",
            &context.config.service_url(Service::Billing),
            id,
            );

        context.request::<Vec<MerchantBalance>>(Method::Get, url, None)
            .wait()
    }

    field stripe_customer(&executor) -> FieldResult<Option<Customer>> as "Customer." {
        let context = executor.context();

        let billing = context.get_billing_microservice();
        billing.get_current_customer()
    }

});

graphql_object!(UtmMark: Context as "UsersUtmMark" |&self| {
    description: "Users UtmMark"

    field key() -> &str {
        &self.key
    }

    field value() -> &str {
        &self.value
    }
});

graphql_object!(Connection<User, PageInfo>: Context as "UsersConnection" |&self| {
    description: "Users Connection"

    field edges() -> &[Edge<User>] {
        &self.edges
    }

    field page_info() -> &PageInfo {
        &self.page_info
    }
});

graphql_object!(Connection<User, PageInfoSegments>: Context as "UsersConnectionPages" |&self| {
    description: "Users Connection"

    field edges() -> &[Edge<User>] {
        &self.edges
    }

    field page_info() -> &PageInfoSegments {
        &self.page_info
    }
});

graphql_object!(Edge<User>: Context as "UsersEdge" |&self| {
    description:"Users Edge"

    field cursor() -> &juniper::ID {
        &self.cursor
    }

    field node() -> &User {
        &self.node
    }
});

graphql_object!(Connection<CartProduct, PageInfo>: Context as "CartProductConnection" |&self| {
    description:"CartProduct Connection"

    field edges() -> &[Edge<CartProduct>] {
        &self.edges
    }

    field page_info() -> &PageInfo {
        &self.page_info
    }
});

graphql_object!(Edge<CartProduct>: Context as "CartProductEdge" |&self| {
    description:"CartProduct Edge"

    field cursor() -> &juniper::ID {
        &self.cursor
    }

    field node() -> &CartProduct {
        &self.node
    }
});

pub fn get_user_by_id(context: &Context, user_id: UserId) -> Result<User, FieldError> {
    let users_url = context.config.service_url(Service::Users);
    let url = format!("{}/{}/{}", users_url, Model::User.to_url(), user_id);
    context
        .request::<Option<User>>(Method::Get, url, None)
        .wait()?
        .ok_or(FieldError::new(
            "User is not found in users microservice.",
            graphql_value!({ "code": 100, "details": { "User with such id does not exist in users microservice." }}),
        ))
}

pub fn run_verify_email(context: &Context, input: VerifyEmailApply) -> FieldResult<VerifyEmailApplyOutput> {
    let saga_addr = context.config.saga_microservice.url.clone();
    let url = format!("{}/email_verify_apply", saga_addr);
    let body = serde_json::to_string(&input)?;
    let result = context.request::<EmailVerifyApplyToken>(Method::Post, url, Some(body)).wait()?;

    Ok(VerifyEmailApplyOutput {
        success: true,
        token: result.token,
        email: result.user.email,
    })
}

pub fn change_alpha2_to_alpha3(context: &Context, additional_data: &mut NewUserAdditionalData) {
    additional_data.country = additional_data.country.clone().and_then(|alpha2| {
        let find_by_alpha2_url = format!(
            "{}/{}/alpha2/{}",
            context.config.service_url(Service::Delivery),
            Model::Country.to_url(),
            alpha2
        );
        let country: Option<Country> = match context.request::<Option<Country>>(Method::Get, find_by_alpha2_url, None).wait() {
            Ok(country) => country,
            Err(err) => {
                warn!("createUser - could not find country by alpha2 code: {:?}", err);
                None
            }
        };
        country.map(|country| country.alpha3)
    });
}

pub fn existing_reset_token(context: &Context, input: ExistingResetTokenInput) -> FieldResult<ResetToken> {
    let users_url = context.config.service_url(Service::Users);
    let url = match input.token_type {
        TokenTypeInput::EmailVerify => format!("{}/{}/{}/email_verify_token", users_url, Model::User.to_url(), input.user_id),
        TokenTypeInput::PasswordReset => format!("{}/{}/{}/password_reset_token", users_url, Model::User.to_url(), input.user_id),
    };
    context.request::<ResetToken>(Method::Get, url, None).wait()
}
