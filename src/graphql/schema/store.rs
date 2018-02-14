//! File containing node object of graphql schema
//! File containing store object of graphql schema
use juniper;
use graphql::context::Context;
use graphql::models::*;
use juniper::ID as GraphqlID;

use super::*;

graphql_object!(Store: Context as "Store" |&self| {
    description: "Store's info."

    interfaces: [&Node]

    field id() -> GraphqlID as "Unique id"{
        ID::new(Service::Stores, Model::Store, self.id).to_string().into()
    }

    field raw_id() -> GraphqlID as "Unique id"{
        self.id.to_string().into()
    }

    field name() -> String as "Full Name" {
        self.name.clone()
    }

    field isActive() -> bool as "If the store was disabled (deleted), isActive is false" {
        self.is_active
    }

    field currency_id() -> i32 as "Currency Id" {
        self.currency_id.clone()
    }

    field short_description() -> String as "Short description" {
        self.short_description.clone()
    }

    field long_description() -> Option<String> as "Long description" {
        self.long_description.clone()
    }

    field slug() -> String as "Slug" {
        self.slug.clone()
    }

    field cover() -> Option<String> as "Cover" {
        self.cover.clone()
    }

    field logo() -> Option<String> as "Logo" {
        self.logo.clone()
    }

    field phone() -> String as "Phone" {
        self.phone.clone()
    }

    field email() -> String as "Email" {
        self.email.clone()
    }

    field address() -> String as "Address" {
        self.address.clone()
    }

    field facebook_url() -> Option<String> as "Facebook url" {
        self.facebook_url.clone()
    }

    field twitter_url() -> Option<String> as "Twitter url" {
        self.twitter_url.clone()
    }

    field instagram_url() -> Option<String> as "Instagram url" {
        self.instagram_url.clone()
    }

});

graphql_object!(Connection<Store>: Context as "StoresConnection" |&self| {
    description:"Stores Connection"

    field edges() -> Vec<Edge<Store>> {
        self.edges.to_vec()
    }

    field page_info() -> PageInfo {
        self.page_info.clone()
    }
});

graphql_object!(Edge<Store>: Context as "StoresEdge" |&self| {
    description:"Stores Edge"
    
    field cursor() -> juniper::ID {
        self.cursor.clone()
    }

    field node() -> Store {
        self.node.clone()
    }
});
