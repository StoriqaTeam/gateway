use juniper::FieldResult;

use graphql::context::Context;
use graphql::models::*;

pub fn run_create_customer_with_source_mutation(context: &Context, input: CreateCustomerWithSourceInput) -> FieldResult<Customer> {
    let billing = context.get_billing_microservice();

    billing
        .create_customer_with_source(input.into())
        .map(|response| Customer::from(response))
}
