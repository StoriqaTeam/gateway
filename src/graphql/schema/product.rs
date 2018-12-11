//! File containing product object of graphql schema

use std::str::FromStr;

use futures::Future;
use hyper::Method;
use juniper;
use juniper::ID as GraphqlID;
use juniper::{FieldError, FieldResult};

use stq_api::warehouses::{Stock, Warehouse};
use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_static_resources::Currency;
use stq_types::{ProductId, ProductSellerPrice, Quantity, StockId};

use super::*;
use graphql::context::Context;
use graphql::models::*;
use graphql::schema::base_product as base_product_module;
use graphql::schema::stock as module_stock;
use graphql::schema::warehouse as module_warehouse;

graphql_object!(Product: Context as "Product" |&self| {
    description: "Product's info."

    interfaces: [&Node]

    field id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Stores, Model::Product, self.id.0).to_string().into()
    }

    field raw_id() -> &i32 as "Unique int id"{
        &self.id.0
    }

    field base_product_id() -> &i32 as "Base product raw id" {
        &self.base_product_id.0
    }

    field is_active() -> &bool as "If the product was disabled (deleted), isActive is false" {
        &self.is_active
    }

    field discount() -> &Option<f64> as "Discount" {
        &self.discount
    }

    field currency() -> Currency as "Currency" {
        self.currency
    }

    field photo_main() -> &Option<String> as "Photo main" {
        &self.photo_main
    }

    field additional_photos() -> &Option<Vec<String>> as "Additional photos of the product." {
        &self.additional_photos
    }

    field vendor_code() -> &String as "Vendor code" {
        &self.vendor_code
    }

    field cashback() -> &Option<f64> as "Cashback" {
        &self.cashback
    }

    field price() -> &f64 as "Price" {
        &self.price.0
    }

    field pre_order() -> &bool as "Pre-order" {
        &self.pre_order
    }

    field pre_order_days() -> &i32 as "Pre-order days" {
        &self.pre_order_days
    }

    field customer_price() -> &CustomerPrice as "Customer price" {
        &self.customer_price
    }

    field base_product(&executor,
        visibility: Option<Visibility> as "Specifies allowed visibility of the base_product"
    ) -> FieldResult<Option<BaseProduct>> as "Fetches base product by product." {
        let context = executor.context();
        let visibility = visibility.unwrap_or_default();

        base_product_module::try_get_base_product(context, self.base_product_id, visibility)
    }

    field attributes(&executor) -> FieldResult<Option<Vec<ProdAttrValue>>> as "Variants" {
       let context = executor.context();
        let url = format!("{}/{}/{}/attributes",
            context.config.service_url(Service::Stores),
            Model::Product.to_url(),
            self.id);

        context.request::<Vec<ProdAttrValue>>(Method::Get, url, None)
            .wait()
            .or_else(|_| Ok(vec![]))
            .map(Some)
    }

    field quantity(&executor) -> FieldResult<Option<i32>> as "Fetches product quantity from warehouses." {
        let context = executor.context();

        self.get_quantity(context)
    }

    field stocks(&executor,
        visibility: Option<Visibility> as "Specifies allowed visibility of the stocks") -> FieldResult<Vec<GraphQLStock>> as "Find product on warehouses." {

       let context = executor.context();
       self.get_stocks(context, visibility)
    }

});

graphql_object!(CustomerPrice: Context as "CustomerPrice" |&self| {
    description: "Customer price."

    field price() -> &f64 as "Price" {
        &self.price.0
    }

    field currency() -> &Currency as "Currency" {
        &self.currency
    }
});

graphql_object!(Connection<Product, PageInfo>: Context as "ProductsConnection" |&self| {
    description:"Products Connection"

    field edges() -> &[Edge<Product>] {
        &self.edges
    }

    field page_info() -> &PageInfo {
        &self.page_info
    }
});

graphql_object!(Edge<Product>: Context as "ProductsEdge" |&self| {
    description:"Products Edge"

    field cursor() -> &juniper::ID {
        &self.cursor
    }

    field node() -> &Product {
        &self.node
    }
});

