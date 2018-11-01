//! File containing node object of graphql schema
//! File containing store object of graphql schema
use juniper;
use juniper::FieldResult;
use juniper::ID as GraphqlID;

use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_static_resources::Translation;

use super::*;
use graphql::context::Context;
use graphql::models::*;
use graphql::schema::cart_product::calculate_product_price;
use graphql::schema::coupon::get_coupon;

graphql_object!(CartStore: Context as "CartStore" |&self| {
    description: "Cart store's info."

    interfaces: [&Node]

    field id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Stores, Model::CartStore, self.id.0).to_string().into()
    }

    field raw_id() -> &i32 as "Unique int id"{
        &self.id.0
    }

    field name() -> &[Translation] as "Full Name" {
        &self.name
    }

    field rating() -> &f64 as "Rating" {
        &self.rating
    }

    field logo() -> &Option<String> as "Logo" {
        &self.logo
    }

    field cover() -> &Option<String> as "Cover" {
        &self.cover
    }

    field products_cost(&executor) -> FieldResult<f64> as "Products cost" {
        let context = executor.context();

        calculate_products_price(context, &self.products)
    }

    field products_cost_without_discounts() -> f64 as "Products without cost" {
        calculate_products_price_without_discounts(&self.products)
    }

    field coupons(&executor) -> FieldResult<Vec<Coupon>> as "Coupons added user" {
        let context = executor.context();

        self.products.iter().try_fold(vec![], |mut acc, product| {
            if let Some(coupon_id) = product.coupon_id {
                let coupon = get_coupon(context, coupon_id)?;
                acc.push(coupon);
            }

            Ok(acc)
        })
    }

    field coupons_discount(&executor) -> FieldResult<f64> as "Coupons discount" {
        let context = executor.context();

        calculate_coupons_discount(context, &self.products)
    }

    field delivery_cost() -> f64 as "Delivery cost" {
        0.0
    }

    field total_cost(&executor) -> FieldResult<f64> as "Total cost" {
        let context = executor.context();

        calculate_products_price(context, &self.products)
    }

    field total_cost_without_discounts() -> f64 as "Total without cost" {
        calculate_products_price_without_discounts(&self.products)
    }

    field total_count() -> i32 as "Total products count" {
        self.products.iter().fold(0, |acc, x| {
            if x.selected {
                acc + x.quantity.0
            } else {
                acc
            }
        })
    }

    field products() -> &[CartProduct] as "Fetches products in the store cart." {
        &self.products
    }

});

graphql_object!(Connection<CartStore, PageInfo>: Context as "CartStoresConnection" |&self| {
    description:"Cart Store Connection"

    field edges() -> &[Edge<CartStore>] {
        &self.edges
    }

    field page_info() -> &PageInfo {
        &self.page_info
    }
});

graphql_object!(Edge<CartStore>: Context as "CartStoresEdge" |&self| {
    description:"Cart Store Edge"

    field cursor() -> &juniper::ID {
        &self.cursor
    }

    field node() -> &CartStore {
        &self.node
    }
});

pub fn calculate_products_price(context: &Context, products: &[CartProduct]) -> FieldResult<f64> {
    products.iter().try_fold(0.0, |acc, x| {
        if x.selected {
            Ok(acc + calculate_product_price(context, &x)?)
        } else {
            Ok(acc)
        }
    })
}

pub fn calculate_products_price_without_discounts(products: &[CartProduct]) -> f64 {
    products.iter().fold(0.0, |acc, x| {
        if x.selected {
            acc + x.price.0 * f64::from(x.quantity.0)
        } else {
            acc
        }
    })
}

pub fn calculate_coupons_discount(context: &Context, products: &[CartProduct]) -> FieldResult<f64> {
    let price_with_discounts = calculate_products_price(context, products)?;

    Ok(calculate_products_price_without_discounts(products) - price_with_discounts)
}
