//! File containing mutations object of graphql schema
use std::str::FromStr;
use std::time::SystemTime;

use futures::Future;
use graphql::context::Context;
use graphql::models::*;
use hyper::Method;
use juniper::{FieldError, FieldResult};
use serde_json;

use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_types::{CurrencyId, ProductId, ProductSellerPrice, SagaId, StoreId};

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
        let url = format!("{}/create_account",
            saga_addr);

        let new_ident = NewIdentity {
            provider: Provider::Email,
            email: input.email.clone(),
            password: input.password.clone(),
            saga_id: SagaId::new(),
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
            saga_id: SagaId::new(),
        };
        let saga_profile = SagaCreateProfile {
            identity: new_ident,
            user: Some(new_user),
        };

        let body: String = serde_json::to_string(&saga_profile)?.to_string();

        context.request::<User>(Method::Post, url, Some(body))
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
        let saga_addr = context.config.saga_microservice.url.clone();
        let url = format!("{}/reset_password",
            saga_addr);
        let body = serde_json::to_string(&input)?;
        context.request::<()>(Method::Post, url, Some(body))
            .wait()?;

        Ok(ResetActionOutput {
            success: true,
        })
    }

    field applyPasswordReset(&executor, input: ResetApply as "Password reset apply input.") -> FieldResult<ResetActionOutput>  as "Applies password reset." {
        let context = executor.context();
        let saga_addr = context.config.saga_microservice.url.clone();
        let url = format!("{}/reset_password_apply",
            saga_addr);
        let body = serde_json::to_string(&input)?;
        context.request::<()>(Method::Post, url, Some(body))
            .wait()?;

        Ok(ResetActionOutput {
            success: true,
        })
    }

    field resendEmailVerificationLink(&executor, input: VerifyEmailResend as "Email verify request input.") -> FieldResult<VerifyEmailOutput>  as "Requests email verification link on email send." {
        let context = executor.context();
        let saga_addr = context.config.saga_microservice.url.clone();
        let url = format!("{}/email_verify",
            saga_addr
            );
        let body = serde_json::to_string(&input)?;
        context.request::<()>(Method::Post, url, Some(body))
            .wait()?;

        Ok(VerifyEmailOutput {
            success: true,
        })
    }

    field verifyEmail(&executor, input: VerifyEmailApply as "Email verify apply input.") -> FieldResult<VerifyEmailOutput>  as "Applies email verification." {
        let context = executor.context();
        let saga_addr = context.config.saga_microservice.url.clone();
        let url = format!("{}/email_verify_apply",
            saga_addr);
        let body = serde_json::to_string(&input)?;
        context.request::<()>(Method::Post, url, Some(body))
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

    field incrementInCart(&executor, input: IncrementInCartInput as "Increment in cart input.") -> FieldResult<Option<Cart>> as "Increment in cart." {
        let context = executor.context();

        let url = format!("{}/{}/store_id?product_id={}",
            context.config.service_url(Service::Stores),
            Model::Product.to_url(),
            input.product_id);
        let store_id = context.request::<Option<StoreId>>(Method::Get, url, None)
            .wait()?
            .ok_or_else(||
                FieldError::new(
                    "Could not find store for product id.",
                    graphql_value!({ "code": 100, "details": { "Product with such id does not exist in stores microservice." }}),
            ))?;

        let cp_input = CartProductIncrementPayload { store_id };
        let body: String = serde_json::to_string(&cp_input)?.to_string();
        let url = format!("{}/{}/products/{}/increment", context.config.service_url(Service::Orders), Model::Cart.to_url(), input.product_id);
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
            .map(|stores| convert_to_cart(stores, &products))
            .map(Some)
            .wait()

    }

    field setQuantityInCart(&executor, input: SetQuantityInCartInput as "Set product quantity in cart input.") -> FieldResult<Option<Cart>> as "Sets product quantity in cart." {
        let context = executor.context();
        let url = format!("{}/{}/products/{}/quantity", context.config.service_url(Service::Orders), Model::Cart.to_url(), input.product_id);

        let body = serde_json::to_string(&input)?;

        let products = context.request::<CartHash>(Method::Put, url, Some(body))
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
            .map(|stores| convert_to_cart(stores, &products))
            .map(Some)
            .wait()
    }

    field setSelectionInCart(&executor, input: SetSelectionInCartInput as "Select product in cart input.") -> FieldResult<Option<Cart>> as "Select product in cart." {
        let context = executor.context();
        let url = format!("{}/{}/products/{}/selection", context.config.service_url(Service::Orders), Model::Cart.to_url(), input.product_id);

        let body = serde_json::to_string(&input)?;

        let products = context.request::<CartHash>(Method::Put, url, Some(body))
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
            .map(|stores| convert_to_cart(stores, &products))
            .map(Some)
            .wait()
    }

    field setCommentInCart(&executor, input: SetCommentInCartInput as "Set comment in cart input.") -> FieldResult<Option<Cart>> as "product in cart." {
        let context = executor.context();
        let url = format!("{}/{}/products/{}/comment", context.config.service_url(Service::Orders), Model::Cart.to_url(), input.product_id);

        let body = serde_json::to_string(&input)?;

        let products = context.request::<CartHash>(Method::Put, url, Some(body))
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
            .map(|stores| convert_to_cart(stores, &products))
            .map(Some)
            .wait()
    }

    field deleteFromCart(&executor, input: DeleteFromCartInput as "Delete items from cart input.") -> FieldResult<Cart> as "Deletes products from cart." {
        let context = executor.context();

        let url = format!("{}/{}/products/{}", context.config.service_url(Service::Orders), Model::Cart.to_url(), input.product_id);

        let products = context.request::<CartHash>(Method::Delete, url, None)
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
            .map(|stores| convert_to_cart(stores, &products))
            .wait()
    }

    field clearCart(&executor) -> FieldResult<Cart> as "Clears cart." {
        let context = executor.context();

        let url = format!("{}/{}/clear", context.config.service_url(Service::Orders), Model::Cart.to_url());

        context.request::<CartHash>(Method::Post, url, None)
            .map(|_| convert_to_cart(vec![], &[]))
            .wait()
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

    field createOrders(&executor, input: CreateOrderInput as "Create order input.") -> FieldResult<CreateOrders> as "Creates orders from cart." {
        let context = executor.context();

        let (products, user_id) = if let Some(user) = context.user.clone() {
            let url = format!("{}/{}/products",
                &context.config.service_url(Service::Orders),
                Model::Cart.to_url());

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
                        let url = format!("{}/{}/{}/seller_price",
                            context.config.service_url(Service::Stores),
                            Model::Product.to_url(),
                            p.product_id);

                        context.request::<Option<ProductSellerPrice>>(Method::Get, url, None).wait().and_then(|seller_price|{
                            if let Some(seller_price) = seller_price {
                                Ok((p.product_id, seller_price))
                            } else {
                                Err(FieldError::new(
                                    "Could not find product seller price from id received from cart.",
                                    graphql_value!({ "code": 100, "details": { "Product with such id does not exist in stores microservice." }}),
                                ))
                            }
                        })
                    })
                    .collect::<FieldResult<Vec<(ProductId, ProductSellerPrice)>>>()
                )
                .map(|p| (p, user.user_id)).wait()?
        }  else {
            return Err(FieldError::new(
                "Could not create order for unauthorized user.",
                graphql_value!({ "code": 100, "details": { "No user id in request header." }}),
            ));
        };

        let products_with_prices = products?.into_iter().collect();

        let create_order = CreateOrder {
            customer_id: user_id,
            address: input.address_full,
            receiver_name: input.receiver_name,
            receiver_phone: input.receiver_phone,
            prices: products_with_prices,
            currency_id: CurrencyId(input.currency_id),
        };

        let url = format!("{}/create_order",
            context.config.saga_microservice.url.clone());

        let body: String = serde_json::to_string(&create_order)?.to_string();

        let invoice = context.request::<Invoice>(Method::Post, url, Some(body))
            .wait()?;

        let url = format!("{}/{}/products",
            &context.config.service_url(Service::Orders), Model::Cart.to_url());

        let products = context.request::<CartHash>(Method::Get, url, None)
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

            let cart = context.request::<Vec<Store>>(Method::Post, url, Some(body))
                .map(|stores| convert_to_cart(stores, &products))
                .wait()?;

        Ok(CreateOrders::new (invoice, cart))

    }

    field setOrderStatusDelivery(&executor, input: OrderStatusDeliveryInput as "Order Status Delivery input.") -> FieldResult<Option<Order>>  as "Set Order Status Delivery."{
        let context = executor.context();
        let url = format!("{}/{}/by-slug/{}/status",
            context.config.service_url(Service::Orders),
            Model::Order.to_url(),
            input.order_slug.to_string());

        let mut order: OrderStatusDelivery = input.into();
        if let Some(ref track_id) = order.track_id {
            let comment = if let Some(mut comment) = order.comment {
                comment += format!(" | Track id: {}", track_id).as_ref();
                Some(comment)
            } else {
                Some(format!("Track id: {}", track_id))
            };
            order.comment = comment;
        }

        let body: String = serde_json::to_string(&order)?.to_string();

        context.request::<Option<Order>>(Method::Put, url, Some(body))
            .wait()
    }

    field setOrderStatusCanceled(&executor, input: OrderStatusCanceledInput as "Order Status Canceled input.") -> FieldResult<Option<Order>>  as "Set Order Status Canceled."{
        let context = executor.context();
        let url = format!("{}/{}/by-slug/{}/status",
            context.config.service_url(Service::Orders),
            Model::Order.to_url(),
            input.order_slug.to_string());

        let order: OrderStatusCanceled = input.into();

        let body: String = serde_json::to_string(&order)?.to_string();

        context.request::<Option<Order>>(Method::Put, url, Some(body))
            .wait()
    }

    field setOrderStatusComplete(&executor, input: OrderStatusCompleteInput as "Order Status Complete input.") -> FieldResult<Option<Order>>  as "Set Order Status Complete."{
        let context = executor.context();
        let url = format!("{}/{}/by-slug/{}/status",
            context.config.service_url(Service::Orders),
            Model::Order.to_url(),
            input.order_slug.to_string());

        let order: OrderStatusComplete = input.into();

        let body: String = serde_json::to_string(&order)?.to_string();

        context.request::<Option<Order>>(Method::Put, url, Some(body))
            .wait()
    }

    field recalcInvoiceAmount(&executor, id: String as "Invoice id") -> FieldResult<Invoice> as "Invoice" {
        let context = executor.context();
        let url = format!("{}/invoices/by-id/{}/recalc",
            context.config.service_url(Service::Billing),
            id);

        context.request::<Invoice>(Method::Post, url, None)
            .wait()
    }

    field updateTemplateOrderUpdateStateForUser(&executor, input: EmailTemplateInput as "Update EmailTemplate input.") -> FieldResult<String> as "Update email messages template for user order update state event" {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Notifications),
            Model::User.to_url(),
            "template-order-update-state");

        if input.is_none() {
             return Err(FieldError::new(
                "Nothing to update",
                graphql_value!({ "code": 300, "details": { "All fields to update are none." }}),
            ));
        }

        let body: String = input.data;

        context.request::<String>(Method::Put, url, Some(body))
            .wait()
    }

    field updateTemplateOrderUpdateStateForStore(&executor, input: EmailTemplateInput as "Update EmailTemplate input.") -> FieldResult<String> as "Update email messages template for store order update state event" {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Notifications),
            Model::Store.to_url(),
            "template-order-update-state");

        if input.is_none() {
             return Err(FieldError::new(
                "Nothing to update",
                graphql_value!({ "code": 300, "details": { "All fields to update are none." }}),
            ));
        }

        let body: String = input.data;

        context.request::<String>(Method::Put, url, Some(body))
            .wait()
    }

    field updateTemplateOrderCreateForUser(&executor, input: EmailTemplateInput as "Update EmailTemplate input.") -> FieldResult<String> as "Update email messages template for user order create event" {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Notifications),
            Model::User.to_url(),
            "template-order-create");

        if input.is_none() {
             return Err(FieldError::new(
                "Nothing to update",
                graphql_value!({ "code": 300, "details": { "All fields to update are none." }}),
            ));
        }

        let body: String = input.data;

        context.request::<String>(Method::Put, url, Some(body))
            .wait()
    }

    field updateTemplateOrderCreateForStore(&executor, input: EmailTemplateInput as "Update EmailTemplate input.") -> FieldResult<String> as "Update email messages template for store order create event" {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Notifications),
            Model::Store.to_url(),
            "template-order-create");

        if input.is_none() {
             return Err(FieldError::new(
                "Nothing to update",
                graphql_value!({ "code": 300, "details": { "All fields to update are none." }}),
            ));
        }

        let body: String = input.data;

        context.request::<String>(Method::Put, url, Some(body))
            .wait()
    }

    field updateTemplateEmailVerificationForUser(&executor, input: EmailTemplateInput as "Update EmailTemplate input.") -> FieldResult<String> as "Update email messages template for user email verification event" {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Notifications),
            Model::User.to_url(),
            "template-email-verification");

        if input.is_none() {
             return Err(FieldError::new(
                "Nothing to update",
                graphql_value!({ "code": 300, "details": { "All fields to update are none." }}),
            ));
        }

        let body: String = input.data;

        context.request::<String>(Method::Put, url, Some(body))
            .wait()
    }

    field updateTemplateApplyEmailVerificationForUser(&executor, input: EmailTemplateInput as "Update EmailTemplate input.") -> FieldResult<String> as "Update email messages template for user apply email verification event" {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Notifications),
            Model::User.to_url(),
            "template-apply-email-verification");

        if input.is_none() {
             return Err(FieldError::new(
                "Nothing to update",
                graphql_value!({ "code": 300, "details": { "All fields to update are none." }}),
            ));
        }

        let body: String = input.data;

        context.request::<String>(Method::Put, url, Some(body))
            .wait()
    }

    field updateTemplatePasswordResetForUser(&executor, input: EmailTemplateInput as "Update EmailTemplate input.") -> FieldResult<String> as "Update email messages template for user password reset event" {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Notifications),
            Model::User.to_url(),
            "template-password-reset");

       if input.is_none() {
             return Err(FieldError::new(
                "Nothing to update",
                graphql_value!({ "code": 300, "details": { "All fields to update are none." }}),
            ));
        }

        let body: String = input.data;

        context.request::<String>(Method::Put, url, Some(body))
            .wait()
    }

    field updateTemplateApplyPasswordResetForUser(&executor, input: EmailTemplateInput as "Update EmailTemplate input.") -> FieldResult<String> as "Update email messages template for user apply password reset event" {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Notifications),
            Model::User.to_url(),
            "template-apply-password-reset");

        if input.is_none() {
             return Err(FieldError::new(
                "Nothing to update",
                graphql_value!({ "code": 300, "details": { "All fields to update are none." }}),
            ));
        }

        let body: String = input.data;

        context.request::<String>(Method::Put, url, Some(body))
            .wait()
    }

});
