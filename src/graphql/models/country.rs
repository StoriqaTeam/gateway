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
