//! File containing mutations object of graphql schema
use std::collections::HashMap;
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
use stq_static_resources::{Currency, Provider};
use stq_types::{CartItem, CouponCode, CouponId, ProductId, ProductSellerPrice, Quantity, SagaId, StoreId, WarehouseId};

use errors::into_graphql;
use graphql::models::user::get_user_by_id;

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
            gender: None,
            birthdate: None,
            last_login_at: SystemTime::now(),
            saga_id: SagaId::new(),
        };
        let saga_profile = SagaCreateProfile {
            identity: new_ident,
            user: Some(new_user),
            device: input.device,
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

    field unblockUser(&executor, id: i32 as "Users raw id.") -> FieldResult<User>  as "Unblock existing user." {
        let context = executor.context();
        let url = format!("{}/{}/{}/unblock",
            context.config.service_url(Service::Users),
            Model::User.to_url(),
            id);

        context.request::<User>(Method::Post, url, None)
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
        let saga_addr = context.config.saga_microservice.url.clone();
        let url = format!("{}/email_verify_apply",
            saga_addr);
        let body = serde_json::to_string(&input)?;
        let token = context.request::<String>(Method::Post, url, Some(body))
            .wait()?;

        Ok(VerifyEmailApplyOutput {
            success: true,
            token,
        })
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
        let url = format!("{}/roles",
            context.config.service_url(Service::Stores));
        let body: String = serde_json::to_string(&input)?.to_string();

        context.request::<NewRole<StoresMicroserviceRole>>(Method::Post, url, Some(body))
            .wait()
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
        let url = format!("{}/roles",
            context.config.service_url(Service::Stores));
        let body: String = serde_json::to_string(&input)?.to_string();

        context.request::<NewRole<StoresMicroserviceRole>>(Method::Delete, url, Some(body))
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

    field publishStore(&executor, id: i32 as "Store raw id.") -> FieldResult<Store>  as "Publish store." {
        let context = executor.context();
        let url = format!("{}/{}/{}/publish",
            context.config.service_url(Service::Stores),
            Model::Store.to_url(),
            id);

        context.request::<Store>(Method::Post, url, None)
            .wait()
    }

    field draftStore(&executor, id: i32 as "Store raw id.") -> FieldResult<Store>  as "Draft store." {
        let context = executor.context();
        let url = format!("{}/{}/{}/draft",
            context.config.service_url(Service::Stores),
            Model::Store.to_url(),
            id);

        context.request::<Store>(Method::Post, url, None)
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

    field createBaseProductWithVariants(&executor, input: NewBaseProductWithVariantsInput as "Create base product with variants input.") -> FieldResult<BaseProduct> as "Creates new base product with variants." {
        let context = executor.context();
        let url = format!("{}/{}/with_variants",
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

    field publishBaseProducts(&executor, ids: Vec<i32> as "BaseProduct raw ids.") -> FieldResult<Vec<BaseProduct>>  as "Published base_products." {
        let context = executor.context();
        let url = format!("{}/{}/publish",
            context.config.service_url(Service::Stores),
            Model::BaseProduct.to_url());

        let body: String = serde_json::to_string(&ids)?.to_string();

        context.request::<Vec<BaseProduct>>(Method::Post, url, Some(body))
            .wait()
    }

    field draftBaseProducts(&executor, ids: Vec<i32> as "BaseProduct raw ids.") -> FieldResult<Vec<BaseProduct>>  as "Draft base_products." {
        let context = executor.context();
        let url = format!("{}/{}/draft",
            context.config.service_url(Service::Stores),
            Model::BaseProduct.to_url());

        let body: String = serde_json::to_string(&ids)?.to_string();

        context.request::<Vec<BaseProduct>>(Method::Post, url, Some(body))
            .wait()
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

        let customer = if let Some(ref user) = context.user {
            user.user_id.into()
        } else if let Some(session_id) = context.session_id {
            session_id.into()
        }  else {
            return Err(FieldError::new(
                "Could not increment cart for unauthorized user.",
                graphql_value!({ "code": 100, "details": { "No user id in request header." }}),
            ));
        };

        let url = format!("{}/{}/by_product/{}",
            context.config.service_url(Service::Stores),
            Model::BaseProduct.to_url(),
            input.product_id);
        let base_product = context.request::<Option<BaseProduct>>(Method::Get, url, None)
            .wait()?
            .ok_or_else(||
                FieldError::new(
                    "Could not find base product for product id.",
                    graphql_value!({ "code": 100, "details": { "Base product does not exist in stores microservice." }}),
            ))?;
        let product = base_product.variants.and_then(|v| v.get(0).cloned()).ok_or_else(||
                FieldError::new(
                    "Could not find product in base product variants.",
                    graphql_value!({ "code": 100, "details": { "Product does not exist in variants." }}),
            ))?;

        let rpc_client = context.get_rest_api_client(Service::Orders);

        let products: Vec<_> = rpc_client.increment_item(customer, input.product_id.into(), base_product.store_id, product.pre_order, product.pre_order_days)
            .sync()
            .map_err(into_graphql)
            .and_then(|p| {
                if let Some(value) = input.value {
                    let quantity = Quantity(value);

                    rpc_client.set_quantity(customer, input.product_id.into(), quantity)
                    .sync()
                    .map_err(into_graphql)
                } else {
                    Ok(p)
                }
            })?
            .into_iter().collect();

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

        let url = format!("{}/{}/cart",
            context.config.service_url(Service::Stores),
            Model::Store.to_url());

        let body = serde_json::to_string(&products)?;

        context.request::<Vec<Store>>(Method::Post, url, Some(body))
            .map(|stores| convert_to_cart(stores, &products))
            .map(Some)
            .wait()
    }

    field setCouponInCart(&executor, input: SetCouponInCartInput as "Set coupon in cart input.") -> FieldResult<Option<Cart>> as "Sets coupon in cart." {
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
        let current_cart = rpc_client.get_cart(customer).sync()?;

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

        let products: Vec<_> = rpc_client.get_cart(customer).sync()
            .map_err(into_graphql)?
            .into_iter().collect();

        let url = format!("{}/{}/cart",
            context.config.service_url(Service::Stores),
            Model::Store.to_url());

        let body = serde_json::to_string(&products)?;

        context.request::<Vec<Store>>(Method::Post, url, Some(body))
            .map(|stores| convert_to_cart(stores, &products))
            .map(Some)
            .wait()
    }

    field deleteCouponFromCart(
        &executor, input: DeleteCouponInCartInput as "Delete coupon from cart input."
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
            .map(|_| convert_to_cart(vec![], &[]))
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
        let url = format!("{}/{}/addresses/{}",
            context.config.service_url(Service::Delivery),
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

        let user = context.user.clone().ok_or_else(||
            FieldError::new(
                "Could not create cart for unauthorized user.",
                graphql_value!({ "code": 100, "details": { "No user id in request header." }}),
            )
        )?;

        let rpc_client = context.get_rest_api_client(Service::Orders);
        let current_cart = rpc_client.get_cart(user.user_id.into()).sync().map_err(into_graphql)?;

        for cart_item in current_cart.iter() {
            if let Some(coupon_id) = cart_item.coupon_id {
                validate_coupon(context, coupon_id)?;
            }
        }

        let products_with_prices = current_cart.iter()
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
                .collect::<FieldResult<CartProductWithPriceHash>>()?;

        if products_with_prices.len() == 0  {
            return Err(FieldError::new(
                "Could not create orders for empty cart.",
                graphql_value!({ "code": 100, "details": { "There is no products, selected in cart." }}),
            ));
        }

        let mut coupons_info = vec![];
        for cart_item in current_cart.iter() {
            if let Some(coupon_id) = cart_item.coupon_id {
                try_get_coupon(context, coupon_id)?.map(|coupon|{
                    coupons_info.push(coupon);
                });
            }
        }

        let coupons_info = coupons_info.into_iter()
            .map(|coupon| {
                (coupon.id, coupon)
            })
            .collect::<HashMap<CouponId, Coupon>>();

        let create_order = CreateOrder {
            customer_id: user.user_id,
            address: input.address_full,
            receiver_name: input.receiver_name,
            receiver_phone: input.receiver_phone,
            prices: products_with_prices,
            currency: input.currency,
            coupons: coupons_info,
        };

        let url = format!("{}/create_order",
            context.config.saga_microservice.url.clone());

        let body: String = serde_json::to_string(&create_order)?.to_string();

        context.request::<Invoice>(Method::Post, url, Some(body))
            .wait()
            .map(CreateOrdersOutput)
    }

    field buyNow(&executor, input: BuyNowInput as "Buy now input.") -> FieldResult<CreateOrdersOutput> as "Creates orders." {
        let context = executor.context();

        let user = context.user.clone().ok_or_else(||
            FieldError::new(
                "Could not create cart for unauthorized user.",
                graphql_value!({ "code": 100, "details": { "No user id in request header." }}),
            )
        )?;

        let url = format!("{}/{}/{}/seller_price",
                        context.config.service_url(Service::Stores),
                        Model::Product.to_url(),
                        input.product_id);

        let product_price = context.request::<Option<ProductSellerPrice>>(Method::Get, url, None)
        .wait()
            .and_then(|seller_price|{
            if let Some(seller_price) = seller_price {
                Ok(seller_price)
            } else {
                Err(FieldError::new(
                    "Could not find product seller price from product id.",
                    graphql_value!({ "code": 100, "details": { "Product with such id does not exist in stores microservice." }}),
                    ))
            }
        })?;

        let url_store_id = format!("{}/{}/store_id?product_id={}",
                        context.config.service_url(Service::Stores),
                        Model::Product.to_url(),
                        input.product_id);

        let store_id = context.request::<Option<StoreId>>(Method::Get, url_store_id, None)
        .wait()
            .and_then(|id|{
            if let Some(id) = id {
                Ok(id)
            } else {
                Err(FieldError::new(
                    "Could not find store_id from product id.",
                    graphql_value!({ "code": 100, "details": { "Product with such id does not exist in stores microservice." }}),
                    ))
            }
        })?;

        let url_product = format!("{}/{}/{}",
                        context.config.service_url(Service::Stores),
                        Model::Product.to_url(),
                        input.product_id);

        let product = context.request::<Option<Product>>(Method::Get, url_product, None)
        .wait()
            .and_then(|value|{
            if let Some(value) = value {
                Ok(value)
            } else {
                Err(FieldError::new(
                    "Could not find Product from product id.",
                    graphql_value!({ "code": 100, "details": { "Product with such id does not exist in stores microservice." }}),
                    ))
            }
        })?;

        let coupon = match input.coupon_code {
            Some(code) => {
                let coupon_code = CouponCode(code);
                validate_coupon_by_code(context, coupon_code.clone(), store_id)?;
                Some(get_coupon_by_code(context, coupon_code, store_id)?)
            },
            None => None,
        };

        let customer = get_user_by_id(
            context.config.service_url(Service::Users),
            context.http_client.clone(),
            user.user_id
        )?;

        let buy_now = BuyNow {
            product_id: input.product_id.into(),
            store_id,
            customer_id: user.user_id,
            address: input.address_full,
            receiver_name: input.receiver_name,
            receiver_phone: input.receiver_phone,
            receiver_email: customer.email,
            price: product_price,
            quantity: input.quantity.into(),
            currency: input.currency,
            pre_order: product.pre_order,
            pre_order_days: product.pre_order_days,
            coupon,
        };

        let url = format!("{}/buy_now",
            context.config.saga_microservice.url.clone());

        let body: String = serde_json::to_string(&buy_now)?.to_string();

        context.request::<Invoice>(Method::Post, url, Some(body))
            .wait()
            .map(CreateOrdersOutput)
    }

    field createOrdersFiat(&executor, input: CreateOrderFiatInput as "Create order input.") -> FieldResult<CreateOrdersOutput> as "Creates orders from cart with FIAT billing." {
        let context = executor.context();

        let user = context.user.clone().ok_or_else(||
            FieldError::new(
                "Could not create cart for unauthorized user.",
                graphql_value!({ "code": 100, "details": { "No user id in request header." }}),
            )
        )?;

        let rpc_client = context.get_rest_api_client(Service::Orders);
        let products_with_prices = rpc_client.get_cart(user.user_id.into())
            .sync()
            .map_err(into_graphql)
            .and_then (|hash|
                hash.into_iter()
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
                .collect::<FieldResult<CartProductWithPriceHash>>()
            )?;

        //need a check that all products have the same currency - USD, EUR, RUB
        let products_currencies = products_with_prices.iter().map(|(id, price)| price.currency).collect::<HashSet<Currency>>();
        if products_currencies.len() == 0  {
            return Err(FieldError::new(
                "Could not create fiat orders for empty cart.",
                graphql_value!({ "code": 100, "details": { "There is no products, selected in cart." }}),
            ));
        } else if products_currencies.len() > 1 {
            return Err(FieldError::new(
                "Could not create fiat orders, products currencies in cart are not equal.",
                graphql_value!({ "code": 100, "details": { "There is more than one currency for products, selected in cart." }}),
            ));
        } else if !products_currencies.contains(&Currency::EUR) | !products_currencies.contains(&Currency::USD) | !products_currencies.contains(&Currency::RUB) {
            return Err(FieldError::new(
                "Could not create fiat orders, products currencies are not fiat.",
                graphql_value!({ "code": 100, "details": { "Products, selected in cart are selling not in EUR, USD, RUB." }}),
            ));
        }

        let create_order = CreateOrderFiat {
            customer_id: user.user_id,
            address: input.address_full,
            receiver_name: input.receiver_name,
            receiver_phone: input.receiver_phone,
            prices: products_with_prices,
        };

        let url = format!("{}/create_order_fiat",
            context.config.saga_microservice.url.clone());

        let body: String = serde_json::to_string(&create_order)?.to_string();

        context.request::<Invoice>(Method::Post, url, Some(body))
            .wait()
            .map(CreateOrdersOutput)
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

        let deliveries_to = warehouses.into_iter().nth(0)
            .map(|warehouse|
                warehouse.country_code
                .ok_or_else(||
                    FieldError::new(
                        "Could not find country for warehouse.",
                        graphql_value!({ "code": 100, "details": { "Country does not set in warehouse." }}),
                    )
                )
            )
            .ok_or_else(||
                FieldError::new(
                    "Could not find warehouse for store.",
                    graphql_value!({ "code": 100, "details": { "Warehouses do not exist in stores microservice." }}),
                )
            )?;

        let url = format!("{}/{}/{}",
            context.config.service_url(Service::Delivery),
            Model::Product.to_url(),
            input.base_product_id);

        let input : NewShipping = (input, deliveries_to?).into();

        let body: String = serde_json::to_string(&input)?.to_string();

        context.request::<Shipping>(Method::Post, url, Some(body))
            .map(From::from)
            .wait()
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

    field addPackageToCompany(&executor, input: NewCompaniesPackagesInput as "Create company_package input.") -> FieldResult<CompaniesPackages> as "Creates new company_package." {
        let context = executor.context();
        let url = format!("{}/{}",
            context.config.service_url(Service::Delivery),
            Model::CompanyPackage.to_url());

        let body: String = serde_json::to_string(&input)?.to_string();

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

});

pub struct Mutation;
