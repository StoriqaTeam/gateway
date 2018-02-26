#[derive(GraphQLObject, Serialize, Deserialize, Debug)]
pub struct Currency {
    pub id: i32,
    pub name: String,
}