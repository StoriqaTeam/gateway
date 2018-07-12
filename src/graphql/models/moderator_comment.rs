use stq_types::{StoreId, UserId};

#[derive(Deserialize, Debug, Clone)]
pub struct ModeratorProductComments {
    pub id: i32,
    pub moderator_id: UserId,
    pub base_product_id: i32,
    pub comments: String,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
#[graphql(description = "Create product comments input object")]
pub struct CreateModeratorProductCommentsInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "moderator id.")]
    pub moderator_id: i32,
    #[graphql(description = "base product id.")]
    pub base_product_id: i32,
    #[graphql(description = "comments.")]
    pub comments: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ModeratorStoreComments {
    pub id: i32,
    pub moderator_id: UserId,
    pub store_id: StoreId,
    pub comments: String,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
#[graphql(description = "Create store comments input object")]
pub struct CreateModeratorStoreCommentsInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "moderator id.")]
    pub moderator_id: i32,
    #[graphql(description = "store id.")]
    pub store_id: i32,
    #[graphql(description = "comments.")]
    pub comments: String,
}
