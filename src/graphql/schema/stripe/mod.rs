use juniper::FieldResult;

use graphql::context::Context;
use graphql::models::*;

pub fn run_create_customer_with_source_mutation(context: &Context, input: CreateCustomerWithSourceInput) -> FieldResult<Mock> {
    let billing = context.get_billing_microservice();

    let _ = billing.create_customer_with_source(input.into())?;

    Ok(Mock)
}
