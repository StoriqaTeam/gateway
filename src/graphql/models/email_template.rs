pub struct EmailTemplate;

#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
#[graphql(description = "Update email template input object")]
pub struct EmailTemplateInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Data of a email template.")]
    pub data: String,
}

impl EmailTemplateInput {
    pub fn is_none(&self) -> bool {
        Self { data: String::default() } == self.clone()
    }
}
