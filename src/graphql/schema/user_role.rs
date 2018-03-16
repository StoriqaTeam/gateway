//! File containing product object of graphql schema
use graphql::context::Context;
use graphql::models::*;

graphql_object!(UserRoles: Context as "UserRoles" |&self| {
    description: "User Roles info."

    field user_id() -> i32 as "User id" {
        self.user_id
    }

    field roles() -> Vec<Role> as "User Roles" {
        self.roles.clone()
    }
});
