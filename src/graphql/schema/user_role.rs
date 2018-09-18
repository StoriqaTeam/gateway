//! File containing product object of graphql schema
use graphql::context::Context;
use graphql::models::*;

graphql_object!(UsersRoles: Context as "UsersRoles" |&self| {
    description: "Users Roles info."

    field user_id() -> &i32 as "User id" {
        &self.user_id.0
    }

    field roles() -> &[UserMicroserviceRole] as "User Roles" {
        &self.roles
    }
});

graphql_object!(StoresRoles: Context as "StoresRoles" |&self| {
    description: "Stores Roles info."

    field user_id() -> &i32 as "User id" {
        &self.user_id.0
    }

    field roles() -> &[StoresMicroserviceRole] as "Stores Roles" {
        &self.roles
    }
});
