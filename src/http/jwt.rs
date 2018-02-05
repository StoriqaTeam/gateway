#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JWTPayload {
    pub user_id: i32,
}
