use stq_types::*;

#[derive(GraphQLInputObject, Serialize, Deserialize, Clone, Debug)]
#[graphql(description = "New Shipping Input")]
pub struct NewShippingInput {
    #[graphql(description = "local shipping")]
    pub local: Vec<NewLocalShippingProductsInput>,
    #[graphql(description = "international shipping")]
    pub international: Vec<NewInternationalShippingProductsInput>,
    #[graphql(description = "pickups")]
    pub pickup: Option<NewPickupsInput>,
    #[graphql(description = "base product id")]
    pub base_product_id: i32,
    #[graphql(description = "store id")]
    pub store_id: i32,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Clone, Debug)]
#[graphql(description = "New Local Shipping Products Input")]
pub struct NewLocalShippingProductsInput {
    #[graphql(description = "company package id")]
    pub company_package_id: i32,
    #[graphql(description = "price")]
    pub price: Option<f64>,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Clone, Debug)]
#[graphql(description = "New International Shipping Products Input")]
pub struct NewInternationalShippingProductsInput {
    #[graphql(description = "company package id")]
    pub company_package_id: i32,
    #[graphql(description = "price")]
    pub price: Option<f64>,
    #[graphql(description = "deliveries to")]
    pub deliveries_to: Vec<String>,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Clone, Debug)]
#[graphql(description = "New Pickups Input")]
pub struct NewPickupsInput {
    #[graphql(description = "pickup")]
    pub pickup: bool,
    #[graphql(description = "price")]
    pub price: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewShipping {
    pub items: Vec<NewShippingProducts>,
    pub pickup: Option<NewPickups>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub enum ShippingVariant {
    Local,
    International,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewShippingProducts {
    pub base_product_id: BaseProductId,
    pub store_id: StoreId,
    pub company_package_id: CompanyPackageId,
    pub price: Option<ProductPrice>,
    pub deliveries_to: Vec<CountryLabel>,
    pub shipping: ShippingVariant,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewPickups {
    pub base_product_id: BaseProductId,
    pub store_id: StoreId,
    pub pickup: bool,
    pub price: Option<ProductPrice>,
}

impl From<(NewShippingInput, String)> for NewShipping {
    fn from(shippping: (NewShippingInput, String)) -> NewShipping {
        let country_label = shippping.1.clone();
        let shippping = shippping.0;
        let base_product_id = shippping.base_product_id.into();
        let store_id = shippping.store_id.into();
        let mut local_shippings = shippping
            .local
            .into_iter()
            .map(|local| NewShippingProducts {
                base_product_id,
                store_id,
                company_package_id: local.company_package_id.into(),
                price: local.price.map(|price| price.into()),
                deliveries_to: vec![country_label.into()],
                shipping: ShippingVariant::Local,
            })
            .collect();

        let mut international_shippings = shippping
            .international
            .into_iter()
            .map(|international| NewShippingProducts {
                base_product_id,
                store_id,
                company_package_id: international.company_package_id.into(),
                price: international.price.map(|price| price.into()),
                deliveries_to: international.deliveries_to.into_iter().map(|d| d.into()).collect(),
                shipping: ShippingVariant::International,
            })
            .collect();

        let items = vec![];
        items.append(&mut local_shippings);
        items.append(&mut international_shippings);

        let pickup = shippping.pickup.map(|pickups| NewPickups {
            base_product_id,
            store_id,
            pickup: pickups.pickup,
            price: pickups.price.map(|price| price.into()),
        });

        NewShipping { items, pickup }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Shipping {
    pub items: Vec<ShippingProducts>,
    pub pickup: Option<Pickups>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ShippingProducts {
    pub id: i32,
    pub base_product_id: BaseProductId,
    pub store_id: StoreId,
    pub company_package_id: CompanyPackageId,
    pub price: Option<ProductPrice>,
    pub deliveries_to: Vec<CountryLabel>,
    pub shipping: ShippingVariant,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Pickups {
    pub id: i32,
    pub base_product_id: BaseProductId,
    pub store_id: StoreId,
    pub pickup: bool,
    pub price: Option<ProductPrice>,
}

#[derive(GraphQLObject, Serialize, Deserialize, Clone, Debug)]
#[graphql(description = "Shipping Output")]
pub struct ShippingOutput {
    #[graphql(description = "local shipping")]
    pub local: Vec<LocalShippingProducts>,
    #[graphql(description = "international shipping")]
    pub international: Vec<InternationalShippingProducts>,
    #[graphql(description = "pickups")]
    pub pickup: Option<PickupsOutput>,
}

#[derive(GraphQLObject, Serialize, Deserialize, Clone, Debug)]
#[graphql(description = " Local Shipping Products Output")]
pub struct LocalShippingProducts {
    #[graphql(description = "company package id")]
    pub company_package_id: i32,
    #[graphql(description = "price")]
    pub price: Option<f64>,
}

#[derive(GraphQLObject, Serialize, Deserialize, Clone, Debug)]
#[graphql(description = " International Shipping Products Output")]
pub struct InternationalShippingProducts {
    #[graphql(description = "company package id")]
    pub company_package_id: i32,
    #[graphql(description = "price")]
    pub price: Option<f64>,
    #[graphql(description = "deliveries to")]
    pub deliveries_to: Vec<String>,
}

#[derive(GraphQLObject, Serialize, Deserialize, Clone, Debug)]
#[graphql(description = " Pickups Output")]
pub struct PickupsOutput {
    #[graphql(description = "pickup")]
    pub pickup: bool,
    #[graphql(description = "price")]
    pub price: Option<f64>,
}

impl From<Shipping> for ShippingOutput {
    fn from(shipping: Shipping) -> ShippingOutput {
        let local = vec![];
        let international = vec![];
        for item in shipping.items {
            match item.shipping {
                ShippingVariant::International => {
                    international.push(InternationalShippingProducts {
                        company_package_id: item.company_package_id.0,
                        price: item.price.map(|price| price.0),
                        deliveries_to: item.deliveries_to.into_iter().map(|d| d.0).collect(),
                    });
                }

                ShippingVariant::Local => {
                    local.push(LocalShippingProducts {
                        company_package_id: item.company_package_id.0,
                        price: item.price.map(|price| price.0),
                    });
                }
            }
        }

        let pickup = shipping.pickup.map(|pickups| PickupsOutput {
            pickup: pickups.pickup,
            price: pickups.price.map(|price| price.0),
        });

        ShippingOutput {
            local,
            international,
            pickup,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AvailablePackages {
    pub id: CompanyPackageId,
    pub name: String,
    pub deliveries_to: Vec<CountryLabel>,
    pub local_available: bool,
}
