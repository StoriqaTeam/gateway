#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
#[graphql(description = "Stripe Customer input.")]
pub struct CreateCustomerWithSourceInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Customer’s email address.")]
    pub email: Option<String>,
    #[graphql(description = "Credit card token for use Stripe API.")]
    pub card_token: String,
}
