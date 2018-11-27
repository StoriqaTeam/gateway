//! File containing query object of graphql schema
use std::str::FromStr;

use futures::Future;
use hyper::Method;
use juniper::ID as GraphqlID;
use juniper::{FieldError, FieldResult};
use uuid::Uuid;

use stq_api::orders::{CartClient, OrderClient};
use stq_api::types::ApiFutureExt;
use stq_api::warehouses::WarehouseClient;
use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_static_resources::currency::Currency;
use stq_static_resources::{Language, LanguageGraphQl, OrderState, TemplateVariant};
use stq_types::{OrderId, WarehouseId};

use super::*;
use errors::into_graphql;
use graphql::context::Context;
use graphql::models::*;
use graphql::schema::cart as cart_module;
use schema::buy_now;

pub const QUERY_NODE_ID: i32 = 1;

pub struct Query;

graphql_object!(Query: Context |&self| {

    description: "Top level query.

    Remote mark

    Some fields are marked as `Remote`. That means that they are
    part of microservices and their fetching can fail.
    In this case null will be returned (even if o/w
    type signature declares not-null type) and corresponding errors
    will be returned in errors section. Each error is guaranteed
    to have a `code` field and `details field`.

    Codes:
    - 100 - microservice responded,
    but with error http status. In this case `details` is guaranteed
    to have `status` field with http status and
    probably some additional details.

    - 200 - there was a network error while connecting to microservice.

    - 300 - there was a parse error - that usually means that
    graphql couldn't parse api json response
    (probably because of mismatching types on graphql and microservice)
    or api url parse failed.

    - 400 - Unknown error."

    interfaces: [&Node]

    field id() -> GraphqlID as "Base64 Unique id"{
        QUERY_NODE_ID.to_string().into()
    }

    field apiVersion() -> &str as "Current api version." {
        "1.0"
    }

    field static_node_id() -> StaticNodeIds as "Static node id dictionary." {
        StaticNodeIds{}
    }

    field me(&executor) -> FieldResult<Option<User>> as "Fetches viewer for users." {
        let context = executor.context();
        let url = format!("{}/{}/current",
            context.config.service_url(Service::Users),
            Model::User.to_url());
        context.request::<Option<User>>(Method::Get, url, None)
                    .wait()
    }

    field node(&executor, id: GraphqlID as "Base64 Id of a node.") -> FieldResult<Option<Node>> as "Fetches graphql interface node by Base64 id."  {
        let context = executor.context();
        if *id == QUERY_NODE_ID.to_string() {
             Ok(Some(Node::Query(Query{})))
        } else {
            let identifier = ID::from_str(&*id)?;
            match (&identifier.service, &identifier.model) {
                (&Service::Users, &Model::User) => {
                    context.request::<Option<User>>(Method::Get, identifier.url(&context.config), None)
                        .wait()
                        .map(|res| res.map(Node::User))
                },
                (&Service::Users, _) => {
                    Err(FieldError::new(
                        "Could not get model from users microservice.",
                        graphql_value!({ "internal_error": "Unknown model" })
                    ))
                },
                (&Service::Stores, &Model::Store) => {
                    context.request::<Option<Store>>(Method::Get, identifier.url(&context.config), None)
                        .wait()
                        .map(|res| res.map(Box::new).map(Node::Store))
                },
                (&Service::Stores, &Model::Product) => {
                    context.request::<Option<Product>>(Method::Get, identifier.url(&context.config), None)
                        .wait()
                        .map(|res| res.map(Node::Product))
                },
                (&Service::Stores, &Model::BaseProduct) => {
                    context.request::<Option<BaseProduct>>(Method::Get, identifier.url(&context.config), None)
                        .wait()
                        .map(|res| res.map(Node::BaseProduct))
                },
                (&Service::Stores, &Model::Category) => {
                    context.request::<Option<Category>>(Method::Get, identifier.url(&context.config), None)
                        .wait()
                        .map(|res| res.map(Node::Category))
                },
                (&Service::Stores, &Model::Attribute) => {
                    context.request::<Option<Attribute>>(Method::Get, identifier.url(&context.config), None)
                        .wait()
                        .map(|res| res.map(Node::Attribute))
                },
                (&Service::Stores, &Model::CustomAttribute) => {
                    context.request::<Option<CustomAttribute>>(Method::Get, identifier.url(&context.config), None)
                        .wait()
                        .map(|res| res.map(Node::CustomAttribute))
                },
                (&Service::Stores, _) => {
                    Err(FieldError::new(
                        "Could not get model from stores microservice.",
                        graphql_value!({ "internal_error": "Unknown model" })
                    ))
                },
                (Service::Orders, &Model::Order) => {
                    Uuid::parse_str(&id.to_string())
                        .map_err(|_|
                            FieldError::new(
                                "Given id can not be parsed as Uuid",
                                graphql_value!({ "parse_error": "Order id must be uuid" })
                            )
                        )
                        .and_then(|id|{
                            let rpc_client = context.get_rest_api_client(Service::Orders);
                            rpc_client.get_order(OrderId(id).into())
                                .sync()
                                .map_err(into_graphql)
                                .map(|res| res.map(GraphQLOrder).map(Box::new).map(Node::Order))
                        })
                },
                (Service::Orders, _) => {
                    Err(FieldError::new(
                        "Could not get model from orders microservice.",
                        graphql_value!({ "internal_error": "Unknown model" })
                    ))
                },
                (&Service::Warehouses, &Model::Warehouse) => {
                    Uuid::parse_str(&id.to_string())
                        .map_err(|_|
                            FieldError::new(
                                "Given id can not be parsed as Uuid",
                                graphql_value!({ "parse_error": "Warehouse id must be uuid" })
                            )
                        )
                        .and_then(|id|{
                            let rpc_client = context.get_rest_api_client(Service::Warehouses);
                            rpc_client.get_warehouse(WarehouseId(id).into())
                                .sync()
                                .map_err(into_graphql)
                                .map(|res| res.map(GraphQLWarehouse).map(Box::new).map(Node::Warehouse))
                        })
                },
                (&Service::Warehouses, _) => {
                    Err(FieldError::new(
                        "Could not get model from warehouses microservice.",
                        graphql_value!({ "internal_error": "Unknown model" })
                    ))
                }
                (&Service::Notifications, _) => {
                    Err(FieldError::new(
                        "Could not get model from notifications microservice.",
                        graphql_value!({ "internal_error": "Unknown model" })
                    ))
                }
                (&Service::Billing, _) => {
                    Err(FieldError::new(
                        "Could not get model from billing microservice.",
                        graphql_value!({ "internal_error": "Unknown model" })
                    ))
                }
                (&Service::Delivery, &Model::Company) => {
                    context.request::<Option<Company>>(Method::Get, identifier.url(&context.config), None)
                        .wait()
                        .map(|res| res.map(Node::Company))
                },
                (&Service::Delivery, &Model::Package) => {
                    context.request::<Option<Packages>>(Method::Get, identifier.url(&context.config), None)
                        .wait()
                        .map(|res| res.map(Node::Package))
                },
                (&Service::Delivery, &Model::CompanyPackage) => {
                    context.request::<Option<CompaniesPackages>>(Method::Get, identifier.url(&context.config), None)
                        .wait()
                        .map(|res| res.map(Node::CompanyPackage))
                },
                (&Service::Delivery, _) => {
                    Err(FieldError::new(
                        "Could not get model from delivery microservice.",
                        graphql_value!({ "internal_error": "Unknown model" })
                    ))
                }
            }
        }
    }

    field languages(&executor) -> Vec<LanguageGraphQl> as "Fetches languages." {
        Language::as_vec()
    }

    field currencies(&executor) -> Vec<Currency> as "Fetches currencies." {
        // trello: https://trello.com/c/Q5ZdFhNF (#317)
        vec![Currency::STQ]
    }

    field order_statuses(&executor) -> Vec<OrderState> as "Fetches order statuses." {
        OrderState::enum_iter().collect()
    }

    field categories(&executor) -> FieldResult<Option<Category>> as "Fetches categories tree." {
        let context = executor.context();
        let url = format!("{}/{}",
            context.config.service_url(Service::Stores),
            Model::Category.to_url());

        context.request::<Option<Category>>(Method::Get, url, None)
            .wait()
    }

    field category_by_slug(&executor, category_slug: String) -> FieldResult<Option<Category>> as "Find category by slug" {
        let context = executor.context();
        let url = format!("{}/{}/by-slug/{}",
            context.config.service_url(Service::Stores),
            Model::Category.to_url(),
            category_slug);

        context.request::<Option<Category>>(Method::Get, url, None)
            .wait()
    }

    field countries(&executor) -> FieldResult<Country> as "Fetches country tree." {
        let context = executor.context();
        let url = format!("{}/{}",
            context.config.service_url(Service::Delivery),
            Model::Country.to_url());

        context.request::<Country>(Method::Get, url, None)
            .wait()
    }

    field country(&executor, alpha3: String as "Alpha3 code") -> FieldResult<Option<Country>> as "Find country by alpha3 code." {
        let context = executor.context();
        let url = format!("{}/{}/alpha3/{}",
            context.config.service_url(Service::Delivery),
            Model::Country.to_url(),
            alpha3);

        context.request::<Option<Country>>(Method::Get, url, None)
            .wait()
    }

    field calculate_buy_now(&executor, product_id: i32 as "Product raw id",
                            quantity: i32 as "Quantity",
                            coupon_code: Option<String> as "Coupon code",
                            _company_package_id: Option<i32> as "[DEPRECATED] Select available package raw id",
                            shipping_id: Option<i32> as "Select available package shipping raw id") -> FieldResult<BuyNowCheckout> as "Calculate values for buy now." {

        let context = executor.context();

        buy_now::calculate_buy_now(context, product_id, quantity, coupon_code, shipping_id)
    }

    field currency_exchange(&executor) -> FieldResult<Option<Vec<CurrencyExchange>>> as "Fetches currency exchange." {
        let context = executor.context();
        let url = format!("{}/currency_exchange",
            context.config.service_url(Service::Stores));

        // trello: https://trello.com/c/Q5ZdFhNF (#317)
        let _ = context.request::<Option<CurrencyExchangeInfo>>(Method::Get, url, None)
            .wait().map(|v| {
                v.map(|v| CurrencyExchange::from_data(v.data))
            });

        Ok(Some(vec![]))

    }

    field attributes(&executor) -> FieldResult<Option<Vec<Attribute>>> as "Fetches all attributes." {
        let context = executor.context();
        let url = format!("{}/{}",
            context.config.service_url(Service::Stores),
            Model::Attribute.to_url());

        context.request::<Vec<Attribute>>(Method::Get, url, None)
            .wait()
            .map(Some)
    }

    field custom_attributes(&executor) -> FieldResult<Option<Vec<CustomAttribute>>> as "Fetches all custom attributes." {
        let context = executor.context();
        let url = format!("{}/{}",
            context.config.service_url(Service::Stores),
            Model::CustomAttribute.to_url());

        context.request::<Vec<CustomAttribute>>(Method::Get, url, None)
            .wait()
            .map(Some)
    }

    field search(&executor) -> Search as "Search endpoint" {
        Search{}
    }

    field main_page(&executor) -> MainPage as "Main page endpoint" {
        MainPage{}
    }

    field email_template(&executor, variant: TemplateVariant) -> FieldResult<String> as "Template email message endpoint" {
        let context = executor.context();

        let url = format!(
            "{}/templates/{}",
            &context.config.service_url(Service::Notifications),
            variant);

        context.request::<String>(Method::Get, url, None)
            .wait()
    }

    field store(&executor,
        id: i32 as "Int id of a store.",
        visibility: Option<Visibility> as "Specifies allowed visibility of the store",
    ) -> FieldResult<Option<Store>> as "Fetches store by id." {

        let context = executor.context();
        let visibility = visibility.unwrap_or_default();

        let url = format!(
            "{}/{}/{}?visibility={}",
            &context.config.service_url(Service::Stores),
            Model::Store.to_url(),
            id.to_string(),
            visibility,
        );

        context.request::<Option<Store>>(Method::Get, url, None)
            .wait()
    }

    field store_by_slug(
        &executor,
        slug: String as "String slug of a store.",
        visibility: Option<Visibility> as "Specifies allowed visibility of the store",
    ) -> FieldResult<Option<Store>> as "Fetches store by slug." {
        let context = executor.context();
        let visibility = visibility.unwrap_or_default();

        let url = format!(
            "{}/{}/by-slug/{}?visibility={}",
            &context.config.service_url(Service::Stores),
            Model::Store.to_url(),
            slug,
            visibility,
        );

        context.request::<Option<Store>>(Method::Get, url, None)
            .wait()
    }

    field base_product(&executor,
        id: i32 as "Int Id of a base product.",
        visibility: Option<Visibility> as "Specifies allowed visibility of the base product",
    ) -> FieldResult<Option<BaseProduct>> as "Fetches base product by id." {
        let context = executor.context();
        let visibility = visibility.unwrap_or_default();

        let url = match visibility {
            Visibility::Published => format!(
                "{}/{}/{}/update_view",
                &context.config.service_url(Service::Stores),
                Model::BaseProduct.to_url(),
                id.to_string()
            ),
            Visibility::Active => format!(
                "{}/{}/{}?visibility={}",
                &context.config.service_url(Service::Stores),
                Model::BaseProduct.to_url(),
                id.to_string(),
                visibility,
            ),
        };

        context.request::<Option<BaseProduct>>(Method::Get, url, None)
            .wait()
    }

    field base_product_by_slug(
        &executor,
        store_slug: String as "String slug of a store.",
        base_product_slug: String as "String slug of a base product.",
        visibility: Option<Visibility> as "Specifies allowed visibility of the base product",
    ) -> FieldResult<Option<BaseProduct>> as "Fetches base product by slug." {
        let context = executor.context();
        let visibility = visibility.unwrap_or_default();

        let url = match visibility {
            Visibility::Published => format!(
                "{}/{}/by-slug/{}/{}/by-slug/{}/update_view",
                &context.config.service_url(Service::Stores),
                Model::Store.to_url(),
                store_slug,
                Model::BaseProduct.to_url(),
                base_product_slug
            ),
            Visibility::Active => format!(
                "{}/{}/by-slug/{}/{}/by-slug/{}?visibility={}",
                &context.config.service_url(Service::Stores),
                Model::Store.to_url(),
                store_slug,
                Model::BaseProduct.to_url(),
                base_product_slug,
                visibility,
            ),
        };

        context.request::<Option<BaseProduct>>(Method::Get, url, None)
            .wait()
    }

    field companies(&executor) -> FieldResult<Option<Vec<Company>>> as "Fetches all companies." {
        let context = executor.context();

        let url = format!(
            "{}/{}",
            &context.config.service_url(Service::Delivery),
            Model::Company.to_url()
        );

        context.request::<Option<Vec<Company>>>(Method::Get, url, None)
            .wait()
    }

    field company(&executor, id: i32 as "Int Id of a company.") -> FieldResult<Option<Company>> as "Fetches company by id." {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Delivery),
            Model::Company.to_url(),
            id.to_string()
        );

        context.request::<Option<Company>>(Method::Get, url, None)
            .wait()
    }

    field package(&executor, id: i32 as "Int Id of a package.") -> FieldResult<Option<Packages>> as "Fetches package by id." {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Delivery),
            Model::Package.to_url(),
            id.to_string()
        );

        context.request::<Option<Packages>>(Method::Get, url, None)
            .wait()
    }

    field company_package(&executor, id: i32 as "Int Id of a company_package.") -> FieldResult<Option<CompaniesPackages>> as "Fetches company_package by id." {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Delivery),
            Model::CompanyPackage.to_url(),
            id.to_string()
        );

        context.request::<Option<CompaniesPackages>>(Method::Get, url, None)
            .wait()
    }

    field cart(&executor) -> FieldResult<Option<Cart>> as "Fetches cart products." {
        let context = executor.context();

        let rpc_client = context.get_rest_api_client(Service::Orders);
        let fut = if let Some(session_id) = context.session_id {
            if let Some(ref user) = context.user {
                rpc_client.merge(session_id.into(), user.user_id.into())
            } else {
                rpc_client.get_cart(session_id.into())
            }
        } else if let Some(ref user) = context.user {
            rpc_client.get_cart(user.user_id.into())
        }  else {
            return Err(FieldError::new(
                "Could not get users cart.",
                graphql_value!({ "code": 100, "details": { "No user id or session id in request header." }}),
            ));
        };

        let products: Vec<_> = fut
            .sync()
            .map_err(into_graphql)?.into_iter().collect();

        cart_module::convert_products_to_cart(context, &products).map(Some)

    }

    field store_slug_exists(&executor, slug: String as "Stores slug") -> FieldResult<bool> as "Checks store slug" {
        let context = executor.context();
        let url = format!("{}/{}/slug_exists?slug={}",
            context.config.service_url(Service::Stores),
            Model::Store.to_url(),
            slug);
        context.request::<bool>(Method::Get, url, None)
            .wait()
    }

    field available_packages(
        &executor,
        country_code: String as "Alpha3 code country",
        size: i32 as "Volume of the product (cm^3)",
        weight: i32 as "Weight of the product (g)"
    ) -> FieldResult<AvailablePackagesOutput> as "Available Packages" {
        let context = executor.context();

        if !country_code.is_empty() {
            let url = format!("{}/available_packages?country={}&weight={}&size={}",
                context.config.service_url(Service::Delivery),
                country_code,
                size,
                weight
                );

            context.request::<Vec<AvailablePackages>>(Method::Get, url, None)
                .map(From::from)
                .wait()
        } else {
            Err(FieldError::new(
                "Country code is empty",
                graphql_value!({ "code": 300, "details": { "Country code needs to have length > 0." }}),
            ))
        }
    }

    field generate_coupon_code(&executor) -> FieldResult<String> as "New coupon code" {
        let context = executor.context();

        let url = format!("{}/{}/generate_code",
            context.config.service_url(Service::Stores),
            Model::Coupon.to_url());

        context.request::<String>(Method::Get, url, None)
            .wait()
    }

    field available_shipping_for_user(&executor, user_country: String as "Alpha3 code country", base_product_id: i32 as "Int Id of a base_product.") -> FieldResult<AvailableShippingForUser> as "Available shipping for user" {
        let context = executor.context();
        let url = format!("{}/available_packages_for_user/{}?user_country={}",
            context.config.service_url(Service::Delivery),
            base_product_id,
            user_country
        );

        context.request::<AvailableShippingForUser>(Method::Get, url, None)
            .wait()
    }

});
