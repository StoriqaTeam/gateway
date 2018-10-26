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
use graphql::schema::cart::calculate_product_price;

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

        self.products.iter().try_fold(0.0, |acc, x| {
            if x.selected {
                Ok(acc + calculate_product_price(context, &x)?)
            } else {
                Ok(acc)
            }
        })
    }

    field delivery_cost() -> f64 as "Delivery cost" {
        0.0
    }

    field total_cost(&executor) -> FieldResult<f64> as "Total cost" {
        let context = executor.context();

        self.products.iter().try_fold(0.0, |acc, x| {
            if x.selected {
                Ok(acc + calculate_product_price(context, &x)?)
            } else {
                Ok(acc)
            }
        })
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
