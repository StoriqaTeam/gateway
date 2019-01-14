use std::time::SystemTime;

use juniper::ID as GraphqlID;
use uuid::Uuid;

use stq_static_resources::{Language, ModerationStatus, Translation, TranslationInput};
use stq_types::{Alpha3, StoreId, UserId};

use super::*;

#[derive(Deserialize, Debug, Clone)]
pub struct Store {
    pub id: StoreId,
    pub user_id: UserId,
    pub name: Vec<Translation>,
    pub is_active: bool,
    pub short_description: Vec<Translation>,
    pub long_description: Option<Vec<Translation>>,
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
    pub rating: f64,
    pub country: Option<String>,
    pub country_code: Option<Alpha3>,
    pub status: ModerationStatus,
    pub administrative_area_level_1: Option<String>,
    pub administrative_area_level_2: Option<String>,
    pub locality: Option<String>,
    pub political: Option<String>,
    pub postal_code: Option<String>,
    pub route: Option<String>,
    pub street_number: Option<String>,
    pub place_id: Option<String>,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
    pub base_products: Option<Vec<BaseProduct>>,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
#[graphql(description = "Update store input object")]
pub struct UpdateStoreInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Id of a store.")]
    #[serde(skip_serializing)]
    pub id: GraphqlID,
    #[graphql(description = "New name of a store.")]
    pub name: Option<Vec<TranslationInput>>,
    #[graphql(description = "Short description")]
    pub short_description: Option<Vec<TranslationInput>>,
    #[graphql(description = "Long description")]
    pub long_description: Option<Vec<TranslationInput>>,
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
    #[graphql(description = "Address")]
    #[serde(flatten)]
    pub address_full: AddressInput,
}

impl UpdateStoreInput {
    pub fn is_none(&self) -> bool {
        Self {
            client_mutation_id: self.client_mutation_id.clone(),
            id: self.id.clone(),
            name: None,
            short_description: None,
            long_description: None,
            slug: None,
            cover: None,
            logo: None,
            phone: None,
            email: None,
            facebook_url: None,
            twitter_url: None,
            instagram_url: None,
            default_language: None,
            slogan: None,
            address_full: AddressInput {
                country: None,
                country_code: None,
                administrative_area_level_1: None,
                administrative_area_level_2: None,
                locality: None,
                political: None,
                postal_code: None,
                route: None,
                street_number: None,
                value: None,
                place_id: None,
            },
        } == self.clone()
    }
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Create store input object")]
pub struct CreateStoreInput {
    #[graphql(name = "clientMutationId", description = "Client mutation id.")]
    pub uuid: String,
    #[graphql(description = "New name of a store.")]
    pub name: Vec<TranslationInput>,
    #[graphql(description = "User id.")]
    pub user_id: i32,
    #[graphql(description = "Short description")]
    pub short_description: Vec<TranslationInput>,
    #[graphql(description = "Long description")]
    pub long_description: Option<Vec<TranslationInput>>,
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
    #[graphql(description = "Address")]
    #[serde(flatten)]
    pub address_full: AddressInput,
}

impl CreateStoreInput {
    pub fn fill_uuid(mut self) -> Self {
        self.uuid = Some(self.uuid)
            .filter(|id| !id.is_empty())
            .unwrap_or_else(|| Uuid::new_v4().hyphenated().to_string());
        self
    }
}

#[derive(GraphQLInputObject, Debug, Clone)]
#[graphql(description = "Deactivate store input object")]
pub struct DeactivateStoreInput {
    #[graphql(description = "Client mutation id.")]
    pub client_mutation_id: String,
    #[graphql(description = "Id of a store.")]
    pub id: GraphqlID,
}

#[derive(GraphQLInputObject, Serialize, Clone, Debug)]
pub struct StoresSearchOptionsInput {
    #[graphql(description = "Category id.")]
    pub category_id: Option<i32>,
    #[graphql(description = "Country.")]
    pub country: Option<String>,
}

#[derive(GraphQLInputObject, Serialize, Clone, Debug)]
#[graphql(description = "Search store input object")]
pub struct SearchStoreInput {
    #[graphql(description = "Name part of the store.")]
    pub name: String,
    #[serde(skip_serializing)]
    #[graphql(description = "Get stores total count")]
    pub get_stores_total_count: bool,
    #[graphql(description = "Searching options")]
    pub options: Option<StoresSearchOptionsInput>,
}

#[derive(GraphQLInputObject, Serialize, Clone, Debug)]
#[graphql(description = "Search store by moderator input object")]
pub struct SearchModeratorStoreInput {
    #[graphql(description = "Name part of the store.")]
    pub name: Option<String>,
    #[graphql(description = "Email of store manager of the store.")]
    pub store_manager_email: Option<String>,
    #[graphql(description = "Moderation state of the store.")]
    pub state: Option<ModerationStatus>,
}

#[derive(Serialize, Clone, Debug)]
pub struct SearchModeratorStore {
    pub name: Option<String>,
    pub store_manager_ids: Option<Vec<UserId>>,
    pub state: Option<ModerationStatus>,
}

impl SearchModeratorStore {
    pub fn new(search_term: SearchModeratorStoreInput, store_manager_ids: Option<Vec<UserId>>) -> Self {
        Self {
            name: search_term.name,
            store_manager_ids,
            state: search_term.state,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct StoreSearchResults {
    pub total_count: u32,
    pub stores: Vec<Store>,
}

#[derive(Serialize, Clone, Debug)]
pub struct StoresSearchFilters {
    pub search_term: SearchStoreInput,
}

impl StoresSearchFilters {
    pub fn new(search_term: SearchStoreInput) -> Self {
        Self { search_term }
    }
}

#[derive(GraphQLInputObject, Clone, Debug)]
#[graphql(description = "Management store moderation status input object")]
pub struct StoreModerateInput {
    #[graphql(description = "Id of a store.")]
    pub id: GraphqlID,
    #[graphql(description = "Moderation state of the store.")]
    pub status: ModerationStatus,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoreModerate {
    pub store_id: StoreId,
    pub status: ModerationStatus,
}
