use stq_types::{Alpha2, Alpha3, CountryLabel};

/// Payload for creating countries
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Country {
    pub label: CountryLabel,
    pub parent: Option<Alpha3>,
    pub level: i32,
    pub alpha2: Alpha2,
    pub alpha3: Alpha3,
    pub numeric: i32,
    pub is_selected: bool,
    pub children: Vec<Country>,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Debug, Clone)]
#[graphql(description = "Country input object")]
pub struct CountryInput {
    #[graphql(description = "label")]
    pub label: String,
    #[graphql(description = "parent Alpha3 code")]
    pub parent: Option<String>,
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

impl Country {
    pub fn childless_entries(&self) -> Vec<&Country> {
        let mut childless_entries = Vec::new();
        add_childless_entries(self, &mut childless_entries);
        childless_entries
    }
}

fn add_childless_entries<'a, 'b>(country: &'a Country, accumulator: &'b mut Vec<&'a Country>) {
    if country.children.is_empty() {
        accumulator.push(country);
    } else {
        for child in &country.children {
            add_childless_entries(child, accumulator);
        }
    }
}
