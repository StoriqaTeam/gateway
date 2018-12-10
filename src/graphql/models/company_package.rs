use stq_types::{CompanyId, CompanyPackageId, PackageId};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompaniesPackages {
    pub id: CompanyPackageId,
    pub company_id: CompanyId,
    pub package_id: PackageId,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
#[graphql(description = "New Companies Packages input object")]
pub struct NewCompaniesPackagesInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "company_id")]
    pub company_id: i32,
    #[graphql(description = "package_id")]
    pub package_id: i32,
    #[graphql(description = "dimensional factor")]
    pub dimensional_factor: Option<i32>,
    #[graphql(description = "uses static rates")]
    pub uses_static_rates: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ShippingRateSource {
    NotAvailable,
    Static { dimensional_factor: Option<i32> },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewCompaniesPackagesPayload {
    pub company_id: i32,
    pub package_id: i32,
    pub shipping_rate_source: Option<ShippingRateSource>,
}

impl From<NewCompaniesPackagesInput> for NewCompaniesPackagesPayload {
    fn from(input: NewCompaniesPackagesInput) -> Self {
        let NewCompaniesPackagesInput {
            company_id,
            package_id,
            dimensional_factor,
            uses_static_rates,
            ..
        } = input;

        let shipping_rate_source = match uses_static_rates {
            None => None,
            Some(false) => Some(ShippingRateSource::NotAvailable),
            Some(true) => Some(ShippingRateSource::Static { dimensional_factor }),
        };

        NewCompaniesPackagesPayload {
            company_id,
            package_id,
            shipping_rate_source,
        }
    }
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
#[graphql(description = "Replace shipping rates input object")]
pub struct ReplaceShippingRatesInput {
    #[graphql(description = "Client mutation id")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Id of the target company package")]
    pub company_package_id: i32,
    #[graphql(description = "Base64 CSV with rates per zone")]
    pub rates_csv: String,
    #[graphql(description = "Base64 CSV with zones")]
    pub zones_csv: String,
}

impl From<ReplaceShippingRatesInput> for ReplaceShippingRatesPayload {
    fn from(input: ReplaceShippingRatesInput) -> Self {
        let ReplaceShippingRatesInput { rates_csv, zones_csv, .. } = input;

        ReplaceShippingRatesPayload {
            rates_csv_base64: rates_csv,
            zones_csv_base64: zones_csv,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReplaceShippingRatesPayload {
    pub rates_csv_base64: String,
    pub zones_csv_base64: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub struct ShippingRate {
    pub weight_g: i32,
    pub price: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ShippingRates {
    pub id: i32,
    pub company_package_id: i32,
    pub from_alpha3: String,
    pub to_alpha3: String,
    pub rates: Vec<ShippingRate>,
}
