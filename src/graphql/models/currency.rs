#[derive(GraphQLObject, Serialize, Deserialize, Debug)]
pub struct Currency {
    #[graphql(name="key")]
    pub id: i32,
    pub name: String,
}