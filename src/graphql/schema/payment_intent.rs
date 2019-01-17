//! File containing PaymentIntent object of graphql schema
use graphql::context::Context;
use graphql::microservice::response::*;
use juniper::ID as GraphqlID;

use stq_static_resources::Currency;

graphql_object!(PaymentIntent: Context as "PaymentIntent" |&self| {
    description: "PaymentIntent info."

    field id() -> GraphqlID as "Unique id" {
        self.id.to_string().into()
    }

    field invoice_id() -> String as "Invoice id" {
        self.invoice_id.to_string()
    }

    field amount() -> f64 as "Amount" {
        self.amount
    }

    field amount_received() -> f64 as "Amount received" {
        self.amount_received
    }

    field client_secret() -> &Option<String> as "Client secret" {
        &self.client_secret
    }

    field currency() -> &Currency as " Currency" {
        &self.currency
    }

    field last_payment_error_message() -> &Option<String> as "Last payment error message" {
        &self.last_payment_error_message
    }

    field receipt_email() -> &Option<String> as "Email address that the receipt for the resulting payment will be sent to." {
        &self.receipt_email
    }

    field charge_id() -> Option<String> as "Charge id" {
        self.charge_id.as_ref().map(|v| v.0.clone())
    }

    field status() -> &PaymentIntentStatus as "Status" {
        &self.status
    }
});
