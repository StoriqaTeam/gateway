//! File containing financial manager object of graphql schema
use juniper::FieldResult;
use juniper::ID as GraphqlID;

use graphql::context::Context;
use graphql::models::*;

graphql_object!(FinancialManager: Context as "FinancialManager" |&self| {
    description: "Financial manager's profile."

    field orders
    (
        &executor,
        current_page : i32 as "Current page",
        items_count : i32 as "Items count",
        search_params: OrderBillingSearchInput as "Search parameters"
    )
    -> FieldResult<Connection<OrderBilling, PageInfoSegments>> as "find orders for financier manager." 
    {
        let context = executor.context();

        let current_page = std::cmp::max(current_page, 1);

        let records_limit = context.config.gateway.records_limit;

        let items_count = std::cmp::max(1, std::cmp::min(items_count, records_limit as i32));

        let skip = items_count * (current_page - 1);

        let orders = context.get_billing_microservice().orders(skip, items_count, search_params)?;

        let total_pages = std::cmp::max(0, orders.total_count as i32 - 1) / items_count + 1;

        let mut orders_edges: Vec<Edge<OrderBilling>> = orders.orders
            .into_iter()
            .map(|order| Edge::new(
                GraphqlID::from(order.order.id.0.to_string()), order
            )).collect();
        let page_info = PageInfoSegments {
            current_page,
            page_items_count: items_count,
            total_pages,
        };
        Ok(Connection::new(orders_edges, page_info))
    }
});
