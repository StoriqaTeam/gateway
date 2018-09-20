//! File containing product object of graphql schema
use graphql::context::Context;
use graphql::models::*;

graphql_object!(NewRole<UserMicroserviceRole>: Context as "UsersRoles" |&self| {
    description: "Users Roles info."

    field user_id() -> &i32 as "User id" {
        &self.user_id.0
    }

    field name() -> &UserMicroserviceRole as "User Roles" {
        &self.name
    }
});

graphql_object!(NewRole<StoresMicroserviceRole>: Context as "StoresRoles" |&self| {
    description: "Users Roles info."

    field user_id() -> &i32 as "User id" {
        &self.user_id.0
    }

    field name() -> &StoresMicroserviceRole as "User Roles" {
        &self.name
    }
});
