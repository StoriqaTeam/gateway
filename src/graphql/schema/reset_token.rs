use graphql::context::Context;

use graphql::models::ResetToken;

graphql_object!(ResetToken: Context as "ResetToken" |&self| {
    description: "ResetToken info"

    field token() -> &str as "token" {
        &self.token
    }
    field email() -> &str as "email" {
        &self.email
    }
});
