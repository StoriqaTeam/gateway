#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
pub enum Role {
    Superuser,
    User,
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
pub struct NewUserRole {
    pub user_id: i32,
    pub role: Role,
}

#[derive(Deserialize, Debug)]
pub struct UserRole {
    pub id: i32,
    pub user_id: i32,
    pub role: Role,
}
