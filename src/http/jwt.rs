#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JWTPayload {
    user_email: String,
}
