//! File containing mutations object of graphql schema
use std::str::FromStr;

use juniper::FieldResult;
use graphql::context::Context;
use graphql::models::*;
use hyper::Method;
use futures::Future;
use serde_json;

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
        let url = format!("{}/{}", 
            Service::Users.to_url(&context.config),
            Model::User.to_url());
        let body: String = serde_json::to_string(&input)?.to_string();

        context.http_client.request::<User>(Method::Post, url, Some(body), None)
            .or_else(|err| Err(err.to_graphql()))
            .wait()
            .and_then(|res| {
                let url = format!("{}/{}", 
                    Service::Users.to_url(&context.config),
                    Model::UserRoles.to_url());
                
                let user_role = NewUserRole {
                    user_id: res.id,
                    role: Role::User,
                };

                let body = serde_json::to_string(&user_role)?.to_string();

                // sending role to users microservice
                context.http_client.request::<UserRole>(Method::Post, url, Some(body.clone()), None)
                    .or_else(|err| Err(err.to_graphql()))
                    .wait()?;

                let url = format!("{}/{}", 
                    Service::Stores.to_url(&context.config),
                    Model::UserRoles.to_url());

                // sending role to stores microservice
                context.http_client.request::<UserRole>(Method::Post, url, Some(body), None)
                    .or_else(|err| Err(err.to_graphql()))
                    .wait()?;

                Ok(res)
            })
    }

    field updateUser(&executor, input: UpdateUserInput as "Create user input.") -> FieldResult<User>  as "Updates existing user."{
        let context = executor.context();
        let identifier = ID::from_str(&*input.id)?;
        let url = identifier.url(&context.config);
        
        let body: String = serde_json::to_string(&input)?.to_string();

        context.http_client.request_with_auth_header::<User>(Method::Put, url, Some(body), context.user.clone())
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

    field deactivateUser(&executor, input: DeactivateUserInput as "Deactivate user input.") -> FieldResult<User>  as "Deactivates existing user." {
        let context = executor.context();
        let identifier = ID::from_str(&*input.id)?;
        let url = identifier.url(&context.config);

        context.http_client.request_with_auth_header::<User>(Method::Delete, url, None, context.user.clone())
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

    field createStore(&executor, input: CreateStoreInput as "Create store input.") -> FieldResult<Store> as "Creates new store." {
        let context = executor.context();
        let url = format!("{}/{}", 
            Service::Stores.to_url(&context.config),
            Model::Store.to_url());
        let body: String = serde_json::to_string(&input)?.to_string();

        context.http_client.request_with_auth_header::<Store>(Method::Post, url, Some(body), context.user.clone())
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

    field updateStore(&executor, input: UpdateStoreInput as "Update store input.") -> FieldResult<Store>  as "Updates existing store."{
        let context = executor.context();
        let identifier = ID::from_str(&*input.id)?;
        let url = identifier.url(&context.config);
        
        let body: String = serde_json::to_string(&input)?.to_string();

        context.http_client.request_with_auth_header::<Store>(Method::Put, url, Some(body), context.user.clone())
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

    field deactivateStore(&executor, input: DeactivateStoreInput as "Deactivate store input.") -> FieldResult<Store>  as "Deactivates existing store." {
        let context = executor.context();
        let identifier = ID::from_str(&*input.id)?;
        let url = identifier.url(&context.config);

        context.http_client.request::<Store>(Method::Delete, url, None, None)
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

    field createProduct(&executor, input: CreateProductInput as "Create product input.") -> FieldResult<Product> as "Creates new product." {
        let context = executor.context();
        let url = format!("{}/{}", 
            Service::Stores.to_url(&context.config),
            Model::Product.to_url());

        let body: String = serde_json::to_string(&input)?.to_string();

        context.http_client.request_with_auth_header::<Product>(Method::Post, url, Some(body), context.user.clone())
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

    field updateProduct(&executor, input: UpdateProductInput as "Update product input.") -> FieldResult<Product>  as "Updates existing product."{
       
        let context = executor.context();
        let identifier = ID::from_str(&*input.id)?;
        let url = identifier.url(&context.config);
        
        let body: String = serde_json::to_string(&input)?.to_string();

        context.http_client.request_with_auth_header::<Product>(Method::Put, url, Some(body), context.user.clone())
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

    field deactivateProduct(&executor, input: DeactivateProductInput as "Deactivate product input.") -> FieldResult<Product>  as "Deactivates existing product." {
        let context = executor.context();
        let identifier = ID::from_str(&*input.id)?;
        let url = identifier.url(&context.config);

        context.http_client.request_with_auth_header::<Product>(Method::Delete, url, None, context.user.clone())
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

    field getJWTByEmail(&executor, input: CreateJWTEmailInput as "Create jwt input.") -> FieldResult<JWT> as "Get JWT Token by email." {
        let context = executor.context();
        let url = format!("{}/{}/email", 
            Service::Users.to_url(&context.config),
            Model::JWT.to_url());

        let body: String = serde_json::to_string(&input)?.to_string();

        context.http_client.request::<JWT>(Method::Post, url, Some(body), None)
            .or_else(|err| Err(err.to_graphql()))
            .wait()
    }

    field getJWTByProvider(&executor, input: CreateJWTProviderInput as "Create jwt input.") -> FieldResult<JWT> as "Get JWT Token by provider." {
        let context = executor.context();
        let url = format!("{}/{}/{}", 
            Service::Users.to_url(&context.config), 
            Model::JWT.to_url(),
            input.provider);
        let oauth = ProviderOauth { token: input.token };
        let body: String = serde_json::to_string(&oauth)?;

        context.http_client.request::<JWT>(Method::Post, url, Some(body), None)
            .or_else(|err| Err(err.to_graphql()))
            .wait()
            .and_then(|jwt| {
                match &jwt.status {
                    &UserStatus::New(user_id) => {
                        let url = format!("{}/{}", 
                            Service::Users.to_url(&context.config),
                            Model::UserRoles.to_url());
                        
                        let user_role = NewUserRole {
                            user_id: user_id,
                            role: Role::User,
                        };

                        let body = serde_json::to_string(&user_role)?.to_string();

                        // sending role to users microservice
                        context.http_client.request::<UserRole>(Method::Post, url, Some(body.clone()), None)
                            .or_else(|err| Err(err.to_graphql()))
                            .wait()?;

                        let url = format!("{}/{}", 
                            Service::Stores.to_url(&context.config),
                            Model::UserRoles.to_url());

                        // sending role to stores microservice
                        context.http_client.request::<UserRole>(Method::Post, url, Some(body), None)
                            .or_else(|err| Err(err.to_graphql()))
                            .wait()?;

                        Ok(jwt.into())
                    },
                    &UserStatus::Exists => Ok(jwt.into()),
                }
            })
    }

});
