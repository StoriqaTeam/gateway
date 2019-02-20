//! File containing mutations object of graphql schema
use std::collections::HashSet;
use std::str::FromStr;
use std::time::SystemTime;

use futures::Future;
use graphql::context::Context;
use graphql::models::*;
use graphql::schema::coupon::*;
use hyper::Method;
use juniper::{FieldError, FieldResult};
use serde_json;
use uuid::Uuid;

use stq_api::orders::{CartClient, Order};
use stq_api::types::ApiFutureExt;
use stq_api::warehouses::WarehouseClient;
use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_static_resources::{CurrencyType, Provider};
use stq_types::{BaseProductId, CartItem, CouponCode, CouponId, ProductId, SagaId, StoreId, UserId, WarehouseId};

use errors::into_graphql;
use graphql::schema::base_product as base_product_module;
use graphql::schema::buy_now;
use graphql::schema::cart as cart_module;
use graphql::schema::category as category_module;
use graphql::schema::order;
use graphql::schema::payout;
use graphql::schema::product as product_module;
use graphql::schema::store as store_module;
use graphql::schema::stripe as stripe_module;
use graphql::schema::user as user_module;

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

        let mut additional_data: NewUserAdditionalData = input.additional_data.unwrap_or_default().into();

        user_module::change_alpha2_to_alpha3(&context, &mut additional_data);

        let new_user = NewUser {
            email: input.email.clone(),
            phone: None,
            first_name: Some(input.first_name.clone()),
            last_name: Some(input.last_name.clone()),
            middle_name: None,
            gender: None,
            birthdate: None,
            last_login_at: SystemTime::now(),
            saga_id: SagaId::new(),
            referal: additional_data.referal.map(UserId),
            utm_marks: additional_data.utm_marks,
            country: additional_data.country,
            referer: additional_data.referer,
        };
        let saga_profile = SagaCreateProfile {
            identity: new_ident,
            user: Some(new_user),
            device: input.device,
            project: input.project,
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

    field blockUser(&executor, id: i32 as "Users raw id.") -> FieldResult<User>  as "Block existing user." {
        let context = executor.context();
        let url = format!("{}/{}/{}/block",
            context.config.service_url(Service::Users),
            Model::User.to_url(),
            id);

        context.request::<User>(Method::Post, url, None)
            .wait()
    }

    field unblockUser(&executor, id: i32 as "User raw id.") -> FieldResult<User>  as "Unblock existing user." {
        let context = executor.context();
        let url = format!("{}/{}/{}/unblock",
            context.config.service_url(Service::Users),
            Model::User.to_url(),
            id);

        context.request::<User>(Method::Post, url, None)
            .wait()
    }

    field deleteUser(&executor, id: i32 as "User raw id.") -> FieldResult<Mock> as "Delete user from DB" {
        let context = executor.context();
        let url = format!("{}/{}/{}/delete",
            context.config.service_url(Service::Users),
            Model::User.to_url(),
            id);

        context.request::<()>(Method::Delete, url, None)
            .wait().map(|_| Mock)
    }

    field changePassword(&executor, input: ChangePasswordInput as "Password change input.") -> FieldResult<ResetApplyActionOutput>  as "Changes user password." {
        let context = executor.context();
        let url = format!("{}/{}/password_change",
            context.config.service_url(Service::Users),
            Model::User.to_url());
        let body: String = serde_json::to_string(&input)?.to_string();

        let token = context.request::<String>(Method::Post, url, Some(body))
            .wait()?;

        Ok(ResetApplyActionOutput {
            success: true,
            token
        })
    }

    field requestPasswordReset(&executor, input: ResetRequest as "Password reset request input.") -> FieldResult<ResetActionOutput>  as "Requests password reset." {
        let context = executor.context();
        let saga_addr = context.config.saga_microservice.url.clone();
        let url = format!("{}/reset_password",
            saga_addr);

        let input = input.fill_uuid();
        let body = serde_json::to_string(&input)?;
        context.request::<()>(Method::Post, url, Some(body))
            .wait()?;

        Ok(ResetActionOutput {
            success: true,
        })
    }

    field applyPasswordReset(&executor, input: ResetApply as "Password reset apply input.") -> FieldResult<ResetApplyActionOutput>  as "Applies password reset." {
        let context = executor.context();
        let saga_addr = context.config.saga_microservice.url.clone();
        let url = format!("{}/reset_password_apply",
            saga_addr);
        let body = serde_json::to_string(&input)?;
        let token = context.request::<String>(Method::Post, url, Some(body))
            .wait()?;

        Ok(ResetApplyActionOutput {
            success: true,
            token,
        })
    }

    field resendEmailVerificationLink(&executor, input: VerifyEmailResend as "Email verify request input.") -> FieldResult<VerifyEmailResendOutput>  as "Requests email verification link on email send." {
        let context = executor.context();
        let saga_addr = context.config.saga_microservice.url.clone();
        let url = format!("{}/email_verify",
            saga_addr
            );
        let body = serde_json::to_string(&input)?;
        context.request::<()>(Method::Post, url, Some(body))
            .wait()?;

        Ok(VerifyEmailResendOutput {
            success: true,
        })
    }

    field verifyEmail(&executor, input: VerifyEmailApply as "Email verify apply input.") -> FieldResult<VerifyEmailApplyOutput>  as "Applies email verification." {
        let context = executor.context();

        user_module::run_verify_email(context, input)
    }

    field addRoleToUserOnUsersMicroservice(&executor, input: NewUsersRoleInput as "New Users  Role Input.") -> FieldResult<NewRole<UserMicroserviceRole>>  as "Adds users  role to user." {
        let context = executor.context();
        let url = format!("{}/roles",
            context.config.service_url(Service::Users));
        let body: String = serde_json::to_string(&input)?.to_string();

        context.request::<NewRole<UserMicroserviceRole>>(Method::Post, url, Some(body))
            .wait()
    }

    field addRoleToUserOnStoresMicroservice(&executor, input: NewStoresRoleInput as "New Stores  Role Input.") -> FieldResult<NewRole<StoresMicroserviceRole>>  as "Adds stores role to user." {
        let context = executor.context();

        let stores = context.get_stores_microservice();
        stores.add_role_to_user(input)
    }

    field addRoleToUserOnBillingMicroservice(&executor, input: NewBillingRoleInput as "New Billing Role Input.") -> FieldResult<NewRole<BillingMicroserviceRole>>  as "Adds billing role to user." {
        let context = executor.context();

        let billing = context.get_billing_microservice();
        billing.add_role_to_user(input)
    }

    field removeRoleFromUserOnUsersMicroservice(&executor, input: RemoveUsersRoleInput as "New Users  Role Input.") -> FieldResult<NewRole<UserMicroserviceRole>>  as "Removes users role." {
        let context = executor.context();
        let url = format!("{}/roles",
            context.config.service_url(Service::Users));
        let body: String = serde_json::to_string(&input)?.to_string();

        context.request::<NewRole<UserMicroserviceRole>>(Method::Delete, url, Some(body))
            .wait()
    }

    field removeRoleFromUserOnStoresMicroservice(&executor, input: RemoveStoresRoleInput as "New Stores  Role Input.") -> FieldResult<NewRole<StoresMicroserviceRole>>  as "Removes stores role." {
        let context = executor.context();

        let stores = context.get_stores_microservice();
        stores.remove_role_from_user(input)
    }

    field removeRoleFromUserOnBillingMicroservice(&executor, input: RemoveBillingRoleInput as "Remove Billing Role Input.") -> FieldResult<NewRole<BillingMicroserviceRole>>  as "Removes billing role." {
        let context = executor.context();

        let billing = context.get_billing_microservice();
        billing.remove_role_from_user(input)
    }

    field createStore(&executor, input: CreateStoreInput as "Create store input.") -> FieldResult<Store> as "Creates new store." {
        let context = executor.context();
        let saga_addr = context.config.saga_microservice.url.clone();
        let url = format!("{}/{}",
            saga_addr,
            "create_store");
        let body: String = serde_json::to_string(&input.fill_uuid())?.to_string();

        context.request::<Store>(Method::Post, url, Some(body))
            .wait()
    }

    field updateStore(&executor, input: UpdateStoreInput as "Update store input.") -> FieldResult<Store>  as "Updates existing store."{
        let context = executor.context();

        store_module::run_update_store_mutation(context, input)
    }

    field deleteStore(&executor, id: i32 as "Delete store raw id.") -> FieldResult<Mock> as "Deletes existing store from DB." {
        let context = executor.context();
        let url = format!("{}/{}/{}/delete",
            context.config.service_url(Service::Stores),
            Model::Store.to_url(),
            id);

        context.request::<()>(Method::Delete, url, None)
            .wait().map(|_| Mock)
    }

    field deactivateStore(&executor, input: DeactivateStoreInput as "Deactivate store input.") -> FieldResult<Store>  as "Deactivates existing store." {
        let context = executor.context();
        let identifier = ID::from_str(&*input.id)?;
        let url = format!("{}/{}/{}/deactivate", context.config.saga_microservice.url, Model::Store.to_url(), identifier.raw_id);
        context.request::<Store>(Method::Post, url, None)
            .wait()
    }

    field deprecated "use setModerationStatusStore" publishStore(&executor, id: i32 as "Store raw id.") -> FieldResult<Store>  as "Publish store." {
        let context = executor.context();
        let url = format!("{}/{}/{}/publish",
            context.config.service_url(Service::Stores),
            Model::Store.to_url(),
            id);

        context.request::<Store>(Method::Post, url, None)
            .wait()
    }

    field draftStore(&executor, id: i32 as "Store raw id.") -> FieldResult<Store>  as "Hide the store from users." {
        let context = executor.context();

        store_module::run_send_to_draft_store_mutation(context, StoreId(id))
    }

    field sendStoreToModeration(&executor, id: i32 as "Store raw id.") -> FieldResult<Store>  as "Send store on moderation for store manager." {
        let context = executor.context();

        store_module::run_send_to_moderation_store(context, StoreId(id))
    }

    field setModerationStatusStore(&executor, input: StoreModerateInput as "Change store moderation status input.") -> FieldResult<Store>  as "Change store moderation status for moderator." {
        let context = executor.context();

        store_module::run_moderation_status_store(context, input)
    }

    field createProduct(&executor, input: CreateProductWithAttributesInput as "Create product with attributes input.") -> FieldResult<Product> as "Creates new product." {
        let context = executor.context();
        let url = format!("{}/{}",
            context.config.service_url(Service::Stores),
            Model::Product.to_url());
        let mut input = input;
        input.product = input.product.fill_uuid(input.client_mutation_id.clone());
        let body: String = serde_json::to_string(&input)?.to_string();

        context.request::<Product>(Method::Post, url, Some(body))
            .wait()
    }

    field updateProduct(&executor, input: UpdateProductWithAttributesInput as "Update product input.") -> FieldResult<Product>  as "Updates existing product."{

        let context = executor.context();

        product_module::run_update_product_mutation(context, input)
    }

    field deactivateProduct(&executor, input: DeactivateProductInput as "Deactivate product input.") -> FieldResult<Product>  as "Deactivates existing product." {
        let context = executor.context();
        let identifier = ID::from_str(&*input.id)?;
        let url = format!("{}/{}/{}/deactivate", context.config.saga_microservice.url, Model::Product.to_url(), identifier.raw_id);
        context.request::<Product>(Method::Post, url, None)
            .wait()
    }

    field createBaseProduct(&executor, input: CreateBaseProductInput as "Create base product with attributes input.") -> FieldResult<BaseProduct> as "Creates new base product." {
        let context = executor.context();
        let url = format!("{}/{}",
            context.config.service_url(Service::Stores),
            Model::BaseProduct.to_url());
        let body: String = serde_json::to_string(&input.fill_uuid())?.to_string();

        context.request::<BaseProduct>(Method::Post, url, Some(body))
            .wait()
    }

    field createBaseProductWithVariants(&executor, input: NewBaseProductWithVariantsInput as "Create base product with variants input.") -> FieldResult<BaseProduct> as "Creates new base product with variants." {
        let context = executor.context();
        let url = format!("{}/{}/create_with_variants",
            context.config.saga_microservice.url,
            Model::BaseProduct.to_url());
        let mut input = input;
        input.variants = input.variants.into_iter()
            .map(|mut variant| {
                variant.product = variant.product.fill_uuid(variant.client_mutation_id.clone());
                variant
            })
            .collect();
        let body: String = serde_json::to_string(&input.fill_uuid())?.to_string();

        context.request::<BaseProduct>(Method::Post, url, Some(body))
            .wait()
    }

    field updateBaseProduct(&executor, input: UpdateBaseProductInput as "Update base product input.") -> FieldResult<BaseProduct>  as "Updates existing base product."{

        let context = executor.context();

        base_product_module::run_update_base_product(context, input)
    }

    field deactivateBaseProduct(&executor, input: DeactivateBaseProductInput as "Deactivate base product input.") -> FieldResult<BaseProduct>  as "Deactivates existing base product." {
        let context = executor.context();
        let identifier = ID::from_str(&*input.id)?;
        let url = format!("{}/{}/{}/deactivate", context.config.saga_microservice.url, Model::BaseProduct.to_url(), identifier.raw_id);
        context.request::<BaseProduct>(Method::Post, url, None)
            .wait()
    }

    field deprecated "use setModerationStatusBaseProduct" publishBaseProducts(&executor, ids: Vec<i32> as "BaseProduct raw ids.") -> FieldResult<Vec<BaseProduct>>  as "Published base_products." {
        let context = executor.context();
        let url = format!("{}/{}/publish",
            context.config.service_url(Service::Stores),
            Model::BaseProduct.to_url());

        let body: String = serde_json::to_string(&ids)?.to_string();

        context.request::<Vec<BaseProduct>>(Method::Post, url, Some(body))
            .wait()
    }

    field draftBaseProducts(&executor, ids: Vec<i32> as "BaseProduct raw ids.") -> FieldResult<Vec<BaseProduct>>  as "Hide base_products from users." {
        let context = executor.context();

        base_product_module::run_draft_base_products_mutation(context, ids)
    }

    field sendBaseProductToModeration(&executor, id: i32 as "BaseProduct raw id.") -> FieldResult<BaseProduct>  as "Send base product on moderation for store manager." {
        let context = executor.context();

        base_product_module::run_send_to_moderation_base_product(context, BaseProductId(id))
    }

    field setModerationStatusBaseProduct(&executor, input: BaseProductModerateInput as "Change base product moderation status input.") -> FieldResult<BaseProduct>  as "Change base product moderation status for moderator." {
        let context = executor.context();

        base_product_module::run_moderation_status_base_product(context, input)
    }

    field createCustomAttribute(&executor, input: NewCustomAttributeInput as "Create custom attribute input.") -> FieldResult<CustomAttribute> as "Creates new custom attribute" {
        let context = executor.context();
        let url = format!("{}/{}",
            context.config.service_url(Service::Stores),
            Model::CustomAttribute.to_url());

        let body: String = serde_json::to_string(&input)?.to_string();

        context.request::<CustomAttribute>(Method::Post, url, Some(body))
            .wait()
    }

    field deleteCustomAttribute(&executor, input: DeleteCustomAttributeInput as "Delete custom attribute input.") -> FieldResult<CustomAttribute> as "Deletes custom attribute" {
        let context = executor.context();
        let url = format!("{}/{}/{}",
            context.config.service_url(Service::Stores),
            Model::CustomAttribute.to_url(),
            input.custom_attribute_id);

        context.request::<CustomAttribute>(Method::Delete, url, None)
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

        let mut additional_data = input.additional_data.unwrap_or_default().into();
        user_module::change_alpha2_to_alpha3(&context, &mut additional_data);

        let oauth = ProviderOauth { token: input.token, additional_data: Some(additional_data)};
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

        let body: String = serde_json::to_string(&input.fill_uuid())?.to_string();

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

    field deleteAttribute(&executor, input: DeleteAttributeInput as "Delete attribute input.") -> FieldResult<Mock>  as "Deletes existing attribute."{
        let context = executor.context();
        let identifier = ID::from_str(&*input.id)?;
        let url = identifier.url(&context.config);

        context.request::<()>(Method::Delete, url, None).wait()?;
        Ok(Mock)
    }

    field createAttributeValue(&executor, input: CreateAttributeValueInput) -> FieldResult<AttributeValue> as "Creates new attribute value" {
        let context = executor.context();
        let url = format!(
            "{}/{}/{}/{}",
            context.config.service_url(Service::Stores),
            Model::Attribute.to_url(),
            input.raw_attribute_id,
            Model::AttributeValue.to_url(),
        );

        let body: String = serde_json::to_string(&input)?.to_string();

        context.request::<AttributeValue>(Method::Post, url, Some(body))
            .wait()
    }

    field updateAttributeValue(&executor, input: UpdateAttributeValueInput) -> FieldResult<AttributeValue> as "Updates existing attribute value" {
        let context = executor.context();
        if input.is_none() {
             return Err(FieldError::new(
                "Nothing to update",
                graphql_value!({ "code": 300, "details": { "All fields to update are none." }}),
            ));
        }

        let url = format!(
            "{}/{}/{}/{}",
            context.config.service_url(Service::Stores),
            Model::Attribute.to_url(),
            Model::AttributeValue.to_url(),
            input.raw_id,
        );

        let body: String = serde_json::to_string(&input)?.to_string();

        context.request::<AttributeValue>(Method::Put, url, Some(body)).wait()
    }

    field deleteAttributeValue(&executor, input: DeleteAttributeValueInput) -> FieldResult<Mock> as "Deletes existing attribute value" {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}/{}",
            context.config.service_url(Service::Stores),
            Model::Attribute.to_url(),
            Model::AttributeValue.to_url(),
            input.raw_id,
        );

        context.request::<AttributeValue>(Method::Delete, url, None).wait()?;

        Ok(Mock)
    }

    field createCategory(&executor, input: CreateCategoryInput as "Create category input.") -> FieldResult<Category> as "Creates new category." {
        let context = executor.context();
        let url = format!("{}/{}",
            context.config.service_url(Service::Stores),
            Model::Category.to_url());
        let body: String = serde_json::to_string(&input.fill_uuid())?.to_string();

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

    field deleteCategory(&executor, input: DeleteCategoryInput as "Category to delete") -> FieldResult<Mock> as "Delete specific category" {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}",
            context.config.service_url(Service::Stores),
            Model::Category.to_url(),
            input.cat_id,
        );

        context.request::<()>(Method::Delete, url, None)
            .wait()?;
        Ok(Mock{})
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

    field incrementInCart(
        &executor,
        input: IncrementInCartInput as "Increment in cart input.",
    ) -> FieldResult<Option<Cart>> as "Increment in cart." {
        let context = executor.context();

        cart_module::run_increment_in_cart_v1(context, input)
    }

    field incrementInCartV2(
        &executor,
        input: IncrementInCartInputV2 as "Increment in cart input.",
    ) -> FieldResult<Option<Cart>> as "Increment in cart." {
        let context = executor.context();

        cart_module::run_increment_in_cart(context, input)
    }

    field AddInCart(
        &executor,
        input: AddInCartInput as "Add product quantity, plus delivery method in cart input.",
    ) -> FieldResult<Option<Cart>> as "Add in cart." {
        let context = executor.context();

        cart_module::run_add_in_cart_v1(context, input)

    }

    field addInCartV2(
        &executor,
        input: AddInCartInputV2 as "Add product quantity, plus delivery method in cart input.",
    ) -> FieldResult<Option<Cart>> as "Add in cart." {
        let context = executor.context();

        cart_module::run_add_in_cart(context, input)
    }

    field setQuantityInCart(&executor, input: SetQuantityInCartInput as "Set product quantity in cart input.") -> FieldResult<Option<Cart>> as "Sets product quantity in cart." {
        let context = executor.context();

        let customer = if let Some(ref user) = context.user {
            user.user_id.into()
        } else if let Some(session_id) = context.session_id {
            session_id.into()
        }  else {
            return Err(FieldError::new(
                "Could not set item quantity in cart for unauthorized user.",
                graphql_value!({ "code": 100, "details": { "No user id in request header." }}),
            ));
        };

        let rpc_client = context.get_rest_api_client(Service::Orders);
        let products:Vec<_> = rpc_client.set_quantity(customer, input.product_id.into(), input.value.into())
            .sync()
            .map_err(into_graphql)?
            .into_iter().collect();

        cart_module::convert_products_to_cart(context, &products, None).map(Some)

    }

    field setQuantityInCartV2(
        &executor,
        input: SetQuantityInCartInputV2 as "Set product quantity in cart input."
    ) -> FieldResult<Option<Cart>> as "Sets product quantity in cart." {
        let context = executor.context();

        let customer = if let Some(ref user) = context.user {
            user.user_id.into()
        } else if let Some(session_id) = context.session_id {
            session_id.into()
        }  else {
            return Err(FieldError::new(
                "Could not set item quantity in cart for unauthorized user.",
                graphql_value!({ "code": 100, "details": { "No user id in request header." }}),
            ));
        };

        let rpc_client = context.get_rest_api_client(Service::Orders);
        let products:Vec<_> = rpc_client.set_quantity(customer, input.product_id.into(), input.value.into())
            .sync()
            .map_err(into_graphql)?
            .into_iter().collect();

        cart_module::convert_products_to_cart(context, &products, Some(input.user_country_code)).map(Some)

    }

    field setCouponInCart(
        &executor,
        input: SetCouponInCartInput as "Set coupon in cart input.",
        currency_type: Option<CurrencyType> as "Currency type",
    ) -> FieldResult<Option<Cart>> as "Sets coupon in cart." {
        let context = executor.context();

        let customer = if let Some(ref user) = context.user {
            user.user_id.into()
        } else if let Some(session_id) = context.session_id {
            session_id.into()
        }  else {
            return Err(FieldError::new(
                "Could not set coupon in cart for unauthorized user.",
                graphql_value!({ "code": 100, "details": { "No user id in request header." }}),
            ));
        };

        let coupon_code = CouponCode(input.coupon_code.clone());
        let store_id = StoreId(input.store_id);
        validate_coupon_by_code(context, coupon_code.clone(), store_id)?;
        let coupon = get_coupon_by_code(context, coupon_code, store_id)?;

        // validate scope coupon
        let scope_support = coupon.scope_support()?;
        if !scope_support {
            return Ok(None);
        }

        let rpc_client = context.get_rest_api_client(Service::Orders);
        let current_cart = rpc_client.get_cart(customer, currency_type).sync()?;

        // validate used coupon
        let coupon_apply = current_cart.iter().any(|c| {
                if let Some(coupon_id) = c.coupon_id {
                    coupon_id == coupon.id
                } else {
                    false
                }
        });

        if coupon_apply {
            return Err(FieldError::new(
                "Coupon not set",
                graphql_value!({ "code": 400, "details": { "coupon already applied" }}),
            ));
        }

        // validate products
        let url = format!("{}/{}/{}/base_products",
            context.config.service_url(Service::Stores),
            Model::Coupon.to_url(),
            coupon.id);
        let base_products = context.request::<Vec<BaseProduct>>(Method::Get, url, None).wait()?;
        let all_support_products = base_products.into_iter().flat_map(|b| {
            if let Some(variants) = b.variants {
                variants
            } else {
                vec![]
            }
            })
            .filter(|p|
                match p.discount {
                    Some(discount) => discount < ZERO_DISCOUNT,
                    None => true,
                })
            .map(|p| p.id).collect::<HashSet<ProductId>>();

        let all_cart_products:HashSet<ProductId> = current_cart.iter().map(|c| c.product_id).collect();
        let products_for_cart:HashSet<ProductId> = all_cart_products.intersection(&all_support_products).cloned().collect();

        if products_for_cart.is_empty() {
            return Err(FieldError::new(
                "Coupon not set",
                graphql_value!({ "code": 400, "details": { "no products found for coupon usage" }}),
            ));
        }

        for product_id in products_for_cart {
            let rpc_client = context.get_rest_api_client(Service::Orders);
            rpc_client.add_coupon(customer, product_id, coupon.id).sync()?;
        }

        let rpc_client = context.get_rest_api_client(Service::Orders);
        let products: Vec<_> = rpc_client.get_cart(customer, currency_type).sync()
            .map_err(into_graphql)?
            .into_iter().collect();

        cart_module::convert_products_to_cart(context, &products, None).map(Some)

    }

    field setCouponInCartV2(
        &executor,
        input: SetCouponInCartInputV2 as "Set coupon in cart input.",
        currency_type: Option<CurrencyType> as "Currency type",
    ) -> FieldResult<Option<Cart>> as "Sets coupon in cart." {
        let context = executor.context();

        let customer = if let Some(ref user) = context.user {
            user.user_id.into()
        } else if let Some(session_id) = context.session_id {
            session_id.into()
        }  else {
            return Err(FieldError::new(
                "Could not set coupon in cart for unauthorized user.",
                graphql_value!({ "code": 100, "details": { "No user id in request header." }}),
            ));
        };

        let coupon_code = CouponCode(input.coupon_code.clone());
        let store_id = StoreId(input.store_id);
        validate_coupon_by_code(context, coupon_code.clone(), store_id)?;
        let coupon = get_coupon_by_code(context, coupon_code, store_id)?;

        // validate scope coupon
        let scope_support = coupon.scope_support()?;
        if !scope_support {
            return Ok(None);
        }

        let rpc_client = context.get_rest_api_client(Service::Orders);
        let current_cart = rpc_client.get_cart(customer, currency_type).sync()?;

        // validate used coupon
        let coupon_apply = current_cart.iter().any(|c| {
                if let Some(coupon_id) = c.coupon_id {
                    coupon_id == coupon.id
                } else {
                    false
                }
        });

        if coupon_apply {
            return Err(FieldError::new(
                "Coupon not set",
                graphql_value!({ "code": 400, "details": { "coupon already applied" }}),
            ));
        }

        // validate products
        let url = format!("{}/{}/{}/base_products",
            context.config.service_url(Service::Stores),
            Model::Coupon.to_url(),
            coupon.id);
        let base_products = context.request::<Vec<BaseProduct>>(Method::Get, url, None).wait()?;
        let all_support_products = base_products.into_iter().flat_map(|b| {
            if let Some(variants) = b.variants {
                variants
            } else {
                vec![]
            }
            })
            .filter(|p|
                match p.discount {
                    Some(discount) => discount < ZERO_DISCOUNT,
                    None => true,
                })
            .map(|p| p.id).collect::<HashSet<ProductId>>();

        let all_cart_products:HashSet<ProductId> = current_cart.iter().map(|c| c.product_id).collect();
        let products_for_cart:HashSet<ProductId> = all_cart_products.intersection(&all_support_products).cloned().collect();

        if products_for_cart.is_empty() {
            return Err(FieldError::new(
                "Coupon not set",
                graphql_value!({ "code": 400, "details": { "no products found for coupon usage" }}),
            ));
        }

        for product_id in products_for_cart {
            rpc_client.add_coupon(customer, product_id, coupon.id).sync()?;
        }

        let products: Vec<_> = rpc_client.get_cart(customer, currency_type).sync()
            .map_err(into_graphql)?
            .into_iter().collect();

        cart_module::convert_products_to_cart(context, &products, Some(input.user_country_code)).map(Some)
    }

    field deprecated "use deleteCouponFromCartV2" deleteCouponFromCart(
        &executor,
        input: DeleteCouponInCartInput as "Delete coupon from cart input.",
    ) -> FieldResult<Option<Cart>> as "Delete base product from coupon." {
        let context = executor.context();

        let customer = if let Some(ref user) = context.user {
            user.user_id.into()
        } else if let Some(session_id) = context.session_id {
            session_id.into()
        }  else {
            return Err(FieldError::new(
                "Could not set coupon in cart for unauthorized user.",
                graphql_value!({ "code": 100, "details": { "No user id in request header." }}),
            ));
        };

        let coupon_id = match (input.coupon_id, input.coupon_code) {
            (Some(coupon_id), _) => CouponId(coupon_id),
            (None, Some(by_code)) => {
                get_coupon_by_code(context, CouponCode(by_code.coupon_code), StoreId(by_code.store_id))?.id
            },
            (None, None) => return Err(FieldError::new(
                "Could not delete coupon from cart could not identify coupon.",
                graphql_value!({ "code": 100, "details": { "Either coupon_code or coupon_id must be present." }}),
            ))
        };

        let rpc_client = context.get_rest_api_client(Service::Orders);
        let products: Vec<CartItem> = rpc_client.delete_coupon(customer, coupon_id).sync()?
            .into_iter().collect();

        cart_module::convert_products_to_cart(context, &products, None).map(Some)
    }

    field deleteCouponFromCartV2(
        &executor,
        input: DeleteCouponInCartInputV2 as "Delete coupon from cart input.",
    ) -> FieldResult<Option<Cart>> as "Delete base product from coupon." {
        let context = executor.context();

        let customer = if let Some(ref user) = context.user {
            user.user_id.into()
        } else if let Some(session_id) = context.session_id {
            session_id.into()
        }  else {
            return Err(FieldError::new(
                "Could not set coupon in cart for unauthorized user.",
                graphql_value!({ "code": 100, "details": { "No user id in request header." }}),
            ));
        };

        let coupon_id = match (input.coupon_id, input.coupon_code) {
            (Some(coupon_id), _) => CouponId(coupon_id),
            (None, Some(by_code)) => {
                get_coupon_by_code(context, CouponCode(by_code.coupon_code), StoreId(by_code.store_id))?.id
            },
            (None, None) => return Err(FieldError::new(
                "Could not delete coupon from cart could not identify coupon.",
                graphql_value!({ "code": 100, "details": { "Either coupon_code or coupon_id must be present." }}),
            ))
        };

        let rpc_client = context.get_rest_api_client(Service::Orders);
        let products: Vec<CartItem> = rpc_client.delete_coupon(customer, coupon_id).sync()?
            .into_iter().collect();

        cart_module::convert_products_to_cart(context, &products, Some(input.user_country_code)).map(Some)
    }

    field setSelectionInCart(
        &executor,
        input: SetSelectionInCartInput as "Select product in cart input."
    ) -> FieldResult<Option<Cart>> as "Select product in cart." {
        let context = executor.context();

        let customer = if let Some(ref user) = context.user {
            user.user_id.into()
        } else if let Some(session_id) = context.session_id {
            session_id.into()
        }  else {
            return Err(FieldError::new(
                "Could not select item in cart for unauthorized user.",
                graphql_value!({ "code": 100, "details": { "No user id in request header." }}),
            ));
        };

        let rpc_client = context.get_rest_api_client(Service::Orders);
        let products: Vec<_> = rpc_client.set_selection(customer, input.product_id.into(), input.value)
            .sync()
            .map_err(into_graphql)?
            .into_iter().collect();

        cart_module::convert_products_to_cart(context, &products, None).map(Some)
    }

    field setSelectionInCartV2(
        &executor,
        input: SetSelectionInCartInputV2 as "Select product in cart input.",
    ) -> FieldResult<Option<Cart>> as "Select product in cart." {
        let context = executor.context();

        let customer = if let Some(ref user) = context.user {
            user.user_id.into()
        } else if let Some(session_id) = context.session_id {
            session_id.into()
        }  else {
            return Err(FieldError::new(
                "Could not select item in cart for unauthorized user.",
                graphql_value!({ "code": 100, "details": { "No user id in request header." }}),
            ));
        };

        let rpc_client = context.get_rest_api_client(Service::Orders);
        let products: Vec<_> = rpc_client.set_selection(customer, input.product_id.into(), input.value)
            .sync()
            .map_err(into_graphql)?
            .into_iter().collect();

        cart_module::convert_products_to_cart(context, &products, Some(input.user_country_code)).map(Some)

    }

    field setCommentInCart(&executor, input: SetCommentInCartInput as "Set comment in cart input.")
        -> FieldResult<Option<Cart>> as "Set comment in cart." {

        let context = executor.context();

        let customer = if let Some(ref user) = context.user {
            user.user_id.into()
        } else if let Some(session_id) = context.session_id {
            session_id.into()
        }  else {
            return Err(FieldError::new(
                "Could not comment item in cart for unauthorized user.",
                graphql_value!({ "code": 100, "details": { "No user id in request header." }}),
            ));
        };

        let rpc_client = context.get_rest_api_client(Service::Orders);
        let products:Vec<_> = rpc_client.set_comment(customer, input.product_id.into(), input.value)
            .sync()
            .map_err(into_graphql)?
            .into_iter().collect();

        cart_module::convert_products_to_cart(context, &products, None).map(Some)
    }

    field setCommentInCartV2(&executor, input: SetCommentInCartInputV2 as "Set comment in cart input.")
        -> FieldResult<Option<Cart>> as "Set comment in cart." {

        let context = executor.context();

        let customer = if let Some(ref user) = context.user {
            user.user_id.into()
        } else if let Some(session_id) = context.session_id {
            session_id.into()
        }  else {
            return Err(FieldError::new(
                "Could not comment item in cart for unauthorized user.",
                graphql_value!({ "code": 100, "details": { "No user id in request header." }}),
            ));
        };

        let rpc_client = context.get_rest_api_client(Service::Orders);
        let products:Vec<_> = rpc_client.set_comment(customer, input.product_id.into(), input.value)
            .sync()
            .map_err(into_graphql)?
            .into_iter().collect();

        cart_module::convert_products_to_cart(context, &products, Some(input.user_country_code)).map(Some)
    }

    field setDeliveryMethodInCart(
        &executor,
        input: SetDeliveryMethodInCartInput as "Set delivery method in cart input.",
    ) -> FieldResult<Cart> as "Sets delivery method in the cart." {
        let context = executor.context();

        cart_module::run_set_delivery_method_in_cart_v1(context, input)
    }

    field setDeliveryMethodInCartV2(
        &executor,
        input: SetDeliveryMethodInCartInputV2 as "Set delivery method in cart input.",
    ) -> FieldResult<Cart> as "Sets delivery method in the cart." {
        let context = executor.context();

        cart_module::run_set_delivery_method_in_cart(context, input)
    }

    field removeDeliveryMethodFromCart(
        &executor,
        input: RemoveDeliveryMethodFromCartInput as "Remove delivery method from cart input.",
    ) -> FieldResult<Cart> as "Removes delivery method from the cart." {
        let context = executor.context();

        cart_module::run_remove_delivery_method_from_cart_v1(context, input)

    }

    field removeDeliveryMethodFromCartV2(
        &executor,
        input: RemoveDeliveryMethodFromCartInputV2 as "Remove delivery method from cart input.",
    ) -> FieldResult<Cart> as "Removes delivery method from the cart." {
        let context = executor.context();

        cart_module::run_remove_delivery_method_from_cart(context, input)

    }

    field deleteFromCart(
        &executor,
        input: DeleteFromCartInput as "Delete items from cart input.",
    ) -> FieldResult<Cart> as "Deletes products from cart." {
        let context = executor.context();

        let customer = if let Some(ref user) = context.user {
            user.user_id.into()
        } else if let Some(session_id) = context.session_id {
            session_id.into()
        }  else {
            return Err(FieldError::new(
                "Could not delete item from cart for unauthorized user.",
                graphql_value!({ "code": 100, "details": { "No user id in request header." }}),
            ));
        };

        let rpc_client = context.get_rest_api_client(Service::Orders);
        let products:Vec<_> = rpc_client.delete_item(customer, input.product_id.into())
            .sync()
            .map_err(into_graphql)?
            .into_iter().collect();

        cart_module::convert_products_to_cart(context, &products, None)
    }

    field deleteFromCartV2(
        &executor,
        input: DeleteFromCartInputV2 as "Delete items from cart input.",
    ) -> FieldResult<Cart> as "Deletes products from cart." {
        let context = executor.context();

        let customer = if let Some(ref user) = context.user {
            user.user_id.into()
        } else if let Some(session_id) = context.session_id {
            session_id.into()
        }  else {
            return Err(FieldError::new(
                "Could not delete item from cart for unauthorized user.",
                graphql_value!({ "code": 100, "details": { "No user id in request header." }}),
            ));
        };

        let rpc_client = context.get_rest_api_client(Service::Orders);
        let products:Vec<_> = rpc_client.delete_item(customer, input.product_id.into())
            .sync()
            .map_err(into_graphql)?
            .into_iter().collect();

        cart_module::convert_products_to_cart(context, &products, Some(input.user_country_code))
    }

    field clearCart(&executor) -> FieldResult<Cart> as "Clears cart." {
        let context = executor.context();

        let customer = if let Some(ref user) = context.user {
            user.user_id.into()
        } else if let Some(session_id) = context.session_id {
            session_id.into()
        }  else {
            return Err(FieldError::new(
                "Could not clear cart for unauthorized user.",
                graphql_value!({ "code": 100, "details": { "No user id in request header." }}),
            ));
        };

        let rpc_client = context.get_rest_api_client(Service::Orders);
        rpc_client.clear_cart(customer)
            .sync()
            .map_err(into_graphql)
            .map(|_| convert_to_cart(vec![], &[], None))
    }

    field clearCartV2(&executor, user_country_code: String as "User country code") -> FieldResult<Cart> as "Clears cart." {
        let context = executor.context();

        let customer = if let Some(ref user) = context.user {
            user.user_id.into()
        } else if let Some(session_id) = context.session_id {
            session_id.into()
        }  else {
            return Err(FieldError::new(
                "Could not clear cart for unauthorized user.",
                graphql_value!({ "code": 100, "details": { "No user id in request header." }}),
            ));
        };

        let rpc_client = context.get_rest_api_client(Service::Orders);
        rpc_client.clear_cart(customer)
            .sync()
            .map_err(into_graphql)
            .map(|_| convert_to_cart(vec![], &[], Some(user_country_code)))
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

    field deprecated "use deleteUserDeliveryAddressFull" deleteUserDeliveryAddress(&executor, id: i32 as "Raw id of delivery address") -> FieldResult<UserDeliveryAddress>  as "Deletes delivery address." {
        let context = executor.context();
        let url = format!("{}/{}/delivery_addresses/{}",
            context.config.service_url(Service::Users),
            Model::User.to_url(),
            id);

        context.request::<UserDeliveryAddress>(Method::Delete, url, None)
            .wait()
    }

    field createUserDeliveryAddressFull(&executor, input: NewUserDeliveryAddressFullInput  as "Create delivery address full input.") -> FieldResult<UserDeliveryAddress> as "Creates new user delivery address full." {
        let context = executor.context();
        let url = format!("{}/{}/addresses",
            context.config.service_url(Service::Delivery),
            Model::User.to_url());

        let body: String = serde_json::to_string(&input)?.to_string();

        context.request::<UserDeliveryAddress>(Method::Post, url, Some(body))
            .wait()
    }

    field updateUserDeliveryAddressFull(&executor, input: UpdateUserDeliveryAddressFullInput as "Update delivery address full input.") -> FieldResult<UserDeliveryAddress>  as "Updates delivery address full."{
        let context = executor.context();

        if input.is_none() {
             return Err(FieldError::new(
                "Nothing to update",
                graphql_value!({ "code": 300, "details": { "All fields to update are none." }}),
            ));
        }

        let delivery = context.get_delivery_microservice();
        delivery.update_user_delivery_address(input)
    }

    field deleteUserDeliveryAddressFull(&executor, id: i32 as "Raw id of delivery address") -> FieldResult<UserDeliveryAddress>  as "Deletes delivery address." {
        let context = executor.context();
        let url = format!("{}/{}/addresses/{}",
            context.config.service_url(Service::Delivery),
            Model::User.to_url(),
            id);

        context.request::<UserDeliveryAddress>(Method::Delete, url, None)
            .wait()
    }

    field createWarehouse(&executor, input: CreateWarehouseInput as "Create warehouse input.") -> FieldResult<GraphQLWarehouse> as "Creates new warehouse." {
        let context = executor.context();
        let rpc_client = context.get_rest_api_client(Service::Warehouses);
        rpc_client.create_warehouse(input.into())
            .sync()
            .map_err(into_graphql)
            .map(GraphQLWarehouse)
    }

    field updateWarehouse(&executor, input: UpdateWarehouseInput as "Update Warehouse input.") -> FieldResult<Option<GraphQLWarehouse>>  as "Updates existing Warehouse."{
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

        let rpc_client = context.get_rest_api_client(Service::Warehouses);
        Uuid::parse_str(&input.id)
            .map_err(|_|
                FieldError::new(
                    "Given id can not be parsed as Uuid",
                    graphql_value!({ "parse_error": "Warehouse id must be uuid" })
                )
            )
            .and_then(|id|{
                rpc_client.update_warehouse(WarehouseId(id).into(), input.into())
                    .sync()
                    .map_err(into_graphql)
                    .map(|res| res.map(GraphQLWarehouse))
            })
    }

    field deleteWarehouse(&executor, id: String) -> FieldResult<Option<GraphQLWarehouse>>  as "Delete existing Warehouse." {
        let context = executor.context();
        Uuid::parse_str(&id)
            .map_err(|_|
                FieldError::new(
                    "Given id can not be parsed as Uuid",
                    graphql_value!({ "parse_error": "Warehouse id must be uuid" })
                )
            )
            .and_then(|id|{
                let rpc_client = context.get_rest_api_client(Service::Warehouses);
                rpc_client.delete_warehouse(WarehouseId(id).into())
                    .sync()
                    .map_err(into_graphql)
                    .map(|res| res.map(GraphQLWarehouse))
            })
    }

    field deleteAllWarehouses(&executor) -> FieldResult<Vec<GraphQLWarehouse>>  as "Delete all Warehouses." {
        let context = executor.context();
        let rpc_client = context.get_rest_api_client(Service::Warehouses);
        rpc_client.delete_all_warehouses()
            .sync()
            .map_err(into_graphql)
            .map(|res| res.into_iter().map(GraphQLWarehouse).collect())
    }

    field setProductQuantityInWarehouse(&executor, input: ProductQuantityInput as "set Product Quantity In Warehouse input.") -> FieldResult<GraphQLStock> as "Set Product Quantity In Warehouse" {
        let context = executor.context();
        Uuid::parse_str(&input.warehouse_id)
            .map_err(|_|
                FieldError::new(
                    "Given id can not be parsed as Uuid",
                    graphql_value!({ "parse_error": "Warehouse id must be uuid" })
                )
            )
            .and_then(|id|{
                let rpc_client = context.get_rest_api_client(Service::Warehouses);
                rpc_client.set_product_in_warehouse(WarehouseId(id), input.product_id.into(), input.quantity.into())
                    .sync()
                    .map_err(into_graphql)
                    .map(GraphQLStock)
            })
    }

    field createOrders(&executor, input: CreateOrderInput as "Create order input.") -> FieldResult<CreateOrdersOutput> as "Creates orders from cart." {
        let context = executor.context();

        order::run_create_orders_mutation_v1(context, input)
    }

    field createOrdersV2(&executor, input: CreateOrderInputV2 as "Create order input.") -> FieldResult<CreateOrdersOutput> as "Creates orders from cart." {
        let context = executor.context();

        order::run_create_orders_mutation(context, input)
    }

    field deprecated "use buyNowV2. This endpoint will return incorrect delivery price if it is not set to 'fixed price' by the store owner"
    buyNow(&executor, input: BuyNowInput as "Buy now input.") -> FieldResult<CreateOrdersOutput> as "Creates orders." {
        let context = executor.context();

        buy_now::run_buy_now_mutation_v1(context, input)
    }

    field buyNowV2(&executor, input: BuyNowInputV2 as "Buy now input.") -> FieldResult<CreateOrdersOutput> as "Creates orders." {
        let context = executor.context();

        buy_now::run_buy_now_mutation(context, input)
    }

    field setOrderStatusDelivery(&executor, input: OrderStatusDeliveryInput as "Order Status Delivery input.") -> FieldResult<Option<GraphQLOrder>>  as "Set Order Status Delivery."{
        let context = executor.context();
        let slug = input.order_slug;
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
        let url = format!("{}/{}/{}/set_state",
            context.config.saga_microservice.url.clone(),
            Model::Order.to_url(),
            slug,
            );

        let body = serde_json::to_string(&order)?;

        context.request::<Option<Order>>(Method::Post, url, Some(body))
            .map(|res| res.map(GraphQLOrder))
            .wait()
    }

    field setOrderStatusCanceled(&executor, input: OrderStatusCanceledInput as "Order Status Canceled input.") -> FieldResult<Option<GraphQLOrder>>  as "Set Order Status Canceled."{
        let context = executor.context();
        let slug = input.order_slug;
        let order: OrderStatusCanceled = input.into();
        let url = format!("{}/{}/{}/set_state",
            context.config.saga_microservice.url.clone(),
            Model::Order.to_url(),
            slug,
            );

        let body = serde_json::to_string(&order)?;

        context.request::<Option<Order>>(Method::Post, url, Some(body))
            .map(|res| res.map(GraphQLOrder))
            .wait()
    }

    field setOrderStatusComplete(&executor, input: OrderStatusCompleteInput as "Order Status Complete input.") -> FieldResult<Option<GraphQLOrder>>  as "Set Order Status Complete."{
        let context = executor.context();
        let slug = input.order_slug;
        let order: OrderStatusComplete = input.into();
        let url = format!("{}/{}/{}/set_state",
            context.config.saga_microservice.url.clone(),
            Model::Order.to_url(),
            slug,
            );

        let body = serde_json::to_string(&order)?;

        context.request::<Option<Order>>(Method::Post, url, Some(body))
            .map(|res| res.map(GraphQLOrder))
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

    field updateEmailTemplate(&executor,
        input: EmailTemplateInput as "Update EmailTemplate input.") -> FieldResult<String> as "Update email messages template" {
        let context = executor.context();

        let url = format!(
            "{}/templates/{}",
            &context.config.service_url(Service::Notifications),
            input.variant);

        let body: String = input.data;

        context.request::<String>(Method::Put, url, Some(body))
            .wait()
    }

    field upsertShipping(&executor, input: NewShippingInput as "New shipping input.") -> FieldResult<ShippingOutput> as "Upsert shipping for base product." {
        let context = executor.context();

        let rpc_client = context.get_rest_api_client(Service::Warehouses);
        let warehouses = rpc_client.get_warehouses_for_store(input.store_id.into())
            .sync()
            .map_err(into_graphql)?;

        let delivery_from = warehouses.into_iter().nth(0)
            .map(|warehouse|
                warehouse.country_code
                .ok_or_else(||
                    FieldError::new(
                        "Failed to update shipping options.",
                        graphql_value!({ "code": 100, "details": { "Country is not set in warehouse." }}),
                    )
                )
            )
            .ok_or_else(||
                FieldError::new(
                    "Failed to update shipping options.",
                    graphql_value!({ "code": 100, "details": { "Warehouses do not exist in stores microservice." }}),
                )
            )??;

        let local_delivery_to = delivery_from.clone();

        let base_product = base_product_module::try_get_base_product(context, BaseProductId(input.base_product_id), Visibility::Active)?
            .ok_or_else(|| {
                let details = format!("Base product with id: {} not found.", input.base_product_id);
                FieldError::new(
                    "Failed to update shipping options.",
                    graphql_value!({ "code": 300, "details": { details }}),
            )})?;


        let payload = NewShipping::from(NewShippingEnrichedInput {
            shipping: input,
            delivery_from,
            local_delivery_to,
            measurements: base_product.get_measurements(),
            base_product_currency: base_product.currency,
        });

        let saga  = context.get_saga_microservice();
        saga.upsert_shipping(base_product.id, payload).map(From::from)
    }

    field createCompany(&executor, input: NewCompanyInput as "Create company input.") -> FieldResult<Company> as "Creates new company." {
        let context = executor.context();
        let countries_url = format!("{}/{}/flatten", context.config.service_url(Service::Delivery), Model::Country.to_url());
        let all_countries = context.request::<Vec<Country>>(Method::Get, countries_url, None).wait()?;
        if !is_all_codes_valid(&all_countries, &input.deliveries_from) {
            return Err(FieldError::new(
                "Invalid country code.",
                graphql_value!({ "code": 100, "details": { "deliveries_from have invalid value(s)." }}),
            ));
        }

        let url = format!("{}/{}",
            context.config.service_url(Service::Delivery),
            Model::Company.to_url());

        let body: String = serde_json::to_string(&input)?.to_string();

        context.request::<Company>(Method::Post, url, Some(body))
            .wait()
    }

    field updateCompany(&executor, input: UpdateCompanyInput as "Update company input.") -> FieldResult<Company>  as "Updates company."{
        let context = executor.context();
        let identifier = ID::from_str(&*input.id)?;
        let url = identifier.url(&context.config);

        if input.is_none() {
             return Err(FieldError::new(
                "Nothing to update",
                graphql_value!({ "code": 300, "details": { "All fields to update are none." }}),
            ));
        }

        if let Some(deliveries_from) = &input.deliveries_from {
            let countries_url = format!("{}/{}/flatten", context.config.service_url(Service::Delivery), Model::Country.to_url());
            let all_countries = context.request::<Vec<Country>>(Method::Get, countries_url, None).wait()?;
            if !is_all_codes_valid(&all_countries, deliveries_from) {
                return Err(FieldError::new(
                    "Invalid country code.",
                    graphql_value!({ "code": 100, "details": { "deliveries_from have invalid value(s)." }}),
                ));
            }
        }

        let body: String = serde_json::to_string(&input)?.to_string();

        context.request::<Company>(Method::Put, url, Some(body))
            .wait()
    }

    field deleteCompany(&executor, id: i32 as "Raw id of company") -> FieldResult<Company>  as "Deletes company." {
        let context = executor.context();
        let url = format!("{}/{}/{}",
            context.config.service_url(Service::Delivery),
            Model::Company.to_url(),
            id);

        context.request::<Company>(Method::Delete, url, None)
            .wait()
    }

    field createPackage(&executor, input: NewPackagesInput as "Create package input.") -> FieldResult<Packages> as "Creates new package." {
        let context = executor.context();
        let countries_url = format!("{}/{}/flatten", context.config.service_url(Service::Delivery), Model::Country.to_url());
        let all_countries = context.request::<Vec<Country>>(Method::Get, countries_url, None).wait()?;
        if !is_all_codes_valid(&all_countries, &input.deliveries_to) {
            return Err(FieldError::new(
                "Invalid country code.",
                graphql_value!({ "code": 100, "details": { "deliveries_to have invalid value(s)." }}),
            ));
        }

        let url = format!("{}/{}",
            context.config.service_url(Service::Delivery),
            Model::Package.to_url());

        let body: String = serde_json::to_string(&input)?.to_string();

        context.request::<Packages>(Method::Post, url, Some(body))
            .wait()
    }

    field updatePackage(&executor, input: UpdatePackagesInput as "Update package input.") -> FieldResult<Packages>  as "Updates package."{
        let context = executor.context();
        let identifier = ID::from_str(&*input.id)?;
        let url = identifier.url(&context.config);

        if input.is_none() {
             return Err(FieldError::new(
                "Nothing to update",
                graphql_value!({ "code": 300, "details": { "All fields to update are none." }}),
            ));
        }

        if let Some(deliveries_to) = &input.deliveries_to {
            let countries_url = format!("{}/{}/flatten", context.config.service_url(Service::Delivery), Model::Country.to_url());
            let all_countries = context.request::<Vec<Country>>(Method::Get, countries_url, None).wait()?;
            if !is_all_codes_valid(&all_countries, deliveries_to) {
                return Err(FieldError::new(
                    "Invalid country code.",
                    graphql_value!({ "code": 100, "details": { "deliveries_to have invalid value(s)." }}),
                ));
            }
        }

        let body: String = serde_json::to_string(&input)?.to_string();

        context.request::<Packages>(Method::Put, url, Some(body))
            .wait()
    }

    field deletePackage(&executor, id: i32 as "Raw id of package") -> FieldResult<Packages>  as "Deletes package." {
        let context = executor.context();
        let url = format!("{}/{}/{}",
            context.config.service_url(Service::Delivery),
            Model::Package.to_url(),
            id);

        context.request::<Packages>(Method::Delete, url, None)
            .wait()
    }

    field addPackageToCompany(
        &executor,
        input: NewCompaniesPackagesInput as "Create company_package input.",
    ) -> FieldResult<CompaniesPackages> as "Creates new company_package." {
        let context = executor.context();
        let url = format!("{}/{}",
            context.config.service_url(Service::Delivery),
            Model::CompanyPackage.to_url());

        let body: String = serde_json::to_string(&NewCompaniesPackagesPayload::from(input))?.to_string();

        context.request::<CompaniesPackages>(Method::Post, url, Some(body))
            .wait()
    }

    field deleteCompanyPackage(&executor, company_id: i32 as "Raw id of company", package_id: i32 as "Raw id of package") -> FieldResult<CompaniesPackages>  as "Deletes company_package." {
        let context = executor.context();
        let url = format!("{}/{}/{}/{}/{}",
            context.config.service_url(Service::Delivery),
            Model::Company.to_url(),
            company_id,
            Model::Package.to_url(),
            package_id);

        context.request::<CompaniesPackages>(Method::Delete, url, None)
            .wait()
    }

    field createCoupon(&executor, input: NewCouponInput as "Create coupon input") -> FieldResult<Coupon> as "Creates new coupon." {
        let context = executor.context();
        let url = format!(
            "{}/{}",
            context.config.service_url(Service::Stores),
            Model::Coupon.to_url()
        );

        let body: String = serde_json::to_string(&NewCoupon::from(input))?.to_string();

        context.request::<Coupon>(Method::Post, url, Some(body))
            .wait()
    }

    field updateCoupon(&executor, input: UpdateCouponInput as "Update coupon input") -> FieldResult<Coupon> as "Updates coupon." {
        let context = executor.context();
        let identifier = ID::from_str(&*input.id)?;
        let url = identifier.url(&context.config);

        if input.is_none() {
             return Err(FieldError::new(
                "Nothing to update",
                graphql_value!({ "code": 300, "details": { "All fields to update are none." }}),
            ));
        }

        let body: String = serde_json::to_string(&UpdateCoupon::from(input))?.to_string();

        context.request::<Coupon>(Method::Put, url, Some(body))
            .wait()
    }

    field deleteCoupon(&executor, coupon_id: i32 as "Delete coupon by raw id") -> FieldResult<Coupon> as "Delete exists coupon." {
        let context = executor.context();
        let url = format!(
            "{}/{}/{}",
            context.config.service_url(Service::Stores),
            Model::Coupon.to_url(),
            coupon_id
        );

        context.request::<Coupon>(Method::Delete, url, None)
            .wait()
    }

    field addBaseProductToCoupon(&executor, input: ChangeBaseProductsInCoupon as "Add base product input") ->  FieldResult<Mock> as "Add base product to coupon." {
        let context = executor.context();
        let url = format!(
            "{}/{}/{}/{}/{}",
            context.config.service_url(Service::Stores),
            Model::Coupon.to_url(),
            input.raw_id,
            Model::BaseProduct.to_url(),
            input.raw_base_product_id,
        );

        context.request::<CouponScopeBaseProducts>(Method::Post, url, None)
            .wait()?;
        Ok(Mock{})
    }

    field deleteBaseProductFromCoupon(&executor, input: ChangeBaseProductsInCoupon as "Delete base product input") ->  FieldResult<Mock> as "Delete base product from coupon." {
        let context = executor.context();
        let url = format!(
            "{}/{}/{}/{}/{}",
            context.config.service_url(Service::Stores),
            Model::Coupon.to_url(),
            input.raw_id,
            Model::BaseProduct.to_url(),
            input.raw_base_product_id,
        );

        context.request::<CouponScopeBaseProducts>(Method::Delete, url, None)
            .wait()?;
        Ok(Mock{})
    }

    field replaceCategory(&executor, input: CategoryReplaceInput as "Category replace in base products input") ->  FieldResult<Vec<BaseProduct>> as "Category replace in base products." {
        let context = executor.context();

        category_module::run_replace_category(context, input)
    }

    field replaceShippingRates(
        &executor,
        input: ReplaceShippingRatesInput as "Replace shipping rates input",
    ) -> FieldResult<Vec<ShippingRates>> as "Replace shipping rates for a single 'from' country for a particular company-package" {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}/rates",
            &context.config.service_url(Service::Delivery),
            Model::CompanyPackage.to_url(),
            input.company_package_id,
        );

        let body = serde_json::to_string(&ReplaceShippingRatesPayload::from(input))?;

        context.request::<Vec<ShippingRates>>(Method::Post, url, Some(body)).wait()
    }
    field refreshJWT(&executor) -> FieldResult<String> as "Refresh JWT Token." {
        let context = executor.context();
        let url = format!("{}/{}/refresh",
            context.config.service_url(Service::Users),
            Model::JWT.to_url());

        if let Some(ref payload) = context.user {
            let body: String = serde_json::to_string(payload)?.to_string();

            context.request_without_auth::<String>(Method::Post, url, Some(body))
                .wait()
        } else {
             return Err(FieldError::new(
                "No jwt token in request header",
                graphql_value!({ "code": 300, "details": { "Nothing to refresh." }}),
            ));
        }

    }

    field revokeJWT(&executor) -> FieldResult<String> as "Revoke JWT Tokens." {
        let context = executor.context();
        let url = format!("{}/{}/revoke",
            context.config.service_url(Service::Users),
            Model::JWT.to_url());

        if let Some(ref payload) = context.user {
            let body: String = serde_json::to_string(payload)?.to_string();

            context.request::<String>(Method::Post, url, Some(body))
                .wait()
        } else {
             return Err(FieldError::new(
                "No jwt token in request header",
                graphql_value!({ "code": 300, "details": { "Can not revoke tokens for user, because no token in request header." }}),
            ));
        }
    }

    field confirmOrder(&executor,
                        input: OrderConfirmedInput as "Confirm order input object", ) -> FieldResult<Option<GraphQLOrder>> as "Confirm order for seller" {
        let context = executor.context();

        order::run_confirm_order_mutation(context, input)
    }

    field createCustomerWithSource(&executor,
                            input: CreateCustomerWithSourceInput as "Creates Customer object in Stripe",) -> FieldResult<Customer> as "Creates Customer object" {
        let context = executor.context();

        stripe_module::run_create_customer_with_source_mutation(context, input)
    }

    field updateCustomer(&executor, input: UpdateCustomerInput as "Update customer object in Stripe") -> FieldResult<Customer> as "Updates Customer object" {
        let context = executor.context();

        stripe_module::run_update_customer_mutation(context, input)
    }

    field deleteCustomer(&executor,
                            input: DeleteCustomerInput as "Delete Customer object in Stripe",)
                            -> FieldResult<Mock> as "" {
        let context = executor.context();

        stripe_module::run_delete_customer_mutation(context, input).map(|_| Mock)
    }

    field setPaidToSellerOrderState(&executor,
                                    input: PaidToSellerOrderStateInput as "Confirmation by the financier that the money is transferred to the seller.",
                                    ) -> FieldResult<Mock> as "" {
        let context = executor.context();

        order::run_set_paid_to_seller_order_state_mutation(context, input).map(|_| Mock)
    }

    field createInternationalBillingInfo(&executor, input: NewInternationalBillingInfoInput as "Create international billing info for a store")
    -> FieldResult<InternationalBillingInfo> as "Created international billing info" {
        let context = executor.context();

        let billing = context.get_billing_microservice();
        billing.create_international_billing_info(input)
    }

    field updateInternationalBillingInfo(&executor, input: UpdateInternationalBillingInfoInput as "Update international billing info for a store")
    -> FieldResult<InternationalBillingInfo> as "Updated international billing info" {
        let context = executor.context();

        let billing = context.get_billing_microservice();
        billing.update_international_billing_info(input)
    }

    field createRussiaBillingInfo(&executor, input: NewRussiaBillingInfoInput as "Create russia billing info for a store")
    -> FieldResult<RussiaBillingInfo> as "Created russia billing info" {
        let context = executor.context();

        let billing = context.get_billing_microservice();
        billing.create_russia_billing_info(input)
    }

    field updateRussiaBillingInfo(&executor, input: UpdateRussiaBillingInfoInput as "Update russia billing info for a store")
    -> FieldResult<RussiaBillingInfo> as "Updated russia billing info" {
        let context = executor.context();

        let billing = context.get_billing_microservice();
        billing.update_russia_billing_info(input)
    }

    field ChargeFee(&executor,
                            input: ChargeFeeInput as "Creates Charge object in Stripe for pay fee-for-service platform.",) -> FieldResult<Fee> as "Creates Charge object" {
        let context = executor.context();

        order::run_charge_fee_mutation(context, input)
    }

    field ChargeFees(&executor,
                     input: ChargeFeesInput as "Creates Charge object in Stripe for pay fee-for-service platform.",) -> FieldResult<Vec<Fee>> as "Creates Charge object" {
        let context = executor.context();

        order::run_charge_fees_mutation(context, input)
    }

    field createPaymentIntentFee(&executor, input: CreatePaymentIntentFeeInput as "Create payment intent for fee input") -> FieldResult<PaymentIntent> as "Create payment intent for fee" {
        executor.context()
            .get_billing_microservice()
            .create_payment_intent_fee(FeeId::new(input.fee_id))
    }

    field createDispute(
        &executor,
        input: CreateDisputeInput as "Open dispute by order.",
    ) -> FieldResult<Mock> as "" {
        let context = executor.context();

        order::run_create_dispute_mutation(context, input).map(|_| Mock)
    }

    field payOutCryptoToSeller(
        &executor,
        input: PayOutCryptoToSellerInput,
    ) -> FieldResult<Payout> as "Payout info" {
        let context = executor.context();

        payout::run_pay_out_crypto_to_seller_mutation(context, input)
    }
});
