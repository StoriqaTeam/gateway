use juniper;
use juniper::FieldResult;

use helper::users::User;
use helper::microservices::Microservices;


pub struct Query;
pub struct Mutation;

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn create() -> Schema {
    let query = Query {};
    let mutation = Mutation {};
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


graphql_object!(Mutation: Microservices |&self| {

    //POST /users - создать пользователя. + Механизм для подтверждения email, если //не через соцсети
    field createUser(&executor, name: String, email: String) -> FieldResult<User> {
        let microservices = executor.context();
        let users = &microservices.users;
        let user = users.create_user(name, email);
        Ok(user)
    }

    //PUT /users/:id - апдейт пользователя
    field updateUser(&executor,id: i32, name: String, email: String) -> FieldResult<User> {
        let microservices = executor.context();
        let users = &microservices.users;
        let user = users.update_user(id, name, email);
        Ok(user)
    }

    //DELETE /users/:id - удалить пользователя
    field deleteUser(&executor, id: i32) -> FieldResult<()> {
        let microservices = executor.context();
        let users = &microservices.users;
        let user = users.delete_user(id);
        Ok(())
    }
    
});
