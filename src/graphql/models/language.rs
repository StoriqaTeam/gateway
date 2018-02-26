#[derive(GraphQLObject, Serialize, Deserialize, Debug)]
pub struct Language {
    #[graphql(name="key")]
    pub id: i32,
    pub name: String,
}