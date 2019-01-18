//! File containing PaymentIntent object of graphql schema
use graphql::context::Context;
use graphql::models::*;
use juniper::ID as GraphqlID;

graphql_object!(Customer: Context as "Customer" |&self| {
    description: "Customer info."

    field id() -> GraphqlID as "Unique id" {
        self.id.to_string().into()
    }

    field user_id() -> &i32 as "Raw id user" {
        &self.user_id.0
    }

    field email() -> &Option<String> as "Email address" {
        &self.email
    }

    field cards() -> &[Card] as "Cards" {
        &self.cards
    }

});

graphql_object!(Card: Context as "Card" |&self| {
    description: "Card info."

    field id() -> GraphqlID as "Unique id" {
        self.id.to_string().into()
    }

    field brand() -> &CardBrand as "Brand" {
        &self.brand
    }

    field country() -> &str as "Country" {
        &self.country
    }

    field customer() -> &Option<String> as "Customer" {
        &self.customer
    }

    field exp_month() -> i32 as "The expiration month of the card." {
        self.exp_month as i32
    }

    field exp_year() -> i32 as "The expiration year of the card." {
        self.exp_year as i32
    }

    field last4() -> &str as "The last 4 digits of the card number." {
        &self.last4
    }

    field name() -> &Option<String> as "The name of the cardholder, printed on the card." {
        &self.name
    }

});