pub fn try_get_product(context: &Context, product_id: ProductId) -> FieldResult<Option<Product>> {
    let url_product = format!(
        "{}/{}/{}",
        context.config.service_url(Service::Stores),
        Model::Product.to_url(),
        product_id
    );

    context.request::<Option<Product>>(Method::Get, url_product, None).wait()
}

pub fn get_product(context: &Context, product_id: ProductId) -> FieldResult<Product> {
    try_get_product(context, product_id).and_then(|value| {
        if let Some(value) = value {
            Ok(value)
        } else {
            Err(FieldError::new(
                "Could not find Product from product id.",
                graphql_value!({ "code": 100, "details": { format!("Product with such id: {} does not exist in stores microservice.", product_id) }}),
            ))
        }
    })
}

pub fn get_seller_price(context: &Context, product_id: ProductId) -> FieldResult<ProductSellerPrice> {
    let url = format!(
        "{}/{}/{}/seller_price",
        context.config.service_url(Service::Stores),
        Model::Product.to_url(),
        product_id
    );

    context
        .request::<Option<ProductSellerPrice>>(Method::Get, url, None)
        .wait()
        .and_then(|seller_price| {
            if let Some(seller_price) = seller_price {
                Ok(seller_price)
            } else {
                Err(FieldError::new(
                    "Could not find product seller price from product id.",
                    graphql_value!({ "code": 100, "details": { "Product with such id does not exist in stores microservice." }}),
                ))
            }
        })
}

pub fn run_update_product_mutation(context: &Context, input: UpdateProductWithAttributesInput) -> FieldResult<Product> {
    let identifier = ID::from_str(&*input.id)?;
    let product_id = ProductId(identifier.raw_id);

    let url = identifier.url(&context.config);

    if input.is_none() {
        return Err(FieldError::new(
            "Nothing to update",
            graphql_value!({ "code": 300, "details": { "All fields to update are none." }}),
        ));
    }

    if validate_update_product(context, product_id)? {
        let body: String = serde_json::to_string(&input)?.to_string();
        context.request::<Product>(Method::Put, url, Some(body)).wait()
    } else {
        let current_base_product = base_product_module::get_base_product_by_product(context, product_id)?;

        Err(FieldError::new(
            "Could not update product.",
            graphql_value!({ "code": 100, "details": { format!("Variant with id: {} cannot be changed when Product with id: {} in status: {}.", product_id, current_base_product.id, current_base_product.status) }}),
        ))
    }
}

pub fn validate_update_product(context: &Context, product_id: ProductId) -> FieldResult<bool> {
    let url = format!(
        "{}/{}/{}/validate_update",
        context.config.service_url(Service::Stores),
        Model::Product.to_url(),
        product_id
    );

    context.request::<bool>(Method::Get, url, None).wait()
}

impl Product {
    fn get_stocks(&self, context: &Context, visibility: Option<Visibility>) -> FieldResult<Vec<GraphQLStock>> {
        let visibility = visibility.unwrap_or(Visibility::Active);
        let store_id = base_product_module::get_base_product(context, self.base_product_id, visibility)?.store_id;

        module_warehouse::get_warehouses_for_store(context, store_id).and_then(|warehouses: Vec<Warehouse>| {
            warehouses
                .into_iter()
                .map(|warehouse| {
                    module_stock::try_get_stock_for_warehouse(context, warehouse.id, self.id)
                        .map(|stock| {
                            stock.unwrap_or(Stock {
                                id: StockId::new(),
                                product_id: self.id,
                                warehouse_id: warehouse.id,
                                quantity: Quantity::default(),
                            })
                        })
                        .map(GraphQLStock)
                })
                .collect::<FieldResult<Vec<GraphQLStock>>>()
        })
    }

    fn get_quantity(&self, context: &Context) -> FieldResult<Option<i32>> {
        module_stock::get_stocks_for_product(context, self.id)
            .map(|products| products.iter().map(|p| p.quantity.0).sum())
            .map(Some)
    }
}
