use juniper::FieldResult;

use graphql::context::Context;
use graphql::models::*;

pub fn run_create_customer_with_source_mutation(context: &Context, input: CreateCustomerWithSourceInput) -> FieldResult<Customer> {
    let billing = context.get_billing_microservice();

    billing.create_customer_with_source(input.into())
}

pub fn run_update_customer_mutation(context: &Context, input: UpdateCustomerInput) -> FieldResult<Customer> {
    let billing = context.get_billing_microservice();

    billing.update_customer(input.into())
}

pub fn run_delete_customer_mutation(context: &Context, input: DeleteCustomerInput) -> FieldResult<()> {
    let billing = context.get_billing_microservice();

    billing.delete_customer(input.into())
}
