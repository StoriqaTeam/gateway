//! File containing mutations object of graphql schema
use std::collections::HashMap;
use std::iter::FromIterator;
use std::str::FromStr;

use futures::{Future, IntoFuture};
use graphql::context::Context;
use graphql::models::*;
use hyper::Method;
use juniper::{FieldError, FieldResult};
use serde_json;
use std::time::SystemTime;

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
            email: input.email.clone(),
            password: input.password.clone(),
            saga_id: "".to_string(),
        };
        let new_user = NewUser {
            email: input.email.clone(),
            phone: None,
            first_name: Some(input.first_name.clone()),
            last_name: Some(input.last_name.clone()),
            middle_name: None,
            gender: Gender::Undefined,
            birthdate: None,
            last_login_at: SystemTime::now(),
            saga_id: "".to_string(),
        };
        let saga_profile = SagaCreateProfile {
            identity: new_ident,
            user: Some(new_user),
        };

        let body: String = serde_json::to_string(&saga_profile)?.to_string();

        context.request::<User>(Method::Post, url, Some(body))
            .and_then(|user| {
                let url = format!("{}/{}/email_verify_token",
                    context.config.service_url(Service::Users),
                    Model::User.to_url());
                let reset = ResetRequest { email : user.email.clone(), client_mutation_id: input.client_mutation_id};
                let email = user.email.clone();
                serde_json::to_string(&reset)
                    .map_err(From::from)
                    .into_future()
                    .and_then(|body| context.request::<String>(Method::Post, url, Some(body)))
                    .and_then(|token| {
                        let to = email;
                        let subject = "Email verification".to_string();
                        let text = format!("{}/{}", context.config.notification_urls.verify_email_path, token);
                        let url = format!("{}/sendmail", 
                            context.config.service_url(Service::Notifications),
                        );
                        serde_json::to_string(&ResetMail { to, subject, text })
                            .map_err(From::from)
                            .into_future()
                            .and_then(|body| context.request::<String>(Method::Post, url, Some(body)))
                    }).map(|_| user)
            })
            .wait()
    }

    field updateUser(&executor, input: UpdateUserInput as "Create user input.") -> FieldResult<User>  as "Updates existing user."{
        let context = executor.context();
        let identifier = ID::from_str(&*input.id)?;
        let url = identifier.url(&context.config);

        if input.is_none() {
             return Err(FieldError::new(
                "Nothing to update",
                graphql_value!({ "code": 300, "details": { "All fields to update are none." }}),
            ));
        }

        input.validate()?;

        let body: String = serde_json::to_string(&input)?.to_string();

        context.request::<User>(Method::Put, url, Some(body))
            .wait()
    }

    field deactivateUser(&executor, input: DeactivateUserInput as "Deactivate user input.") -> FieldResult<User>  as "Deactivates existing user." {
        let context = executor.context();
        let identifier = ID::from_str(&*input.id)?;
        let url = identifier.url(&context.config);

        context.request::<User>(Method::Delete, url, None)
            .wait()
    }

    field changePassword(&executor, input: ChangePasswordInput as "Password change input.") -> FieldResult<ResetActionOutput>  as "Changes user password." {
        let context = executor.context();
        let url = format!("{}/{}/password_change",
            context.config.service_url(Service::Users),
            Model::User.to_url());
        let body: String = serde_json::to_string(&input)?.to_string();

        context.request::<bool>(Method::Post, url, Some(body))
            .wait()?;

        Ok(ResetActionOutput {
            success: true,
        })
    }

    field requestPasswordReset(&executor, input: ResetRequest as "Password reset request input.") -> FieldResult<ResetActionOutput>  as "Requests password reset." {
        let context = executor.context();

        let url = format!("{}/{}/password_reset_token",
            context.config.service_url(Service::Users),
            Model::User.to_url());
        let body = serde_json::to_string(&input)?;
        context.request::<String>(Method::Post, url, Some(body))
            .and_then(|token| {
                let to = input.email.clone();
                let subject = "Password reset".to_string();
                let text = format!("{}/{}", context.config.notification_urls.reset_password_path, token);
                let url = format!("{}/sendmail", 
                    context.config.service_url(Service::Notifications),
                );
                serde_json::to_string(&ResetMail { to, subject, text })
                    .map_err(From::from)
                    .into_future()
                    .and_then(|body| context.request::<String>(Method::Post, url, Some(body)))
            })
            .wait()?;

        Ok(ResetActionOutput {
            success: true,
        })
    }

    field applyPasswordReset(&executor, input: ResetApply as "Password reset apply input.") -> FieldResult<ResetActionOutput>  as "Applies password reset." {
        let context = executor.context();
        let url = format!("{}/{}/password_reset_token",
            context.config.service_url(Service::Users),
            Model::User.to_url());
        let body: String = serde_json::to_string(&input)?.to_string();

         context.request::<String>(Method::Put, url, Some(body))
            .and_then(|email| {
                let to = email.clone();
                let subject = "Password reset success".to_string();
                let text = "Password for linked account has been successfully reset.".to_string();
                let url = format!("{}/sendmail", 
                    context.config.service_url(Service::Notifications),
                );
                serde_json::to_string(&ResetMail { to, subject, text })
                    .map_err(From::from)
                    .into_future()
                    .and_then(|body| context.request::<String>(Method::Post, url, Some(body)))
            })
            .wait()?;

        Ok(ResetActionOutput {
            success: true,
        })
    }

    field resendEmailVerificationLink(&executor, input: VerifyEmailResend as "Password reset request input.") -> FieldResult<VerifyEmailOutput>  as "Requests password reset." {
        let context = executor.context();
        let url = format!("{}/{}/email_verify_token",
            context.config.service_url(Service::Users),
            Model::User.to_url());
        let reset = ResetRequest { email : input.email.clone(), client_mutation_id: input.client_mutation_id.clone()};
        let body = serde_json::to_string(&reset)?;
        context.request::<String>(Method::Post, url, Some(body))
            .and_then(|token| {
                let to = input.email.clone();
                let subject = "Email verification".to_string();
                let text = format!("{}/{}", context.config.notification_urls.verify_email_path, token);
                let url = format!("{}/sendmail", 
                    context.config.service_url(Service::Notifications),
                );
                serde_json::to_string(&ResetMail { to, subject, text })
                    .map_err(From::from)
                    .into_future()
                    .and_then(|body| context.request::<String>(Method::Post, url, Some(body)))
            })
            .wait()?;

        Ok(VerifyEmailOutput {
            success: true,
        })
    }

    field verifyEmail(&executor, input: VerifyEmailApply as "Password reset request input.") -> FieldResult<VerifyEmailOutput>  as "Requests password reset." {
        let context = executor.context();
        let url = format!("{}/{}/email_verify_token?token={}",
            context.config.service_url(Service::Users),
            Model::User.to_url(),
            input.token.clone());

        context.request::<String>(Method::Put, url, None)
            .and_then(|email| {
                let to = email;
                let subject = "Email verification".to_string();
                let text = "Email for linked account has been verified".to_string();
                let url = format!("{}/sendmail", 
                    context.config.service_url(Service::Notifications),
                );
                serde_json::to_string(&ResetMail { to, subject, text })
                    .map_err(From::from)
                    .into_future()
                    .and_then(|body| context.request::<String>(Method::Post, url, Some(body)))
            })
            .wait()?;

        Ok(VerifyEmailOutput {
            success: true,
        })
    }

    field addRoleToUser(&executor, input: NewUserRoleInput as "New User Role Input.") -> FieldResult<UserRoles>  as "Adds role to user." {
        let context = executor.context();
        let url = format!("{}/{}",
            context.config.service_url(Service::Users),
            Model::UserRoles.to_url());
        let body: String = serde_json::to_string(&input)?.to_string();

        context.request::<UserRoles>(Method::Post, url, Some(body))
            .wait()
    }

    field deleteRoleFromUser(&executor, input: OldUserRoleInput as "Old User Role Input.") -> FieldResult<UserRoles>  as "Deletes role from user." {
        let context = executor.context();
        let url = format!("{}/{}",
            context.config.service_url(Service::Users),
            Model::UserRoles.to_url());
        let body: String = serde_json::to_string(&input)?.to_string();

        context.request::<UserRoles>(Method::Delete, url, Some(body))
            .wait()
    }

    field createStore(&executor, input: CreateStoreInput as "Create store input.") -> FieldResult<Store> as "Creates new store." {
        let context = executor.context();
        let saga_addr = context.config.saga_microservice.url.clone();
        let url = format!("{}/{}",
            saga_addr,
            "create_store");
        let body: String = serde_json::to_string(&input)?.to_string();

        context.request::<Store>(Method::Post, url, Some(body))
            .wait()
    }

    field updateStore(&executor, input: UpdateStoreInput as "Update store input.") -> FieldResult<Store>  as "Updates existing store."{
        let context = executor.context();
        let identifier = ID::from_str(&*input.id)?;
        let url = identifier.url(&context.config);

        if input.is_none() {
             return Err(FieldError::new(
                "Nothing to update",
                graphql_value!({ "code": 300, "details": { "All fields to update are none." }}),
            ));
        }

        let body: String = serde_json::to_string(&input)?.to_string();

        context.request::<Store>(Method::Put, url, Some(body))
            .wait()
    }

    field deactivateStore(&executor, input: DeactivateStoreInput as "Deactivate store input.") -> FieldResult<Store>  as "Deactivates existing store." {
        let context = executor.context();
        let identifier = ID::from_str(&*input.id)?;
        let url = identifier.url(&context.config);

        context.request::<Store>(Method::Delete, url, None)
            .wait()
    }

    field createProduct(&executor, input: CreateProductWithAttributesInput as "Create product with attributes input.") -> FieldResult<Product> as "Creates new product." {
        let context = executor.context();
        let url = format!("{}/{}",
            context.config.service_url(Service::Stores),
            Model::Product.to_url());

        let body: String = serde_json::to_string(&input)?.to_string();

        context.request::<Product>(Method::Post, url, Some(body))
            .wait()
    }

    field updateProduct(&executor, input: UpdateProductWithAttributesInput as "Update product input.") -> FieldResult<Product>  as "Updates existing product."{

        let context = executor.context();
        let identifier = ID::from_str(&*input.id)?;
        let url = identifier.url(&context.config);

        if input.is_none() {
             return Err(FieldError::new(
                "Nothing to update",
                graphql_value!({ "code": 300, "details": { "All fields to update are none." }}),
            ));
        }

        let body: String = serde_json::to_string(&input)?.to_string();

        context.request::<Product>(Method::Put, url, Some(body))
            .wait()
    }

    field deactivateProduct(&executor, input: DeactivateProductInput as "Deactivate product input.") -> FieldResult<Product>  as "Deactivates existing product." {
        let context = executor.context();
        let identifier = ID::from_str(&*input.id)?;
        let url = identifier.url(&context.config);

        context.request::<Product>(Method::Delete, url, None)
            .wait()
    }

    field createBaseProduct(&executor, input: CreateBaseProductInput as "Create base product with attributes input.") -> FieldResult<BaseProduct> as "Creates new base product." {
        let context = executor.context();
        let url = format!("{}/{}",
            context.config.service_url(Service::Stores),
            Model::BaseProduct.to_url());

        let body: String = serde_json::to_string(&input)?.to_string();

        context.request::<BaseProduct>(Method::Post, url, Some(body))
            .wait()
    }

    field updateBaseProduct(&executor, input: UpdateBaseProductInput as "Update base product input.") -> FieldResult<BaseProduct>  as "Updates existing base product."{

        let context = executor.context();
        let identifier = ID::from_str(&*input.id)?;
        let url = identifier.url(&context.config);

        if input.is_none() {
             return Err(FieldError::new(
                "Nothing to update",
                graphql_value!({ "code": 300, "details": { "All fields to update are none." }}),
            ));
        }

        let body: String = serde_json::to_string(&input)?.to_string();

        context.request::<BaseProduct>(Method::Put, url, Some(body))
            .wait()
    }

    field deactivateBaseProduct(&executor, input: DeactivateBaseProductInput as "Deactivate base product input.") -> FieldResult<BaseProduct>  as "Deactivates existing base product." {
        let context = executor.context();
        let identifier = ID::from_str(&*input.id)?;
        let url = identifier.url(&context.config);

        context.request::<BaseProduct>(Method::Delete, url, None)
            .wait()
    }

    field getJWTByEmail(&executor, input: CreateJWTEmailInput as "Create jwt input.") -> FieldResult<JWT> as "Get JWT Token by email." {
        let context = executor.context();
        let url = format!("{}/{}/email",
            context.config.service_url(Service::Users),
            Model::JWT.to_url());

        let body: String = serde_json::to_string(&input)?.to_string();

        context.request::<JWT>(Method::Post, url, Some(body))
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

        context.request::<JWT>(Method::Post, url, Some(body))
            .wait()
    }

    field deprecated "do not use" renewJWT(&executor) -> FieldResult<JWT> as "Get JWT Token by email." {
        let context = executor.context();
        let url = format!("{}/{}/renew",
            context.config.service_url(Service::Users),
            Model::JWT.to_url());

        context.request::<JWT>(Method::Post, url, None)
            .wait()
    }

    field createAttribute(&executor, input: CreateAttributeInput as "Create attribute input.") -> FieldResult<Attribute> as "Creates new attribute." {
        let context = executor.context();
        let url = format!("{}/{}",
            context.config.service_url(Service::Stores),
            Model::Attribute.to_url());

        input.validate()?;

        let body: String = serde_json::to_string(&input)?.to_string();

        context.request::<Attribute>(Method::Post, url, Some(body))
            .wait()
    }

    field updateAttribute(&executor, input: UpdateAttributeInput as "Update attribute input.") -> FieldResult<Attribute>  as "Updates existing attribute."{

        let context = executor.context();
        let identifier = ID::from_str(&*input.id)?;
        let url = identifier.url(&context.config);

        if input.is_none() {
             return Err(FieldError::new(
                "Nothing to update",
                graphql_value!({ "code": 300, "details": { "All fields to update are none." }}),
            ));
        }

        let body: String = serde_json::to_string(&input)?.to_string();

        context.request::<Attribute>(Method::Put, url, Some(body))
            .wait()
    }

    field createCategory(&executor, input: CreateCategoryInput as "Create category input.") -> FieldResult<Category> as "Creates new category." {
        let context = executor.context();
        let url = format!("{}/{}",
            context.config.service_url(Service::Stores),
            Model::Category.to_url());
        let body: String = serde_json::to_string(&input)?.to_string();

        context.request::<Category>(Method::Post, url, Some(body))
            .wait()
    }

    field updateCategory(&executor, input: UpdateCategoryInput as "Update category input.") -> FieldResult<Category>  as "Updates existing category."{

        let context = executor.context();
        let identifier = ID::from_str(&*input.id)?;
        let url = identifier.url(&context.config);

        if input.is_none() {
             return Err(FieldError::new(
                "Nothing to update",
                graphql_value!({ "code": 300, "details": { "All fields to update are none." }}),
            ));
        }

        let body: String = serde_json::to_string(&input)?.to_string();

        context.request::<Category>(Method::Put, url, Some(body))
            .wait()
    }

    field addAttributeToCategory(&executor, input: AddAttributeToCategoryInput as "Create category input.") -> FieldResult<Mock> as "Creates new category." {
        let context = executor.context();
        let url = format!("{}/{}/attributes",
            context.config.service_url(Service::Stores),
            Model::Category.to_url());
        let body: String = serde_json::to_string(&input)?.to_string();

        context.request::<()>(Method::Post, url, Some(body))
            .wait()?;
        Ok(Mock{})
    }

    field deleteAttributeFromCategory(&executor, input: DeleteAttributeFromCategory as "Update category input.") -> FieldResult<Mock>  as "Updates existing category."{
        let context = executor.context();
        let url = format!("{}/{}/attributes",
            context.config.service_url(Service::Stores),
            Model::Category.to_url());
        let body: String = serde_json::to_string(&input)?.to_string();

        context.request::<()>(Method::Delete, url, Some(body))
            .wait()?;
        Ok(Mock{})
    }

    field incrementInCart(&executor, input: IncrementInCartInput as "Increment in cart input.") -> FieldResult<Option<CartStore>> as "Increment in cart." {
        let context = executor.context();

        let url = format!("{}/{}/store_id?product_id={}",
            context.config.service_url(Service::Stores),
            Model::Product.to_url(),
            input.product_id);
        let store_id = context.request::<i32>(Method::Get, url, None)
            .wait()?;

        let cp_input = CartProductIncrementPayload { store_id };
        let body: String = serde_json::to_string(&cp_input)?.to_string();
        let url = format!("{}/cart/products/{}/increment", context.config.service_url(Service::Orders), input.product_id);
        let products = context.request::<CartHash>(Method::Post, url, Some(body))
            .map (|hash| hash.into_iter()
                .map(|(product_id, info)| OrdersCartProduct {
                    product_id,
                    quantity: info.quantity,
                    store_id: info.store_id,
                    selected: info.selected,
                    comment: info.comment,
            }).collect::<Vec<OrdersCartProduct>>())
            .wait()?;

        let url = format!("{}/{}/cart",
            context.config.service_url(Service::Stores),
            Model::Store.to_url());

        let body = serde_json::to_string(&products)?;

        context.request::<Vec<Store>>(Method::Post, url, Some(body))
            .map(|stores|
                stores
                    .into_iter()
                    .find(|s| s.id == store_id)
                    .map(|store| {
                        let products = store.base_products
                            .clone()
                            .unwrap_or_default()
                            .into_iter()
                            .flat_map(|base_product| {
                                base_product.variants.clone()
                                .and_then(|mut v|{
                                    Some(v.iter_mut().map(|variant| {
                                        let (quantity, selected, comment) = products
                                            .iter()
                                            .find(|v|v.product_id == variant.id)
                                            .map(|v| (v.quantity, v.selected, v.comment.clone()))
                                            .unwrap_or_default();

                                        let price = if let Some(discount) = variant.discount {
                                            variant.price * ( 1.0 - discount )
                                        } else {
                                            variant.price
                                        };

                                        CartProduct {
                                            id: variant.id,
                                            name: base_product.name.clone(),
                                            photo_main: variant.photo_main.clone(),
                                            selected,
                                            price,
                                            quantity,
                                            comment,
                                        }
                                    }).collect::<Vec<CartProduct>>())
                                }).unwrap_or_default()
                            }).collect();
                        CartStore::new(store, products)
                    })
                )
        .wait()

    }

    field setQuantityInCart(&executor, input: SetQuantityInCartInput as "Set product in cart input.") -> FieldResult<Option<CartProduct>> as "Sets product data in cart." {
        let context = executor.context();
        let url = format!("{}/cart/products/{}/quantity", context.config.service_url(Service::Orders), input.product_id);

        let body = serde_json::to_string(&input)?;

        let order = context.request::<Option<OrdersCartProduct>>(Method::Put, url, Some(body))
            .wait()?;

        if let Some(order) = order {
            let url = format!("{}/{}/by_product/{}",
                context.config.service_url(Service::Stores),
                Model::BaseProduct.to_url(),
                order.product_id);

            context.request::<BaseProduct>(Method::Get, url, None)
                .map(|base_product| {
                    let name = base_product.name.clone();
                    base_product.variants.and_then(|variants| {
                        variants
                            .into_iter()
                            .nth(0)
                            .map(|variant| {
                                let quantity = order.quantity;
                                let selected = order.selected;
                                let comment = order.comment;

                                let price = if let Some(discount) = variant.discount {
                                    variant.price * ( 1.0 - discount )
                                } else {
                                    variant.price
                                };

                                CartProduct {
                                    id: variant.id,
                                    name,
                                    photo_main: variant.photo_main.clone(),
                                    selected,
                                    price,
                                    quantity,
                                    comment,
                                }
                            })
                    })
                })
                .wait()
        } else {
            Ok(None)
        }
    }

    field setSelectionInCart(&executor, input: SetSelectionInCartInput as "Select product in cart input.") -> FieldResult<Option<CartProduct>> as "Select product in cart." {
        let context = executor.context();
        let url = format!("{}/cart/products/{}/selection", context.config.service_url(Service::Orders), input.product_id);

        let body = serde_json::to_string(&input)?;

        let order = context.request::<Option<OrdersCartProduct>>(Method::Put, url, Some(body))
            .wait()?;

        if let Some(order) = order {

            let url = format!("{}/{}/by_product/{}",
                context.config.service_url(Service::Stores),
                Model::BaseProduct.to_url(),
                order.product_id);

            context.request::<BaseProduct>(Method::Get, url, None)
                .map(|base_product| {
                    let name = base_product.name.clone();
                    base_product.variants.and_then(|variants| {
                        variants
                            .into_iter()
                            .nth(0)
                            .map(|variant| {
                                let quantity = order.quantity;
                                let selected = order.selected;
                                let comment = order.comment;

                                let price = if let Some(discount) = variant.discount {
                                    variant.price * ( 1.0 - discount )
                                } else {
                                    variant.price
                                };

                                CartProduct {
                                    id: variant.id,
                                    name,
                                    photo_main: variant.photo_main.clone(),
                                    selected,
                                    price,
                                    quantity,
                                    comment,
                                }
                            })
                    })
                })
                .wait()
        } else {
            Ok(None)
        }
    }

    field setCommentInCart(&executor, input: SetCommentInCartInput as "Set comment in cart input.") -> FieldResult<Option<CartProduct>> as "product in cart." {
        let context = executor.context();
        let url = format!("{}/cart/products/{}/comment", context.config.service_url(Service::Orders), input.product_id);

        let body = serde_json::to_string(&input)?;

        let order = context.request::<Option<OrdersCartProduct>>(Method::Put, url, Some(body))
            .wait()?;

        if let Some(order) = order {

            let url = format!("{}/{}/by_product/{}",
                context.config.service_url(Service::Stores),
                Model::BaseProduct.to_url(),
                order.product_id);

            context.request::<BaseProduct>(Method::Get, url, None)
                .map(|base_product| {
                    let name = base_product.name.clone();
                    base_product.variants.and_then(|variants| {
                        variants
                            .into_iter()
                            .nth(0)
                            .map(|variant| {
                                let quantity = order.quantity;
                                let selected = order.selected;
                                let comment = order.comment;

                                let price = if let Some(discount) = variant.discount {
                                    variant.price * ( 1.0 - discount )
                                } else {
                                    variant.price
                                };

                                CartProduct {
                                    id: variant.id,
                                    name,
                                    photo_main: variant.photo_main.clone(),
                                    selected,
                                    price,
                                    quantity,
                                    comment,
                                }
                            })
                    })
                })
                .wait()
        } else {
            Ok(None)
        }
    }

    field deleteFromCart(&executor, input: DeleteFromCartInput as "Delete items from cart input.") -> FieldResult<Option<CartProductStore>> as "Deletes products from cart." {
        let context = executor.context();

        let url = format!("{}/cart/products/{}", context.config.service_url(Service::Orders), input.product_id);

        context.request::<Option<CartProductStore>>(Method::Delete, url, None)
            .wait()
    }

    field clearCart(&executor) -> FieldResult<Mock> as "Clears cart." {
        let context = executor.context();

        let url = format!("{}/cart/clear", context.config.service_url(Service::Orders));

        context.request::<CartHash>(Method::Post, url, None)
            .wait()?;
        Ok(Mock{})
    }

    field updateCurrencyExchange(&executor, input: NewCurrencyExchangeInput as "New currency exchange input.") -> FieldResult<CurrencyExchange> as "Updates currencies exchange." {
        let context = executor.context();

        let url = format!("{}/currency_exchange", context.config.service_url(Service::Stores));

        let body = serde_json::to_string(&input)?;

        context.request::<CurrencyExchange>(Method::Post, url, Some(body))
            .wait()
    }

    field createWizardStore(&executor) -> FieldResult<WizardStore> as "Creates new wizard store." {
        let context = executor.context();
        let url = format!("{}/{}",
            context.config.service_url(Service::Stores),
            Model::WizardStore.to_url());

        context.request::<WizardStore>(Method::Post, url, None)
            .wait()
    }

    field updateWizardStore(&executor, input: UpdateWizardStoreInput as "Update wizard store input.") -> FieldResult<WizardStore>  as "Updates existing wizard store."{
        let context = executor.context();
        let url = format!("{}/{}",
            context.config.service_url(Service::Stores),
            Model::WizardStore.to_url());

        if input.is_none() {
             return Err(FieldError::new(
                "Nothing to update",
                graphql_value!({ "code": 300, "details": { "All fields to update are none." }}),
            ));
        }

        let body: String = serde_json::to_string(&input)?.to_string();

        context.request::<WizardStore>(Method::Put, url, Some(body))
            .wait()
    }

    field deleteWizardStore(&executor) -> FieldResult<WizardStore>  as "Delete existing wizard store." {
        let context = executor.context();
        let url = format!("{}/{}",
            context.config.service_url(Service::Stores),
            Model::WizardStore.to_url());

        context.request::<WizardStore>(Method::Delete, url, None)
            .wait()
    }

    field createProductComment(&executor, input: CreateModeratorProductCommentsInput as "Create Moderator Product Comment Input.") -> FieldResult<ModeratorProductComments> as "Creates new product comment." {
        let context = executor.context();
        let url = format!("{}/{}",
            context.config.service_url(Service::Stores),
            Model::ModeratorProductComment.to_url());

        let body: String = serde_json::to_string(&input)?.to_string();

        context.request::<ModeratorProductComments>(Method::Post, url, Some(body))
            .wait()
    }

    field createStoreComment(&executor, input: CreateModeratorStoreCommentsInput as "Create Moderator Store Comment Input.") -> FieldResult<ModeratorStoreComments> as "Creates new store comment." {
        let context = executor.context();
        let url = format!("{}/{}",
            context.config.service_url(Service::Stores),
            Model::ModeratorStoreComment.to_url());

        let body: String = serde_json::to_string(&input)?.to_string();

        context.request::<ModeratorStoreComments>(Method::Post, url, Some(body))
            .wait()
    }

    field deprecated "use createUserDeliveryAddressFull" createUserDeliveryAddress(&executor, input: NewUserDeliveryAddressInput  as "Create delivery address input.") -> FieldResult<UserDeliveryAddress> as "Creates new user delivery address." {
        let context = executor.context();
        let url = format!("{}/{}/delivery_addresses",
            context.config.service_url(Service::Users),
            Model::User.to_url());

        let body: String = serde_json::to_string(&input)?.to_string();

        context.request::<UserDeliveryAddress>(Method::Post, url, Some(body))
            .wait()
    }

    field deprecated "use updateUserDeliveryAddressFull" updateUserDeliveryAddress(&executor, input: UpdateUserDeliveryAddressInput as "Update delivery address input.") -> FieldResult<UserDeliveryAddress>  as "Updates delivery address."{
        let context = executor.context();
        let url = format!("{}/{}/delivery_addresses/{}",
            context.config.service_url(Service::Users),
            Model::User.to_url(),
            input.id.to_string());

        if input.is_none() {
             return Err(FieldError::new(
                "Nothing to update",
                graphql_value!({ "code": 300, "details": { "All fields to update are none." }}),
            ));
        }

        let body: String = serde_json::to_string(&input)?.to_string();

        context.request::<UserDeliveryAddress>(Method::Put, url, Some(body))
            .wait()
    }

    field createUserDeliveryAddressFull(&executor, input: NewUserDeliveryAddressFullInput  as "Create delivery address full input.") -> FieldResult<UserDeliveryAddress> as "Creates new user delivery address full." {
        let context = executor.context();
        let url = format!("{}/{}/delivery_addresses",
            context.config.service_url(Service::Users),
            Model::User.to_url());

        let body: String = serde_json::to_string(&input)?.to_string();

        context.request::<UserDeliveryAddress>(Method::Post, url, Some(body))
            .wait()
    }

    field updateUserDeliveryAddressFull(&executor, input: UpdateUserDeliveryAddressFullInput as "Update delivery address full input.") -> FieldResult<UserDeliveryAddress>  as "Updates delivery address full."{
        let context = executor.context();
        let url = format!("{}/{}/delivery_addresses/{}",
            context.config.service_url(Service::Users),
            Model::User.to_url(),
            input.id.to_string());

        if input.is_none() {
             return Err(FieldError::new(
                "Nothing to update",
                graphql_value!({ "code": 300, "details": { "All fields to update are none." }}),
            ));
        }

        let body: String = serde_json::to_string(&input)?.to_string();

        context.request::<UserDeliveryAddress>(Method::Put, url, Some(body))
            .wait()
    }

    field deleteUserDeliveryAddress(&executor, id: i32 as "Raw id of delivery address") -> FieldResult<UserDeliveryAddress>  as "Deletes delivery address." {
        let context = executor.context();
        let url = format!("{}/{}/delivery_addresses/{}",
            context.config.service_url(Service::Users),
            Model::User.to_url(),
            id);

        context.request::<UserDeliveryAddress>(Method::Delete, url, None)
            .wait()
    }

    field createWarehouse(&executor, input: CreateWarehouseInput as "Create warehouse input.") -> FieldResult<Warehouse> as "Creates new warehouse." {
        let context = executor.context();
        let url = format!("{}/{}",
            context.config.service_url(Service::Warehouses),
            Model::Warehouse.to_url());

        let body: String = serde_json::to_string(&input)?.to_string();

        context.request::<Warehouse>(Method::Post, url, Some(body))
            .wait()
    }

    field updateWarehouse(&executor, input: UpdateWarehouseInput as "Update Warehouse input.") -> FieldResult<Option<Warehouse>>  as "Updates existing Warehouse."{
        let context = executor.context();
        let url = format!("{}/{}/by-id/{}",
            context.config.service_url(Service::Warehouses),
            Model::Warehouse.to_url(),
            input.id.to_string());

        if input.is_none() {
             return Err(FieldError::new(
                "Nothing to update",
                graphql_value!({ "code": 300, "details": { "All fields to update are none." }}),
            ));
        }

        let update: UpdateWarehouse = input.into();

        let body: String = serde_json::to_string(&update)?.to_string();

        context.request::<Option<Warehouse>>(Method::Put, url, Some(body))
            .wait()
    }

    field deleteWarehouse(&executor, id: String) -> FieldResult<Option<Warehouse>>  as "Delete existing Warehouse." {
        let context = executor.context();
        let url = format!("{}/{}/by-id/{}",
            context.config.service_url(Service::Warehouses),
            Model::Warehouse.to_url(),
            id);

        context.request::<Option<Warehouse>>(Method::Delete, url, None)
            .wait()
    }

    field deleteAllWarehouses(&executor) -> FieldResult<Vec<Warehouse>>  as "Delete all Warehouses." {
        let context = executor.context();
        let url = format!("{}/{}",
            context.config.service_url(Service::Warehouses),
            Model::Warehouse.to_url());

        context.request::<Vec<Warehouse>>(Method::Delete, url, None)
            .wait()
    }

    field setProductQuantityInWarehouse(&executor, input: ProductQuantityInput as "set Product Quantity In Warehouse input.") -> FieldResult<Option<Stock>> as "Set Product Quantity In Warehouse" {
        let context = executor.context();
        let url = format!("{}/{}/by-id/{}/{}/{}",
            context.config.service_url(Service::Warehouses),
            Model::Warehouse.to_url(),
            input.warehouse_id,
            Model::Product.to_url(),
            input.product_id);

        let body: String = serde_json::to_string(&input)?.to_string();

        context.request::<Option<Stock>>(Method::Put, url, Some(body))
            .wait()
    }

    field createOrders(&executor, input: CreateOrderInput as "Create order input.") -> FieldResult<Vec<Order>> as "Creates orders from cart." {
        let context = executor.context();

        let (products, user_id) = if let Some(user) = context.user.clone() {
            let url = format!("{}/cart/products",
                &context.config.service_url(Service::Orders));

            context.request::<CartHash>(Method::Get, url, None)
                .map (|hash|
                    hash.into_iter()
                        .map(|(product_id, info)| OrdersCartProduct {
                            product_id,
                            quantity: info.quantity,
                            store_id: info.store_id,
                            selected: info.selected,
                            comment: info.comment,
                    })
                    .map(|p| {
                        let url = format!("{}/{}/{}",
                            context.config.service_url(Service::Stores),
                            Model::Product.to_url(),
                            p.product_id);

                        context.request::<Option<Product>>(Method::Get, url, None).wait().and_then(|prod|{
                            if let Some(prod) = prod {
                                Ok(prod)
                            } else {
                                Err(FieldError::new(
                                    "Could not find product id received from cart in store.",
                                    graphql_value!({ "code": 100, "details": { "Product id does not exist in stores microservice." }}),
                                ))
                            }
                        })
                    })
                    .collect::<FieldResult<Vec<Product>>>()
                )
                .map(|p| (p, user.user_id)).wait()?
        }  else {
            return Err(FieldError::new(
                "Could not create order for unauthorized user.",
                graphql_value!({ "code": 100, "details": { "No user id in request header." }}),
            ));
        };

        let products_with_prices = HashMap::<i32, f64>::from_iter(products?.iter().map(|p| (p.id, p.price)));

        let create_order = CreateOrder {
            customer_id: user_id,
            address: input.address_full,
            receiver_name: input.receiver_name,
            prices: products_with_prices,
        };

        let url = format!("{}/{}/create_from_cart",
            context.config.service_url(Service::Orders),
            Model::Order.to_url());

        let body: String = serde_json::to_string(&create_order)?.to_string();

        context.request::<Vec<Order>>(Method::Post, url, Some(body))
            .wait()
    }

    field setOrderStatusDelivery(&executor, input: OrderStatusDeliveryInput as "Order Status Delivery input.") -> FieldResult<Option<Order>>  as "Set Order Status Delivery."{
        let context = executor.context();
        let url = format!("{}/{}/{}",
            context.config.service_url(Service::Orders),
            Model::Order.to_url(),
            input.id.to_string());

        let order: OrderStatusDelivery = input.into();

        let body: String = serde_json::to_string(&order)?.to_string();

        context.request::<Option<Order>>(Method::Put, url, Some(body))
            .wait()
    }

    field setOrderStatusPaid(&executor, input: OrderStatusPaidInput as "Order Status Paid input.") -> FieldResult<Option<Order>>  as "Set Order Status Paid."{
        let context = executor.context();
        let url = format!("{}/{}/{}",
            context.config.service_url(Service::Orders),
            Model::Order.to_url(),
            input.id.to_string());

        let order: OrderStatusPaid = input.into();

        let body: String = serde_json::to_string(&order)?.to_string();

        context.request::<Option<Order>>(Method::Put, url, Some(body))
            .wait()
    }

    field setOrderStatusComplete(&executor, input: OrderStatusCompleteInput as "Order Status Complete input.") -> FieldResult<Option<Order>>  as "Set Order Status Complete."{
        let context = executor.context();
        let url = format!("{}/{}/{}",
            context.config.service_url(Service::Orders),
            Model::Order.to_url(),
            input.id.to_string());

        let order: OrderStatusComplete = input.into();

        let body: String = serde_json::to_string(&order)?.to_string();

        context.request::<Option<Order>>(Method::Put, url, Some(body))
            .wait()
    }

});
