#[derive(Deserialize, Debug, Clone)]
pub struct Store {
    pub id: i32,
    pub name: String,
    pub is_active: bool,
    pub currency_id: i32,
    pub short_description: String,
    pub long_description: Option<String>,
    pub slug: String,
    pub cover: Option<String>,
    pub logo: Option<String>,
    pub phone: String,
    pub email: String,
    pub address: String,
    pub facebook_url: Option<String>,
    pub twitter_url: Option<String>,
    pub instagram_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewStore {
    pub name: String,
    pub user_id: i32,
    pub currency_id: i32,
    pub short_description: String,
    pub long_description: Option<String>,
    pub slug: String,
    pub cover: Option<String>,
    pub logo: Option<String>,
    pub phone: String,
    pub email: String,
    pub address: String,
    pub facebook_url: Option<String>,
    pub twitter_url: Option<String>,
    pub instagram_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateStore {
    pub name: Option<String>,
    pub currency_id: Option<i32>,
    pub short_description: Option<String>,
    pub long_description: Option<String>,
    pub slug: Option<String>,
    pub cover: Option<String>,
    pub logo: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub facebook_url: Option<String>,
    pub twitter_url: Option<String>,
    pub instagram_url: Option<String>,
}
