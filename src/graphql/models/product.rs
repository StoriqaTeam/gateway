
#[derive(Deserialize, Debug, Clone)]
pub struct Product {
    pub id: i32,
    pub store_id: i32,
    pub name: String,
    pub is_active: bool,
    pub short_description: String,
    pub long_description: Option<String>,
    pub price: f64,
    pub currency_id: i32,
    pub discount: Option<f64>,
    pub category: Option<i32>,
    pub photo_main: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewProduct {
    pub name: String,
    pub store_id: i32,
    pub currency_id: i32,
    pub short_description: String,
    pub long_description: Option<String>,
    pub price: f64,
    pub discount: Option<f64>,
    pub category: Option<i32>,
    pub photo_main: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateProduct {
    pub name: Option<String>,
    pub currency_id: Option<i32>,
    pub short_description: Option<String>,
    pub long_description: Option<String>,
    pub price: Option<f64>,
    pub discount: Option<f64>,
    pub category: Option<i32>,
    pub photo_main: Option<String>,
}