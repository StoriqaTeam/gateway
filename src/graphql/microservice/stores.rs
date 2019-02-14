use futures::Future;
use hyper::Method;
use juniper::FieldResult;

use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_types::{BaseProductId, ProductId, StoreId, StoresRole, UserId};

use graphql::context::Context;
use graphql::microservice::requests::{GetBaseProductsRequest, GetProductsRequest};
use graphql::models::*;

pub trait StoresService {
    fn get_store_by_id(&self, store_id: StoreId) -> FieldResult<Option<Store>>;

    fn roles(&self, user_id: UserId) -> FieldResult<Vec<StoresRole>>;

    fn add_role_to_user(&self, input: NewStoresRoleInput) -> FieldResult<NewRole<StoresMicroserviceRole>>;

    fn remove_role_from_user(&self, input: RemoveStoresRoleInput) -> FieldResult<NewRole<StoresMicroserviceRole>>;

    fn get_store_by_user(&self, user_id: UserId) -> FieldResult<Option<Store>>;

    fn get_currency_exchange_info(&self) -> FieldResult<CurrencyExchangeInfo>;

    fn get_base_products(&self, input: GetBaseProductsRequest) -> FieldResult<Vec<BaseProduct>>;

    fn get_product(&self, product_id: ProductId) -> FieldResult<Option<Product>>;

    fn get_products_by_base_product(&self, base_product_id: BaseProductId) -> FieldResult<Vec<Product>>;

    fn get_products(&self, input: GetProductsRequest) -> FieldResult<Vec<Product>>;
}

pub struct StoresServiceImpl<'ctx> {
    context: &'ctx Context,
}

impl<'ctx> StoresServiceImpl<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        StoresServiceImpl { context }
    }

    fn base_url(&self) -> String {
        self.context.config.service_url(Service::Stores)
    }

    fn request_url(&self, request: &str) -> String {
        format!("{}/{}", self.base_url(), request)
    }
}

impl<'ctx> StoresService for StoresServiceImpl<'ctx> {
    fn get_store_by_id(&self, store_id: StoreId) -> FieldResult<Option<Store>> {
        let request_path = format!("{}/{}", Model::Store.to_url(), store_id);
        let url = self.request_url(&request_path);
        self.context.request(Method::Get, url, None).wait()
    }

    fn roles(&self, user_id: UserId) -> FieldResult<Vec<StoresRole>> {
        let url = format!("{}/roles/by-user-id/{}", self.context.config.stores_microservice.url, user_id);

        self.context.request::<Vec<StoresRole>>(Method::Get, url, None).wait()
    }

    fn add_role_to_user(&self, input: NewStoresRoleInput) -> FieldResult<NewRole<StoresMicroserviceRole>> {
        let request_path = format!("{}", Model::Role.to_url());
        let url = self.request_url(&request_path);
        let body: String = serde_json::to_string(&input)?;
        self.context.request(Method::Post, url, Some(body)).wait()
    }

    fn remove_role_from_user(&self, input: RemoveStoresRoleInput) -> FieldResult<NewRole<StoresMicroserviceRole>> {
        let request_path = format!("{}", Model::Role.to_url());
        let url = self.request_url(&request_path);
        let body: String = serde_json::to_string(&input)?;
        self.context.request(Method::Delete, url, Some(body)).wait()
    }

    fn get_store_by_user(&self, user_id: UserId) -> FieldResult<Option<Store>> {
        let request_path = format!("{}/by_user_id/{}", Model::Store.to_url(), user_id);
        let url = self.request_url(&request_path);
        self.context.request(Method::Get, url, None).wait()
    }

    fn get_currency_exchange_info(&self) -> FieldResult<CurrencyExchangeInfo> {
        let request_path = "currency_exchange";
        let url = self.request_url(request_path);
        self.context.request(Method::Get, url, None).wait()
    }

    fn get_base_products(&self, input: GetBaseProductsRequest) -> FieldResult<Vec<BaseProduct>> {
        let request_path = format!("{}/search_by_ids", Model::BaseProduct.to_url());
        let url = self.request_url(&request_path);
        let body: String = serde_json::to_string(&input)?;
        self.context.request(Method::Post, url, Some(body)).wait()
    }

    fn get_product(&self, product_id: ProductId) -> FieldResult<Option<Product>> {
        let request_path = format!("{}/{}", Model::Product.to_url(), product_id);
        let url = self.request_url(&request_path);
        self.context.request(Method::Get, url, None).wait()
    }

    fn get_products_by_base_product(&self, base_product_id: BaseProductId) -> FieldResult<Vec<Product>> {
        let request_path = format!("{}/by_base_product/{}", Model::Product.to_url(), base_product_id);
        let url = self.request_url(&request_path);
        self.context.request(Method::Get, url, None).wait()
    }

    fn get_products(&self, input: GetProductsRequest) -> FieldResult<Vec<Product>> {
        let request_path = format!("{}/search_by_ids", Model::Product.to_url());
        let url = self.request_url(&request_path);
        let body: String = serde_json::to_string(&input)?;
        self.context.request(Method::Post, url, Some(body)).wait()
    }
}
