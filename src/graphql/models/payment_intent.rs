#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
#[graphql(description = "Update product with attributes input object")]
pub struct CreatePaymentIntentFeeInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Fee id.")]
    pub fee_id: i32,
}
