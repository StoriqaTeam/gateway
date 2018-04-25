//! File containing Category object of graphql schema
use futures::Future;
use hyper::Method;
use juniper::FieldResult;
use juniper::ID as GraphqlID;
use base64::encode;

use stq_routes::model::Model;
use stq_routes::service::Service;

use super::*;
use graphql::context::Context;
use graphql::models::*;

graphql_object!(CartProduct: Context as "CartProduct" |&self| {
    description: "Cart Product info."

    interfaces: [&Node]

    field id() -> GraphqlID as "Base64 Unique id"{
        encode(&*format!("{}|CartProduct|{}", Service::Orders, self.product_id)).to_string().into()
    }

    field quantity() -> &i32 as "Quantity" {
        &self.quantity
    }

    field product(&executor) -> FieldResult<Option<Product>> as "Fetches product from cart." {
        let context = executor.context();
        let url = format!("{}/{}/{}",
            context.config.service_url(Service::Stores),
            Model::Product.to_url(),
            self.product_id);

        context.request::<Product>(Method::Get, url, None)
            .wait()
            .map(|u| Some(u))
    }
});
