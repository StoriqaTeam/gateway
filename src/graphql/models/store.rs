use juniper::ID as GraphqlID;

#[derive(Deserialize, Debug, Clone)]
pub struct Store {
    pub id: i32,
    pub name: String,
    pub is_active: bool,
    pub currency_id: i32,
    pub short_description: String,
    pub long_description: Option<String>,
    pub slug: String,
    pub cover: Option<String>,
    pub logo: Option<String>,
    pub phone: String,
    pub email: String,
    pub address: String,
    pub facebook_url: Option<String>,
    pub twitter_url: Option<String>,
    pub instagram_url: Option<String>,
}



#[derive(GraphQLInputObject, Serialize, Deserialize, Debug, Clone)]
pub struct UpdateStore {
    #[graphql(description="New name of a store.")]
    pub name: Option<String>,
    #[graphql(description="Currency id.")]
    pub currency_id: Option<i32>,
    #[graphql(description="Short description")]
    pub short_description: Option<String>,
    #[graphql(description="Long description")]
    pub long_description: Option<String>,
    #[graphql(description="Slug")]
    pub slug: Option<String>,
    #[graphql(description="Cover")]
    pub cover: Option<String>,
    #[graphql(description="Logo")]
    pub logo: Option<String>,
    #[graphql(description="Phone number")]
    pub phone: Option<String>,
    #[graphql(description="E-mail")]
    pub email: Option<String>,
    #[graphql(description="Address")]
    pub address: Option<String>,
    #[graphql(description="Facebook url")]
    pub facebook_url: Option<String>,
    #[graphql(description="Twitter url")]
    pub twitter_url: Option<String>,
    #[graphql(description="Instagram url")]
    pub instagram_url: Option<String>,
}


#[derive(GraphQLInputObject, Debug, Clone)]
#[graphql(description="Update store input object")]
pub struct UpdateStoreInput {
    #[graphql(description="Client mutation id.")]
    pub client_mutation_id: String,
    #[graphql(description="Input field update store.")]
    pub input_fields: UpdateStoreWithIdInput
}

#[derive(GraphQLInputObject, Debug, Clone)]
#[graphql(description="Update store with id input object")]
pub struct UpdateStoreWithIdInput {
    #[graphql(description="Id of a store.")]
    pub id: GraphqlID,
    #[graphql(description="Input field update store.")]
    pub update_store: UpdateStore
}


#[derive(GraphQLInputObject, Serialize, Deserialize, Debug, Clone)]
#[graphql(description="New store")]
pub struct NewStore {
    pub name: String,
    pub user_id: i32,
    pub currency_id: i32,
    pub short_description: String,
    pub long_description: Option<String>,
    pub slug: String,
    pub cover: Option<String>,
    pub logo: Option<String>,
    pub phone: String,
    pub email: String,
    pub address: String,
    pub facebook_url: Option<String>,
    pub twitter_url: Option<String>,
    pub instagram_url: Option<String>,
}


#[derive(GraphQLInputObject, Serialize, Deserialize, Debug, Clone)]
#[graphql(description="Create store input object")]
pub struct CreateStoreInput {
    #[graphql(description="Client mutation id.")]
    pub client_mutation_id: String,
    #[graphql(description="Input field new store.")]
    pub input_fields: NewStore
}

#[derive(GraphQLInputObject, Debug, Clone)]
#[graphql(description="Delete store")]
pub struct DeleteStore {
    #[graphql(description="Email of a store.")]
    pub id: GraphqlID,
}

#[derive(GraphQLInputObject, Debug, Clone)]
#[graphql(description="Deactivate store input object")]
pub struct DeactivateStoreInput {
    #[graphql(description="Client mutation id.")]
    pub client_mutation_id: String,
    #[graphql(description="Input field Delete store.")]
    pub input_fields: DeleteStore
}