//! File containing wizard store object of graphql schema
use std::cmp;
use std::str::FromStr;

use futures::Future;
use hyper::Method;
use juniper::FieldResult;
use juniper::ID as GraphqlID;

use stq_routes::model::Model;
use stq_routes::service::Service;

use super::*;
use graphql::context::Context;
use graphql::models::*;

graphql_object!(WizardStore: Context as "WizardStore" |&self| {
    description: "Store's wizard info."

    interfaces: [&Node]

    field id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Stores, Model::WizardStore, self.id).to_string().into()
    }

    field raw_id() -> &i32 as "Unique int id"{
        &self.id
    }

    field moderator_comment(&executor) -> FieldResult<Option<ModeratorStoreComments>> as "Fetches moderator comment by id." {
        if let Some(ref store_id) = self.store_id {
            let context = executor.context();

            let url = format!(
                "{}/{}/{}",
                &context.config.service_url(Service::Stores),
                Model::ModeratorStoreComment.to_url(),
                store_id
            );

            context.request::<ModeratorStoreComments>(Method::Get, url, None)
                .wait()
                .map(|u| Some(u))
        } else {
            Ok(None)
        }
    }

    field wizard_step_one(&executor) -> WizardStepOne as "Fetches wizard step one." {
        let context = executor.context();
        self.clone().into()
    }

    field wizard_step_two(&executor) -> WizardStepTwo as "Fetches wizard step two." {
        let context = executor.context();
        self.clone().into()
    }

    field wizard_step_three(&executor, first = None : Option<i32> as "First edges", 
        after = None : Option<GraphqlID> as "Offset from begining",
        skip_base_prod_id = None : Option<i32> as "Skip base prod id" ) 
            -> FieldResult<Option<Connection<BaseProduct, PageInfo>>> as "Fetches wizard step three." {
        let context = executor.context();

        let offset = after
            .and_then(|id|{
                i32::from_str(&id).map(|i| i + 1).ok()
            })
            .unwrap_or_default();

        let records_limit = context.config.gateway.records_limit;
        let count = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        if let Some(ref store_id) = self.store_id {
            let url =  format!(
                        "{}/{}/{}/products?offset={}&count={}",
                        &context.config.service_url(Service::Stores),
                        Model::Store.to_url(),
                        store_id,
                        offset,
                        count + 1
                    );

            context.request::<Vec<BaseProduct>>(Method::Get, url, None)
                .map (|base_products| {
                    let mut base_product_edges: Vec<Edge<BaseProduct>> =  vec![];
                    for i in 0..base_products.len() {
                        let edge = Edge::new(
                                juniper::ID::from( (i as i32 + offset).to_string()),
                                base_products[i].clone()
                            );
                        base_product_edges.push(edge);
                    }
                    let has_next_page = base_product_edges.len() as i32 == count + 1;
                    if has_next_page {
                        base_product_edges.pop();
                    };
                    let has_previous_page = true;
                    let start_cursor =  base_product_edges.iter().nth(0).map(|e| e.cursor.clone());
                    let end_cursor = base_product_edges.iter().last().map(|e| e.cursor.clone());
                    let page_info = PageInfo {
                        has_next_page,
                        has_previous_page,
                        start_cursor,
                        end_cursor};
                    Connection::new(base_product_edges, page_info)
                })
                .wait()
                .map(|u| Some(u))
        } else {
            Ok(None)
        }
    }

});
