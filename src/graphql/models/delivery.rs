use stq_types::*;

use graphql::models::*;

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
    pub items: Vec<NewProducts>,
    pub pickup: Option<NewPickups>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub enum ShippingVariant {
    Local,
    International,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewProducts {
    pub base_product_id: BaseProductId,
    pub store_id: StoreId,
    pub company_package_id: CompanyPackageId,
    pub price: Option<ProductPrice>,
    pub deliveries_to: Vec<Alpha3>,
    pub shipping: ShippingVariant,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewPickups {
    pub base_product_id: BaseProductId,
    pub store_id: StoreId,
    pub pickup: bool,
    pub price: Option<ProductPrice>,
}

impl From<(NewShippingInput, Alpha3)> for NewShipping {
    fn from(shipping: (NewShippingInput, Alpha3)) -> NewShipping {
        let local_deliveries_to = shipping.1;
        let shipping = shipping.0;
        let base_product_id = shipping.base_product_id.into();
        let store_id = shipping.store_id.into();
        let mut local_shippings = shipping
            .local
            .into_iter()
            .map(|local| NewProducts {
                base_product_id,
                store_id,
                company_package_id: local.company_package_id.into(),
                price: local.price.map(|price| price.into()),
                deliveries_to: vec![local_deliveries_to.clone()],
                shipping: ShippingVariant::Local,
            }).collect();

        let mut international_shippings = shipping
            .international
            .into_iter()
            .map(|international| NewProducts {
                base_product_id,
                store_id,
                company_package_id: international.company_package_id.into(),
                price: international.price.map(|price| price.into()),
                deliveries_to: international.deliveries_to.into_iter().map(|v| Alpha3(v)).collect(),
                shipping: ShippingVariant::International,
            }).collect();

        let mut items = vec![];
        items.append(&mut local_shippings);
        items.append(&mut international_shippings);

        let pickup = shipping.pickup.map(|pickups| NewPickups {
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
    pub product: Products,
    pub deliveries_to: Vec<Country>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Products {
    pub id: i32,
    pub base_product_id: BaseProductId,
    pub store_id: StoreId,
    pub company_package_id: CompanyPackageId,
    pub price: Option<ProductPrice>,
    pub deliveries_to: Vec<Alpha3>,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ShippingOutput {
    pub local: Vec<LocalShippingProducts>,
    pub international: Vec<InternationalShippingProducts>,
    pub pickup: Option<PickupsOutput>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LocalShippingProducts {
    pub company_package_id: CompanyPackageId,
    pub price: Option<f64>,
    pub deliveries_to: Vec<Country>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InternationalShippingProducts {
    pub company_package_id: CompanyPackageId,
    pub price: Option<f64>,
    pub deliveries_to: Vec<Country>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PickupsOutput {
    pub pickup: bool,
    pub price: Option<f64>,
}

impl From<Shipping> for ShippingOutput {
    fn from(shipping: Shipping) -> ShippingOutput {
        let mut local = vec![];
        let mut international = vec![];
        for item in shipping.items {
            match item.product.shipping {
                ShippingVariant::International => {
                    international.push(InternationalShippingProducts {
                        company_package_id: item.product.company_package_id,
                        price: item.product.price.map(|price| price.0),
                        deliveries_to: item.deliveries_to,
                    });
                }

                ShippingVariant::Local => {
                    local.push(LocalShippingProducts {
                        company_package_id: item.product.company_package_id,
                        price: item.product.price.map(|price| price.0),
                        deliveries_to: item.deliveries_to,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AvailablePackages {
    pub id: CompanyPackageId,
    pub name: String,
    pub logo: String,
    pub deliveries_to: Vec<Country>,
    pub local_available: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AvailablePackagesOutput {
    pub local: Vec<AvailablePackages>,
    pub international: Vec<AvailablePackages>,
}

impl From<Vec<AvailablePackages>> for AvailablePackagesOutput {
    fn from(packages: Vec<AvailablePackages>) -> Self {
        let mut local = vec![];
        let mut international = vec![];
        for item in packages {
            if item.local_available {
                international.push(item.clone());
                local.push(item.clone());
            } else {
                international.push(item.clone());
            }
        }

        AvailablePackagesOutput { local, international }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AvailablePackageForUser {
    pub id: CompanyPackageId,
    pub name: String,
    pub logo: String,
    pub price: Option<ProductPrice>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AvailableShippingForUser {
    pub packages: Vec<AvailablePackageForUser>,
    pub pickups: Option<PickupsOutput>,
}
