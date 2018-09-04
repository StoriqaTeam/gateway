use stq_types::CountryLabel;

/// Payload for creating countries
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Country {
    pub label: CountryLabel,
    pub parent_label: Option<CountryLabel>,
    pub level: i32,
    pub alpha2: String,
    pub alpha3: String,
    pub numeric: i32,
    pub is_selected: bool,
    pub children: Vec<Country>,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Debug, Clone)]
#[graphql(description = "Country input object")]
pub struct CountryInput {
    #[graphql(description = "label")]
    pub label: String,
    #[graphql(description = "parent_label")]
    pub parent_label: Option<String>,
    #[graphql(description = "level")]
    pub level: i32,
    #[graphql(description = "alpha2")]
    pub alpha2: String,
    #[graphql(description = "alpha3")]
    pub alpha3: String,
    #[graphql(description = "numeric")]
    pub numeric: i32,
    #[graphql(description = "is_selected")]
    pub is_selected: bool,
    #[graphql(description = "children")]
    pub children: Vec<CountryInput>,
}
