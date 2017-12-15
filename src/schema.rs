use juniper;
use juniper::{EmptyMutation, FieldResult};

use helper::users::User;
use helper::microservices::Microservices;


pub struct Query;

pub type Schema = juniper::RootNode<'static, Query, EmptyMutation<Microservices>>;

pub fn create() -> Schema {
    let query = Query {};
    let mutation = EmptyMutation::new();
    Schema::new(query, mutation)
}


graphql_object!(Query: Microservices |&self| {

    field apiVersion() -> &str {
        "1.0"
    }

    field user(&executor, id: i32) -> FieldResult<User> {
        let microservices = executor.context();
        let users = &microservices.users;
        let user = users.get_by_id(id);
        Ok(user)
    }
    
    field users(&executor, from: i32, to: i32) -> FieldResult<Vec<User>> {
        let microservices = executor.context();
        let users = &microservices.users;
        let users = users.get_users(from, to);
        Ok(users)
    }
});
