//! File containing subscription related objects of graphql schema
use juniper::ID as GraphqlID;

use juniper::FieldResult;
use stq_static_resources::Currency;

use super::*;
use graphql::context::Context;
use graphql::models::*;

graphql_object!(StoreSubscription: Context as "StoreSubscription" |&self| {
    description: "Store subscription information."

    field id() -> GraphqlID as "Base64 Unique id" {
        format!("{}|store_subscripotion",self.store_id).to_string().into()
    }

    field store_id() ->  i32 as "Store id" {
        self.store_id.0
    }

    field currency() ->  Currency as "Currency" {
        self.currency
    }

    field value() ->  f64 as "Amount per product per day" {
        self.value.to_string().parse::<f64>().unwrap_or(0.0)
    }

    field wallet_address() ->  Option<&String> as "Wallet address"{
        self.wallet_address.as_ref()
    }

    field trial_start_date() ->  Option<String> as "Trial start date" {
        self.trial_start_date.map(|date| date.format("%+").to_string())
    }
});

graphql_object!(SubscriptionPayment: Context as "SubscriptionPayment" |&self| {
    description: "Subscription payment information."

    field id() -> GraphqlID as "Base64 Unique id" {
        format!("{}|subscription_payment",self.id).to_string().into()
    }

    field store_id() -> i32 as "Store id" {
        self.store_id.0
    }

    field amount() -> f64 as "Total amount" {
        self.amount.to_string().parse::<f64>().unwrap_or(0.0)
    }

    field currency() -> Currency as "Currency" {
        self.currency
    }

    field charge_id() -> Option<&String> as "Charge id (in case of fiat payment)" {
        self.charge_id.as_ref()
    }

    field transaction_id() -> Option<&String> as "Transaction id (in case of crypto payment)" {
        self.transaction_id.as_ref()
    }

    field status() -> SubscriptionPaymentStatus {
        self.status
    }

    field created_at() -> String {
        self.created_at.format("%+").to_string()
    }

    field subscriptions(&executor) -> FieldResult<Vec<Subscription>> {
        executor.context().get_billing_microservice()
            .get_subscriptions(self.id)
    }
});

graphql_object!(Subscription: Context as "Subscription" |&self| {
    description: "Subscription information."

    field id() -> GraphqlID as "Base64 Unique id" {
        format!("{}|subscripotion",self.id).to_string().into()
    }

    field store_id() -> i32 as "Store id" {
        self.store_id.0
    }

    field published_base_products_quantity() -> i32 as "Published base products quantity" {
        self.published_base_products_quantity.0
    }

    field subscription_payment_id() -> Option<i32> as "Subscription payment id" {
        self.subscription_payment_id.map(|id| id.0)
    }

    field created_at() -> String {
        self.created_at.format("%+").to_string()
    }
});

graphql_object!(Connection<SubscriptionPayment, PageInfoSegments>: Context as "SubscriptionPaymentConnectionPages" |&self| {
    description: "SubscriptionPayment Connection"

    field edges() -> &[Edge<SubscriptionPayment>] {
        &self.edges
    }

    field page_info() -> &PageInfoSegments {
        &self.page_info
    }
});

graphql_object!(Edge<SubscriptionPayment>: Context as "SubscriptionPaymentEdge" |&self| {
    description:"SubscriptionPayment Edge"

    field cursor() -> &juniper::ID {
        &self.cursor
    }

    field node() -> &SubscriptionPayment {
        &self.node
    }
});

pub fn subscription_payments(
    context: &Context,
    current_page: i32,
    items_count: i32,
    search_params: SubscriptionPaymentSearch,
) -> FieldResult<Connection<SubscriptionPayment, PageInfoSegments>> {
    let current_page = std::cmp::max(current_page, 1);
    let records_limit = context.config.gateway.records_limit;
    let items_count = std::cmp::max(1, std::cmp::min(items_count, records_limit as i32));
    let skip = items_count * (current_page - 1);
    let result = context
        .get_billing_microservice()
        .subscription_payments(skip, items_count, search_params)?;
    let total_pages = std::cmp::max(0, result.total_count as i32 - 1) / items_count + 1;
    let subscription_payments_edges: Vec<Edge<SubscriptionPayment>> = result
        .subscription_payments
        .into_iter()
        .map(|subscription_payment| {
            Edge::new(
                GraphqlID::from(format!("{}|subscription_payment", subscription_payment.id)),
                subscription_payment,
            )
        })
        .collect();
    let page_info = PageInfoSegments {
        current_page,
        page_items_count: items_count,
        total_pages,
    };
    Ok(Connection::new(subscription_payments_edges, page_info))
}
