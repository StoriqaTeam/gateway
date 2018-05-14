use stq_static_resources::Language;

#[derive(Deserialize, Debug, Clone)]
pub struct WizardStore {
    pub id: i32,
    pub user_id: i32,
    pub store_id: Option<i32>,
    pub name: Option<String>,
    pub short_description: Option<String>,
    pub default_language: Option<Language>,
    pub slug: Option<String>,
    pub country: Option<String>,
    pub address: Option<String>,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
#[graphql(description = "Update store wizard input object")]
pub struct UpdateWizardStoreInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Real store id.")]
    pub store_id: Option<i32>,
    #[graphql(description = "New name of a store.")]
    pub name: Option<String>,
    #[graphql(description = "Short description")]
    pub short_description: Option<String>,
    #[graphql(description = "Language")]
    pub default_language: Option<Language>,
    #[graphql(description = "Slug")]
    pub slug: Option<String>,
    #[graphql(description = "Country")]
    pub country: Option<String>,
    #[graphql(description = "Address")]
    pub address: Option<String>,
}

impl UpdateWizardStoreInput {
    pub fn is_none(&self) -> bool {
        Self {
            client_mutation_id: self.client_mutation_id.clone(),
            name: None,
            store_id: None,
            short_description: None,
            slug: None,
            address: None,
            default_language: None,
            country: None,
        } == self.clone()
    }
}

#[derive(GraphQLObject, Deserialize, Debug, Clone)]
#[graphql(description = "Wizard Step One")]
pub struct WizardStepOne {
    #[graphql(description = "New name of a store.")]
    pub name: Option<String>,
    #[graphql(description = "Short description")]
    pub short_description: Option<String>,
    #[graphql(description = "Slug")]
    pub slug: Option<String>,
}

impl From<WizardStore> for WizardStepOne {
    fn from(w: WizardStore) -> Self {
        Self {
            name: w.name,
            short_description: w.short_description,
            slug: w.slug,
        }
    }
}

#[derive(GraphQLObject, Deserialize, Debug, Clone)]
#[graphql(description = "Wizard Step Two")]
pub struct WizardStepTwo {
    #[graphql(description = "Language")]
    pub default_language: Option<Language>,
    #[graphql(description = "Country")]
    pub country: Option<String>,
    #[graphql(description = "Address")]
    pub address: Option<String>,
}

impl From<WizardStore> for WizardStepTwo {
    fn from(w: WizardStore) -> Self {
        Self {
            default_language: w.default_language,
            country: w.country,
            address: w.address,
        }
    }
}
