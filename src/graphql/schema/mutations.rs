//! File containing mutations object of graphql schema
use std::str::FromStr;

use juniper::FieldResult;
use graphql::context::Context;
use graphql::models::*;
use hyper::Method;
use futures::Future;
use serde_json;
use stq_routes::model::Model;
use stq_routes::service::Service;

pub struct Mutation;

graphql_object!(Mutation: Context |&self| {

    description: "Top level mutation.

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

    field createUser(&executor, input: CreateUserInput as "Create user input.") -> FieldResult<User> as "Creates new user." {
        let context = executor.context();
        let saga_addr = context.config.saga_microservice.url.clone();
        let url = format!("{}/{}",
            saga_addr,
            "create_account");

        let new_ident = NewIdentity {
            provider: Provider::Email,
            email: input.email,
            password: input.password,
            saga_id: "".to_string(),
        };
        let saga_profile = SagaCreateProfile {
            identity: new_ident,
        };
        let body: String = serde_json::to_string(&saga_profile)?.to_string();

        context.http_client.request::<User>(Method::Post, url, Some(body), None)
            .or_else(|err| Err(err.into_graphql()))
            .wait()
    }

    field updateUser(&executor, input: UpdateUserInput as "Create user input.") -> FieldResult<User>  as "Updates existing user."{
        let context = executor.context();
        let identifier = ID::from_str(&*input.id)?;
        let url = identifier.url(&context.config);

        let body: String = serde_json::to_string(&input)?.to_string();

        context.http_client.request_with_auth_header::<User>(Method::Put, url, Some(body), context.user.as_ref().map(|t| t.to_string()))
            .or_else(|err| Err(err.into_graphql()))
            .wait()
    }

    field deactivateUser(&executor, input: DeactivateUserInput as "Deactivate user input.") -> FieldResult<User>  as "Deactivates existing user." {
        let context = executor.context();
        let identifier = ID::from_str(&*input.id)?;
        let url = identifier.url(&context.config);

        context.http_client.request_with_auth_header::<User>(Method::Delete, url, None, context.user.as_ref().map(|t| t.to_string()))
            .or_else(|err| Err(err.into_graphql()))
            .wait()
    }

    field createStore(&executor, input: CreateStoreInput as "Create store input.") -> FieldResult<Store> as "Creates new store." {
        let context = executor.context();
        let url = format!("{}/{}",
            context.config.service_url(Service::Stores),
            Model::Store.to_url());
        let body: String = serde_json::to_string(&input)?.to_string();

        context.http_client.request_with_auth_header::<Store>(Method::Post, url, Some(body), context.user.as_ref().map(|t| t.to_string()))
            .or_else(|err| Err(err.into_graphql()))
            .wait()
    }

    field updateStore(&executor, input: UpdateStoreInput as "Update store input.") -> FieldResult<Store>  as "Updates existing store."{
        let context = executor.context();
        let identifier = ID::from_str(&*input.id)?;
        let url = identifier.url(&context.config);

        let body: String = serde_json::to_string(&input)?.to_string();

        context.http_client.request_with_auth_header::<Store>(Method::Put, url, Some(body), context.user.as_ref().map(|t| t.to_string()))
            .or_else(|err| Err(err.into_graphql()))
            .wait()
    }

    field deactivateStore(&executor, input: DeactivateStoreInput as "Deactivate store input.") -> FieldResult<Store>  as "Deactivates existing store." {
        let context = executor.context();
        let identifier = ID::from_str(&*input.id)?;
        let url = identifier.url(&context.config);

        context.http_client.request::<Store>(Method::Delete, url, None, None)
            .or_else(|err| Err(err.into_graphql()))
            .wait()
    }

    field createProduct(&executor, input: CreateProductWithAttributesInput as "Create product with attributes input.") -> FieldResult<Product> as "Creates new product." {
        let context = executor.context();
        let url = format!("{}/{}",
            context.config.service_url(Service::Stores),
            Model::Product.to_url());

        let body: String = serde_json::to_string(&input)?.to_string();

        context.http_client.request_with_auth_header::<Product>(Method::Post, url, Some(body), context.user.as_ref().map(|t| t.to_string()))
            .or_else(|err| Err(err.into_graphql()))
            .wait()
    }

    field updateProduct(&executor, input: UpdateProductWithAttributesInput as "Update product input.") -> FieldResult<Product>  as "Updates existing product."{

        let context = executor.context();
        let identifier = ID::from_str(&*input.id)?;
        let url = identifier.url(&context.config);

        let body: String = serde_json::to_string(&input)?.to_string();

        context.http_client.request_with_auth_header::<Product>(Method::Put, url, Some(body), context.user.as_ref().map(|t| t.to_string()))
            .or_else(|err| Err(err.into_graphql()))
            .wait()
    }

    field deactivateProduct(&executor, input: DeactivateProductInput as "Deactivate product input.") -> FieldResult<Product>  as "Deactivates existing product." {
        let context = executor.context();
        let identifier = ID::from_str(&*input.id)?;
        let url = identifier.url(&context.config);

        context.http_client.request_with_auth_header::<Product>(Method::Delete, url, None, context.user.as_ref().map(|t| t.to_string()))
            .or_else(|err| Err(err.into_graphql()))
            .wait()
    }

    field createBaseProduct(&executor, input: CreateBaseProductInput as "Create base product with attributes input.") -> FieldResult<BaseProduct> as "Creates new base product." {
        let context = executor.context();
        let url = format!("{}/{}",
            context.config.service_url(Service::Stores),
            Model::BaseProduct.to_url());

        let body: String = serde_json::to_string(&input)?.to_string();

        context.http_client.request_with_auth_header::<BaseProduct>(Method::Post, url, Some(body), context.user.as_ref().map(|t| t.to_string()))
            .or_else(|err| Err(err.into_graphql()))
            .wait()
    }

    field updateBaseProduct(&executor, input: UpdateBaseProductInput as "Update base product input.") -> FieldResult<BaseProduct>  as "Updates existing base product."{

        let context = executor.context();
        let identifier = ID::from_str(&*input.id)?;
        let url = identifier.url(&context.config);

        let body: String = serde_json::to_string(&input)?.to_string();

        context.http_client.request_with_auth_header::<BaseProduct>(Method::Put, url, Some(body), context.user.as_ref().map(|t| t.to_string()))
            .or_else(|err| Err(err.into_graphql()))
            .wait()
    }

    field deactivateBaseProduct(&executor, input: DeactivateBaseProductInput as "Deactivate base product input.") -> FieldResult<BaseProduct>  as "Deactivates existing base product." {
        let context = executor.context();
        let identifier = ID::from_str(&*input.id)?;
        let url = identifier.url(&context.config);

        context.http_client.request_with_auth_header::<BaseProduct>(Method::Delete, url, None, context.user.as_ref().map(|t| t.to_string()))
            .or_else(|err| Err(err.into_graphql()))
            .wait()
    }

    field getJWTByEmail(&executor, input: CreateJWTEmailInput as "Create jwt input.") -> FieldResult<JWT> as "Get JWT Token by email." {
        let context = executor.context();
        let url = format!("{}/{}/email",
            context.config.service_url(Service::Users),
            Model::JWT.to_url());

        let body: String = serde_json::to_string(&input)?.to_string();

        context.http_client.request::<JWT>(Method::Post, url, Some(body), None)
            .or_else(|err| Err(err.into_graphql()))
            .wait()
    }

    field getJWTByProvider(&executor, input: CreateJWTProviderInput as "Create jwt input.") -> FieldResult<JWT> as "Get JWT Token by provider." {
        let context = executor.context();
        let url = format!("{}/{}/{}",
            context.config.service_url(Service::Users),
            Model::JWT.to_url(),
            input.provider);
        let oauth = ProviderOauth { token: input.token };
        let body: String = serde_json::to_string(&oauth)?;

        context.http_client.request::<JWT>(Method::Post, url, Some(body), None)
            .or_else(|err| Err(err.into_graphql()))
            .wait()
    }

    field createAttribute(&executor, input: CreateAttributeInput as "Create attribute input.") -> FieldResult<Attribute> as "Creates new attribute." {
        let context = executor.context();
        let url = format!("{}/{}",
            context.config.service_url(Service::Stores),
            Model::Attribute.to_url());

        let body: String = serde_json::to_string(&input)?.to_string();

        context.http_client.request_with_auth_header::<Attribute>(Method::Post, url, Some(body), context.user.as_ref().map(|t| t.to_string()))
            .or_else(|err| Err(err.into_graphql()))
            .wait()
    }
    
    field updateAttribute(&executor, input: UpdateAttributeInput as "Update attribute input.") -> FieldResult<Attribute>  as "Updates existing attribute."{

        let context = executor.context();
        let identifier = ID::from_str(&*input.id)?;
        let url = identifier.url(&context.config);

        let body: String = serde_json::to_string(&input)?.to_string();

        context.http_client.request_with_auth_header::<Attribute>(Method::Put, url, Some(body), context.user.as_ref().map(|t| t.to_string()))
            .or_else(|err| Err(err.into_graphql()))
            .wait()
    }

    field createCategory(&executor, input: CreateCategoryInput as "Create category input.") -> FieldResult<Category> as "Creates new category." {
        let context = executor.context();
        let url = format!("{}/{}",
            context.config.service_url(Service::Stores),
            Model::Category.to_url());
        let body: String = serde_json::to_string(&input)?.to_string();

        context.http_client.request_with_auth_header::<Category>(Method::Post, url, Some(body), context.user.as_ref().map(|t| t.to_string()))
           .or_else(|err| Err(err.into_graphql()))
            .wait()
    }

    field updateCategory(&executor, input: UpdateCategoryInput as "Update category input.") -> FieldResult<Category>  as "Updates existing category."{

        let context = executor.context();
        let identifier = ID::from_str(&*input.id)?;
        let url = identifier.url(&context.config);

        let body: String = serde_json::to_string(&input)?.to_string();

        context.http_client.request_with_auth_header::<Category>(Method::Put, url, Some(body), context.user.as_ref().map(|t| t.to_string()))
            .or_else(|err| Err(err.into_graphql()))
            .wait()
    }

});
