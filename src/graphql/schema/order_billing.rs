//! File containing order billing object of graphql schema
use juniper::ID as GraphqlID;

use stq_static_resources::Currency;

use super::*;
use graphql::context::Context;
use graphql::models::*;

graphql_object!(OrderBillingInfo: Context as "OrderBillingInfo" |&self| {
    description: "Billing info order information."

    interfaces: [&Node]

    field order() -> &OrderBilling {
        &self.order
    }

    field billing_type() -> BillingType {
        self.billing_type
    }

    field proxy_company_billing_info() -> &Option<ProxyCompanyBillingInfo> {
        &self.proxy_company_billing_info
    }

    field russia_billing_info() -> &Option<RussiaBillingInfo> {
        &self.russia_billing_info
    }

    field international_billing_info() -> &Option<InternationalBillingInfo> {
        &self.international_billing_info
    }
});

graphql_object!(Connection<OrderBillingInfo, PageInfoSegments>: Context as "OrderBillingConnectionPages" |&self| {
    description: "OrderBillingInfo Connection"

    field edges() -> &[Edge<OrderBillingInfo>] {
        &self.edges
    }

    field page_info() -> &PageInfoSegments {
        &self.page_info
    }
});

graphql_object!(Edge<OrderBillingInfo>: Context as "OrderBillingInfoEdge" |&self| {
    description:"OrderBillingInfo Edge"

    field cursor() -> &juniper::ID {
        &self.cursor
    }

    field node() -> &OrderBillingInfo {
        &self.node
    }
});

graphql_object!(Connection<OrderBilling, PageInfoSegments>: Context as "OrderBillingConnectionPages" |&self| {
    description: "OrderBilling Connection"

    field edges() -> &[Edge<OrderBilling>] {
        &self.edges
    }

    field page_info() -> &PageInfoSegments {
        &self.page_info
    }
});

graphql_object!(Edge<OrderBilling>: Context as "OrderBillingEdge" |&self| {
    description:"OrderBilling Edge"

    field cursor() -> &juniper::ID {
        &self.cursor
    }

    field node() -> &OrderBilling {
        &self.node
    }
});

graphql_object!(OrderBilling: Context as "OrderBilling" |&self| {
    field id() -> GraphqlID as "Base64 Unique id" {
        self.id.to_string().into()
    }

    field seller_currency() -> Currency {
        self.seller_currency
    }

    field total_amount() -> f64 {
        self.total_amount
    }

    field cashback_amount() -> f64 {
        self.cashback_amount
    }

    field invoice_id() -> GraphqlID as "Base64 invoice id" {
        self.invoice_id.to_string().into()
    }

    field store_id() -> i32 {
        self.store_id.0
    }

    field state() -> PaymentState {
        self.state
    }

});

graphql_object!(ProxyCompanyBillingInfo: Context as "ProxyCompanyBillingInfo" |&self| {
    field id() -> &i32 {
        &self.id.0
    }

    field country_alpha3() -> &str {
        &self.country_alpha3.0
    }

    field account() -> &str {
        &self.account
    }

    field currency() -> &str {
        self.currency.code()
    }

    field name() -> &str {
        &self.name
    }

    field bank() -> &str {
        &self.bank
    }

    field swift() -> &str {
        &self.swift.0
    }

    field bank_address() -> &str {
        &self.bank_address
    }

    field country() -> &str {
        &self.country
    }

    field city() -> &str {
        &self.city
    }

    field recipient_address() -> &str {
        &self.recipient_address
    }

});

graphql_object!(RussiaBillingInfo: Context as "RussiaBillingInfo" |&self| {
    field id() -> &i32 {
        &self.id.0
    }

    field store_id() -> &i32 {
        &self.store_id.0
    }

    field kpp() -> &String {
        &self.kpp
    }

    field bic() -> &String {
        &self.bic
    }

    field inn() -> &String {
        &self.inn
    }

    field full_name() -> &String {
        &self.full_name
    }
});

graphql_object!(InternationalBillingInfo: Context as "InternationalBillingInfo" |&self| {
    field id() -> &i32 {
        &self.id.0
    }

    field store_id() -> &i32 {
        &self.store_id.0
    }

    field swift_bic() -> &String {
        &self.swift_bic.0
    }

    field bank_name() -> &String {
        &self.bank_name
    }

    field full_name() -> &String {
        &self.full_name
    }

    field iban() -> &String {
        &self.iban
    }
});
