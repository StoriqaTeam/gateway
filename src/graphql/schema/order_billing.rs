//! File containing order billing object of graphql schema
use juniper::ID as GraphqlID;

use juniper::FieldResult;
use stq_static_resources::Currency;

use super::*;
use graphql::context::Context;
use graphql::models::*;
use stq_routes::model::Model;
use stq_routes::service::Service;

graphql_object!(OrderBillingInfo: Context as "OrderBillingInfo" |&self| {
    description: "Billing info order information."

    field order(&executor) -> FieldResult<Option<GraphQLOrder>> as "order" {
       executor.context()
        .get_orders_microservice()
        .get_order_by_id(self.order.id)
    }

    field id() -> GraphqlID as "Base64 Unique id" {
        format!("{}|billing_info",self.order.id).to_string().into()
    }

    field seller_currency() -> Currency {
        self.order.seller_currency
    }

    field total_amount() -> f64 {
        self.order.total_amount
    }

    field cashback_amount() -> f64 {
        self.order.cashback_amount
    }

    field invoice_id() -> GraphqlID as "Base64 invoice id" {
        self.order.invoice_id.to_string().into()
    }

    field store_id() -> i32 as "Store id" {
        self.order.store_id.0
    }

    field stripe_fee() -> &Option<f64> as "Stripe fee" {
        &self.order.stripe_fee
    }

    field store(&executor) -> FieldResult<Option<Store>> as "Store" {
         executor
        .context()
        .get_stores_microservice()
        .get_store_by_id(self.order.store_id)
    }

    field state() -> PaymentState {
        self.order.state
    }

    field fee(&executor) -> FieldResult<Option<Fee>> as "Fee" {
        executor
        .context()
        .get_billing_microservice()
        .get_fee_by_order_id(self.order.id)
    }

    field billing_type() -> BillingType as "billing type" {
        self.billing_type
    }

    field proxy_company_billing_info() -> &Option<ProxyCompanyBillingInfo> as "proxy company billing" {
        &self.proxy_company_billing_info
    }

    field russia_billing_info() -> &Option<RussiaBillingInfo> as "russia billing information" {
        &self.russia_billing_info
    }

    field international_billing_info() -> &Option<InternationalBillingInfo> as "international billing information" {
        &self.international_billing_info
    }
});

graphql_object!(Connection<OrderBillingInfo, PageInfoSegments>: Context as "OrderBillingInfoConnectionPages" |&self| {
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
        format!("{}|billing",self.id).to_string().into()
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

    field store_id() -> i32 as "Store id" {
        self.store_id.0
    }

    field store(&executor) -> FieldResult<Option<Store>> as "Store" {
         executor
        .context()
        .get_stores_microservice()
        .get_store_by_id(self.store_id)
    }

    field state() -> PaymentState {
        self.state
    }

    field fee(&executor) -> FieldResult<Option<Fee>> as "Fee" {
        executor
        .context()
        .get_billing_microservice()
        .get_fee_by_order_id(self.id)
    }

    field stripe_fee() -> &Option<f64> as "Stripe fee" {
        &self.stripe_fee
    }

});

graphql_object!(ProxyCompanyBillingInfo: Context as "ProxyCompanyBillingInfo" |&self| {
    field id() -> GraphqlID as "GraphqlID" {
        ID::new(Service::Billing, Model::ProxyCompanyBillingInfo, self.id.0).to_string().into()
    }

    field raw_id() -> &i32 {
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
    field id() -> GraphqlID as "GraphqlID" {
        ID::new(Service::Billing, Model::RussiaBillingInfo, self.id.0).to_string().into()
    }

    field raw_id() -> &i32 {
        &self.id.0
    }

    field store_id() -> &i32 {
        &self.store_id.0
    }

    field bank_name() -> &str {
        &self.bank_name
    }
    field branch_name() -> &Option<String> {
        &self.branch_name
    }
    field swift_bic() -> &str {
        self.swift_bic.0.as_ref()
    }
    field tax_id() -> &str {
        &self.tax_id
    }
    field correspondent_account() -> &str {
        &self.correspondent_account
    }
    field current_account() -> &str {
        &self.current_account
    }
    field personal_account() -> &Option<String> {
        &self.personal_account
    }
    field beneficiary_full_name() -> &str {
        &self.beneficiary_full_name
    }
});

graphql_object!(InternationalBillingInfo: Context as "InternationalBillingInfo" |&self| {
    field id() -> GraphqlID as "GraphqlID" {
        ID::new(Service::Billing, Model::InternationalBillingInfo, self.id.0).to_string().into()
    }

    field raw_id() -> &i32 {
        &self.id.0
    }

    field store_id() -> &i32 {
        &self.store_id.0
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
