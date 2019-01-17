use graphql::models::*;

#[derive(Serialize, Debug, Clone)]
pub struct NewCustomerWithSourceRequest {
    pub email: String,
    pub card_token: String,
}

impl From<CreateCustomerWithSourceInput> for NewCustomerWithSourceRequest {
    fn from(other: CreateCustomerWithSourceInput) -> Self {
        let CreateCustomerWithSourceInput { email, card_token, .. } = other;

        Self { email, card_token }
    }
}
