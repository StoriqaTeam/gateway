//! File containing product object of graphql schema
use std::cmp;
use std::str::FromStr;

use chrono::prelude::*;
use futures::Future;
use hyper::Method;
use juniper;
use juniper::ID as GraphqlID;
use juniper::{FieldError, FieldResult};

use stq_api::types::ApiFutureExt;
use stq_api::warehouses::WarehouseClient;
use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_static_resources::{Currency, ModerationStatus, Translation};
use stq_types::BaseProductId;

use super::*;
use errors::into_graphql;
use graphql::context::Context;
use graphql::models::*;

graphql_object!(BaseProduct: Context as "BaseProduct" |&self| {
    description: "Base Product's info."

    interfaces: [&Node]

    field id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Stores, Model::BaseProduct, self.id.0).to_string().into()
    }

    field raw_id() -> &i32 as "Unique int id"{
        &self.id.0
    }

    field name() -> &[Translation] as "Full Name" {
        &self.name
    }

    field is_active() -> bool as "If the product was disabled (deleted), isActive is false" {
        self.is_active
    }

    field short_description() -> &[Translation] as "Short description" {
        &self.short_description
    }

    field long_description() -> &Option<Vec<Translation>> as "Long Description" {
        &self.long_description
    }

    field seo_title() -> &Option<Vec<Translation>> as "SEO title" {
        &self.seo_title
    }

    field seo_description() -> &Option<Vec<Translation>> as "SEO Description" {
        &self.seo_description
    }

    field currency() -> &Currency as "Currency" {
        &self.currency
    }

    field rating() -> &f64 as "Rating" {
        &self.rating
    }

    field status() -> &ModerationStatus as "Moderation Status" {
        &self.status
    }

    field store_id(&executor) -> &i32 as "Raw store id"{
        &self.store_id.0
    }

    field category_id(&executor) -> &i32 as "Category Id"{
        &self.category_id.0
    }

    field created_at() -> String as "Created at" {
        let datetime: DateTime<Utc> = self.created_at.into();
        datetime.to_rfc3339()
    }

    field updated_at() -> String as "Updated at" {
        let datetime: DateTime<Utc> = self.updated_at.into();
        datetime.to_rfc3339()
    }

    field moderator_comment(&executor) -> FieldResult<Option<ModeratorProductComments>> as "Fetches moderator comment by id." {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Stores),
            Model::ModeratorProductComment.to_url(),
            self.id.to_string()
        );

        context.request::<Option<ModeratorProductComments>>(Method::Get, url, None)
            .wait()
    }

    field store(&executor,
        visibility: Option<Visibility> as "Specifies allowed visibility of the store"
    ) -> FieldResult<Option<Store>> as "Fetches store by id." {

        let context = executor.context();
        let visibility = visibility.unwrap_or_default();

        let url = format!(
            "{}/{}/{}?visibility={}",
            &context.config.service_url(Service::Stores),
            Model::Store.to_url(),
            self.store_id.to_string(),
            visibility
        );

        context.request::<Option<Store>>(Method::Get, url, None)
            .wait()
    }

    field category(&executor) -> FieldResult<Option<Category>> as "Category" {
        let context = executor.context();
        let url = format!("{}/{}/{}",
            context.config.service_url(Service::Stores),
            Model::Category.to_url(),
            self.category_id.0);

        context.request::<Option<Category>>(Method::Get, url, None)
            .wait()
    }

    field deprecated "Use products instead" variants(&executor) -> FieldResult<Option<Variants>> as "Variants" {
        let context = executor.context();
        if let Some(ref variants) = self.variants {
            Ok(Some(Variants::new(variants.clone())))
        } else {
            let url = format!("{}/{}/by_base_product/{}",
                context.config.service_url(Service::Stores),
                Model::Product.to_url(),
                self.id);

            context.request::<Vec<Product>>(Method::Get, url, None)
                .wait()
                .or_else(|_| Ok(vec![]))
                .map(|u| Some(Variants::new(u)))
        }

    }

    field products(&executor,
        first = None : Option<i32> as "First edges",
        after = None : Option<GraphqlID>  as "Offset from begining")
            -> FieldResult<Option<Connection<Product, PageInfo>>> as "Fetches products using relay connection." {
        let context = executor.context();

        let offset = after
            .and_then(|id|{
                i32::from_str(&id).map(|i| i + 1).ok()
            })
            .unwrap_or_default();

        let records_limit = context.config.gateway.records_limit;
        let first = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        if let Some(ref variants) = self.variants {
            let mut product_edges: Vec<Edge<Product>> = variants.clone()
                .into_iter()
                .skip(offset as usize)
                .take(first as usize)
                .map(|product| Edge::new(
                            juniper::ID::from(ID::new(Service::Stores, Model::Product, product.id.0).to_string()),
                            product.clone()
                        ))
                .collect();
            let has_next_page = product_edges.len() as i32 > first;
            let has_previous_page = true;
            let start_cursor =  product_edges.get(0).map(|e| e.cursor.clone());
            let end_cursor = product_edges.iter().last().map(|e| e.cursor.clone());
            let page_info = PageInfo {
                has_next_page,
                has_previous_page,
                start_cursor,
                end_cursor};
            Ok(Some(Connection::new(product_edges, page_info)))
        } else {
            let url = format!("{}/{}/by_base_product/{}",
                context.config.service_url(Service::Stores),
                Model::Product.to_url(),
                self.id);

            context.request::<Vec<Product>>(Method::Get, url, None)
            .map (|products| {
                let mut product_edges: Vec<Edge<Product>> = products
                    .into_iter()
                    .skip(offset as usize)
                    .take(first as usize)
                    .map(|product| Edge::new(
                                juniper::ID::from(ID::new(Service::Stores, Model::Product, product.id.0).to_string()),
                                product.clone()
                            ))
                    .collect();
                let has_next_page = product_edges.len() as i32 > first;
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
    }

    field views() -> &i32 as "Views" {
        &self.views
    }

    field slug() -> &str as "Slug" {
        &self.slug
    }

    field custom_attributes(&executor) -> FieldResult<Vec<CustomAttribute>> as "Custom attributes" {
        let context = executor.context();
        let url = format!("{}/{}/{}/{}",
            context.config.service_url(Service::Stores),
            Model::BaseProduct.to_url(),
            self.id,
            Model::CustomAttribute.to_url(),
            );

        context.request::<Vec<CustomAttribute>>(Method::Get, url, None)
            .map(From::from)
            .wait()
    }

    field available_packages(&executor) -> FieldResult<AvailablePackagesOutput> as "Available Packages" {
        let context = executor.context();

        let rpc_client = context.get_rest_api_client(Service::Warehouses);
        let warehouses = rpc_client.get_warehouses_for_store(self.store_id)
            .sync()
            .map_err(into_graphql)?;

        if let Some(warehouse) = warehouses.into_iter().nth(0) {
            if let Some(country_code) = warehouse.country_code {
                let url = format!("{}/available_packages?country={}&weight={}&size={}",
                    context.config.service_url(Service::Delivery),
                    country_code.to_string(),
                    0, // TODO: replace with real weight
                    0  // TODO: replace with real size
                    );

                context.request::<Vec<AvailablePackages>>(Method::Get, url, None)
                    .map(From::from)
                    .wait()
            } else {
                Err(FieldError::new(
                    "There is no country in warehouse address belonging to this store",
                    graphql_value!({ "code": 300, "details": { "Could not fetch warehouse address info." }}),
                ))
            }
        } else {
            Err(FieldError::new(
                "There is no warehouses belonging to this store",
                    graphql_value!({ "code": 300, "details": { "Could not fetch warehouse address info." }}),
            ))
        }
    }

    field shipping(&executor) -> FieldResult<ShippingOutput> as "Shipping" {
        let context = executor.context();
        let url = format!("{}/{}/{}",
            context.config.service_url(Service::Delivery),
            Model::Product.to_url(),
            self.id.0
        );

        context.request::<Shipping>(Method::Get, url, None)
            .map(From::from)
            .wait()
    }

    field is_shipping_available(&executor)-> FieldResult<bool> as "Is shipping available" {
        let context = executor.context();
        let url = format!("{}/{}/{}",
            context.config.service_url(Service::Delivery),
            Model::Product.to_url(),
            self.id,
        );

        let available_shipping = context.request::<Shipping>(Method::Get, url, None)
            .wait()?;

        Ok(!available_shipping.items.is_empty())
    }

});

graphql_object!(Variants: Context as "BaseProductVariants" |&self| {
    description:"Base Product Variants"

    field all() -> &[Product] {
        &self.products
    }

    field most_discount() -> Option<&Product> {
        self.get_most_discount()
    }

    field first() -> Option<&Product> {
        self.get_first()
    }

});

graphql_object!(Connection<BaseProduct, PageInfo>: Context as "BaseProductsConnection" |&self| {
    description:"Base Products Connection"

    field edges() -> &[Edge<BaseProduct>] {
        &self.edges
    }

    field page_info() -> &PageInfo {
        &self.page_info
    }
});

graphql_object!(Connection<BaseProduct, PageInfoSegments>: Context as "BaseProductsConnectionPages" |&self| {
    description:"Base Products Connection"

    field edges() -> &[Edge<BaseProduct>] {
        &self.edges
    }

    field page_info() -> &PageInfoSegments {
        &self.page_info
    }
});

graphql_object!(Edge<BaseProduct>: Context as "BaseProductsEdge" |&self| {
    description:"Base Products Edge"

    field cursor() -> &juniper::ID {
        &self.cursor
    }

    field node() -> &BaseProduct {
        &self.node
    }
});

graphql_object!(Connection<BaseProduct, PageInfoProductsSearch>: Context as "BaseProductsSearchConnection" |&self| {
    description:"Base Products Connection"

    field edges() -> &[Edge<BaseProduct>] {
        &self.edges
    }

    field page_info() -> &PageInfoProductsSearch {
        &self.page_info
    }
});

pub fn try_get_base_product(context: &Context, base_product_id: BaseProductId, visibility: Visibility) -> FieldResult<Option<BaseProduct>> {
    let url = format!(
        "{}/{}/{}?visibility={}",
        &context.config.service_url(Service::Stores),
        Model::BaseProduct.to_url(),
        base_product_id.to_string(),
        visibility,
    );

    context.request::<Option<BaseProduct>>(Method::Get, url, None).wait()
}

pub fn get_base_product(context: &Context, base_product_id: BaseProductId, visibility: Visibility) -> FieldResult<BaseProduct> {
    try_get_base_product(context, base_product_id, visibility)?.ok_or_else(|| {
        FieldError::new(
            "Base product not found",
            graphql_value!({ "code": 400, "details": { "base product for this product not found" }}),
        )
    })
}

pub fn run_send_to_moderation_base_product(context: &Context, base_product_id: BaseProductId) -> FieldResult<BaseProduct> {
    let payload = BaseProductModerate {
        base_product_id,
        status: ModerationStatus::Moderation,
    };

    if validate_change_moderation_status(context, &payload)? {
        send_to_moderation(context, base_product_id)
    } else {
        Err(FieldError::new(
            "Could not change base product status.",
            graphql_value!({ "code": 100, "details": { "BaseProduct cannot be sent to moderation." }}),
        ))
    }
}

fn send_to_moderation(context: &Context, base_product_id: BaseProductId) -> FieldResult<BaseProduct> {
    let url = format!(
        "{}/{}/{}/moderation",
        context.config.saga_microservice.url.clone(),
        Model::BaseProduct.to_url(),
        base_product_id
    );

    let _ = context.request::<()>(Method::Post, url, None).wait()?;

    get_base_product(context, base_product_id, Visibility::Active)
}

pub fn run_moderation_status_base_product(context: &Context, input: BaseProductModerateInput) -> FieldResult<BaseProduct> {
    let identifier = ID::from_str(&*input.id)?;
    let base_product_id = BaseProductId(identifier.raw_id);

    let payload = BaseProductModerate {
        base_product_id,
        status: input.status,
    };

    if validate_change_moderation_status(context, &payload)? {
        send_to_moderate(context, payload)
    } else {
        Err(FieldError::new(
            "Could not change base product status.",
            graphql_value!({ "code": 100, "details": { "Base product cannot be changed." }}),
        ))
    }
}

fn send_to_moderate(context: &Context, payload: BaseProductModerate) -> FieldResult<BaseProduct> {
    let url = format!(
        "{}/{}/moderate",
        context.config.saga_microservice.url.clone(),
        Model::BaseProduct.to_url()
    );

    let body: String = serde_json::to_string(&payload)?.to_string();
    let _ = context.request::<()>(Method::Post, url, Some(body)).wait()?;

    get_base_product(context, payload.base_product_id, Visibility::Active)
}

pub fn run_draft_base_products_mutation(context: &Context, ids: Vec<i32>) -> FieldResult<Vec<BaseProduct>> {
    ids.into_iter()
        .map(BaseProductId)
        .map(|base_product_id| send_to_draft_base_product(context, base_product_id))
        .collect::<FieldResult<Vec<BaseProduct>>>()
}

fn send_to_draft_base_product(context: &Context, base_product_id: BaseProductId) -> FieldResult<BaseProduct> {
    let url = format!(
        "{}/{}/{}/draft",
        context.config.service_url(Service::Stores),
        Model::BaseProduct.to_url(),
        base_product_id
    );

    context.request::<BaseProduct>(Method::Post, url, None).wait()
}

fn validate_change_moderation_status(context: &Context, payload: &BaseProductModerate) -> FieldResult<bool> {
    let url = format!(
        "{}/{}/validate_change_moderation_status",
        context.config.service_url(Service::Stores),
        Model::BaseProduct.to_url()
    );

    let body: String = serde_json::to_string(&payload)?.to_string();

    context.request::<bool>(Method::Post, url, Some(body)).wait()
}
