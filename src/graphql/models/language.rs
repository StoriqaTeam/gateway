#[derive(GraphQLObject, Serialize, Deserialize, Debug)]
pub struct Language {
    pub id: i32,
    pub name: String,
}