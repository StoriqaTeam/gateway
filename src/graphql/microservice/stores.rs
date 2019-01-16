use futures::Future;
use hyper::Method;
use juniper::FieldResult;

use stq_types::{StoresRole, UserId};

use graphql::context::Context;

pub trait StoresService {
    fn roles(&self, user_id: UserId) -> FieldResult<Vec<StoresRole>>;
}

pub struct StoresServiceImpl<'ctx> {
    context: &'ctx Context,
}

impl<'ctx> StoresServiceImpl<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        StoresServiceImpl { context }
    }
}

impl<'ctx> StoresService for StoresServiceImpl<'ctx> {
    fn roles(&self, user_id: UserId) -> FieldResult<Vec<StoresRole>> {
        let url = format!("{}/roles/by-user-id/{}", self.context.config.stores_microservice.url, user_id);

        self.context.request::<Vec<StoresRole>>(Method::Get, url, None).wait()
    }
}
