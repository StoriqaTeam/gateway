//! File containing wizard store object of graphql schema
use chrono::prelude::*;
use futures::Future;
use hyper::Method;
use juniper::ID as GraphqlID;
use juniper::{FieldError, FieldResult};

use stq_api::types::ApiFutureExt;
use stq_api::orders::OrderClient;
use stq_routes::service::Service;
use stq_static_resources::OrderState;
use stq_types::{OrderId, OrderIdentifier};

use errors::into_graphql;
use graphql::context::Context;
use graphql::models::*;

graphql_object!(Invoice: Context as "Invoice" |&self| {
    description: "Invoice info"

    field id() -> GraphqlID as "Base64 Unique id"{
        self.invoice_id.to_string().into()
    }

    field orders(&executor) -> FieldResult<Vec<GraphQLOrder>> as "Fetches Orders." {
        let context = executor.context();

        let url = format!(
            "{}/invoices/by-id/{}/order_ids",
            &context.config.service_url(Service::Billing),
            self.invoice_id.to_string()
        );

        context.request::<Vec<OrderId>>(Method::Get, url, None)
            .wait()
            .and_then (|ids| {
                ids.into_iter().map(|id| {
                    let rpc_client = context.get_rest_api_client(Service::Orders);
                    rpc_client.get_order(OrderIdentifier::Id(id))
                        .sync()
                        .map_err(into_graphql)
                        .and_then(|order|{
                            if let Some(order) = order {
                                Ok(GraphQLOrder(order))
                            } else {
                                Err(FieldError::new(
                                    "Could not find order id received from invoice in orders.",
                                    graphql_value!({ "code": 100, "details": { "Order id does not exist in orders microservice." }}),
                                ))
                            }
                        })
                }).collect()
            })
    }

    field amount() -> &f64 as "amount"{
        &self.amount.0
    }

    field currency_id() -> &i32 as "currency id"{
        &self.currency_id.0
    }

    field price_reserved_due_date_time() -> String as "price reserved due to date time"{
        let datetime: DateTime<Utc> = self.price_reserved.into();
        datetime.to_rfc3339()
    }

    field state() -> &OrderState as "order state"{
        &self.state
    }

    field wallet() -> &Option<String> as "wallet"{
        &self.wallet
    }

    field transactions() -> &[Transaction] as "Transactions"{
        &self.transactions
    }

    field amount_captured() -> &f64 as "amount captured"{
        &self.amount_captured.0
    }
});

graphql_object!(Transaction: Context as "Transaction" |&self| {
    description: "Transaction info"

    field id() -> &str as "id"{
        &self.id
    }

    field amount() -> &f64 as "amount captured"{
        &self.amount_captured.0
    }

});
