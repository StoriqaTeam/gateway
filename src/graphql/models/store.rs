use juniper::ID as GraphqlID;

use super::{Language, TranslatedText, TranslatedTextInput};

#[derive(Deserialize, Debug, Clone)]
pub struct Store {
    pub id: i32,
    pub name: Vec<TranslatedText>,
    pub is_active: bool,
    pub currency_id: i32,
    pub short_description: Vec<TranslatedText>,
    pub long_description: Option<Vec<TranslatedText>>,
    pub slug: String,
    pub cover: Option<String>,
    pub logo: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub facebook_url: Option<String>,
    pub twitter_url: Option<String>,
    pub instagram_url: Option<String>,
    pub default_language: Language,
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
    pub name: Option<Vec<TranslatedTextInput>>,
    #[graphql(description = "Currency id.")]
    pub currency_id: Option<i32>,
    #[graphql(description = "Short description")]
    pub short_description: Option<Vec<TranslatedTextInput>>,
    #[graphql(description = "Long description")]
    pub long_description: Option<Vec<TranslatedTextInput>>,
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
    #[graphql(description = "Language")]
    pub default_language: Option<Language>,
    #[graphql(description = "Slogan")]
    pub slogan: Option<String>,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Create store input object")]
pub struct CreateStoreInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "New name of a store.")]
    pub name: Vec<TranslatedTextInput>,
    #[graphql(description = "User id.")]
    pub user_id: i32,
    #[graphql(description = "Currency id.")]
    pub currency_id: i32,
    #[graphql(description = "Short description")]
    pub short_description: Vec<TranslatedTextInput>,
    #[graphql(description = "Long description")]
    pub long_description: Option<Vec<TranslatedTextInput>>,
    #[graphql(description = "Slug")]
    pub slug: String,
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
    #[graphql(description = "Default Language")]
    pub default_language: Language,
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
