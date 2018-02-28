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
    pub language_id: i32,
    pub slogan: Option<String>,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Update store input object")]
pub struct UpdateStoreInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Id of a store.")]
    #[serde(skip_serializing)]
    pub id: GraphqlID,
    #[graphql(description = "New name of a store.")]
    pub name: Option<String>,
    #[graphql(description = "Currency id.")]
    pub currency_id: Option<i32>,
    #[graphql(description = "Short description")]
    pub short_description: Option<String>,
    #[graphql(description = "Long description")]
    pub long_description: Option<String>,
    #[graphql(description = "Slug")]
    pub slug: Option<String>,
    #[graphql(description = "Cover")]
    pub cover: Option<String>,
    #[graphql(description = "Logo")]
    pub logo: Option<String>,
    #[graphql(description = "Phone number")]
    pub phone: Option<String>,
    #[graphql(description = "E-mail")]
    pub email: Option<String>,
    #[graphql(description = "Address")]
    pub address: Option<String>,
    #[graphql(description = "Facebook url")]
    pub facebook_url: Option<String>,
    #[graphql(description = "Twitter url")]
    pub twitter_url: Option<String>,
    #[graphql(description = "Instagram url")]
    pub instagram_url: Option<String>,
    #[graphql(description = "Language id")]
    pub language_id: Option<i32>,
    #[graphql(description = "Slogan")]
    pub slogan: Option<String>,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Debug, Clone)]
#[graphql(description = "Create store input object")]
pub struct CreateStoreInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "New name of a store.")]
    pub name: String,
    #[graphql(description = "User id.")]
    pub user_id: i32,
    #[graphql(description = "Currency id.")]
    pub currency_id: i32,
    #[graphql(description = "Short description")]
    pub short_description: String,
    #[graphql(description = "Long description")]
    pub long_description: Option<String>,
    #[graphql(description = "Slug")]
    pub slug: String,
    #[graphql(description = "Cover")]
    pub cover: Option<String>,
    #[graphql(description = "Logo")]
    pub logo: Option<String>,
    #[graphql(description = "Phone number")]
    pub phone: String,
    #[graphql(description = "E-mail")]
    pub email: String,
    #[graphql(description = "Address")]
    pub address: String,
    #[graphql(description = "Facebook url")]
    pub facebook_url: Option<String>,
    #[graphql(description = "Twitter url")]
    pub twitter_url: Option<String>,
    #[graphql(description = "Instagram url")]
    pub instagram_url: Option<String>,
    #[graphql(description = "Language id")]
    pub language_id: i32,
    #[graphql(description = "Slogan")]
    pub slogan: Option<String>,
}

#[derive(GraphQLInputObject, Debug, Clone)]
#[graphql(description = "Deactivate store input object")]
pub struct DeactivateStoreInput {
    #[graphql(description = "Client mutation id.")]
    pub client_mutation_id: String,
    #[graphql(description = "Id of a store.")]
    pub id: GraphqlID,
}
