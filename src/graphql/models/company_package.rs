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
}
