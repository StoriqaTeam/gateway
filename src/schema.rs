use juniper;
use juniper::FieldResult;
use context::Context;

pub struct Query;
pub struct Mutation;

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn create() -> Schema {
    let query = Query {};
    let mutation = Mutation {};
    Schema::new(query, mutation)
}

#[derive(GraphQLObject)]
#[graphql(description = "Information about a user")]
pub struct User {
    #[graphql(description = "The person's id")] 
    pub id: i32,

    #[graphql(description = "The person's full name, including both first and last names")]
    pub name: String,

    #[graphql(description = "The person's email address")] 
    pub email: String,
}

graphql_object!(Query: Context |&self| {

    field apiVersion() -> &str {
        "1.0"
    }

    field user(&executor, id: i32) -> FieldResult<User> {
        let context = executor.context();
        let pool = &context.users_connection_pool;

        let user = User {
            id: 1,
            name: String::from("Luke"),
            email: String::from("example@mail.com"),
        };
        Ok(user)
    }
    
    field users(&executor, from: i32, to: i32) -> FieldResult<Vec<User>> {
        let context = executor.context();
        let user1 = User {
            id: 1,
            name: String::from("Luke"),
            email: String::from("example@mail.com"),
        };

        let user2 = User {
            id: 2,
            name: String::from("Mike"),
            email: String::from("elpmaxe@mail.com"),
        };
        let users = vec![user1, user2];
        Ok(users)
    }
});



//mutation {
//  createUser(name: "andy", email: "hope is a good thing") {
//    id
//  }
//}

graphql_object!(Mutation: Context |&self| {

    //POST /users - создать пользователя. + Механизм для подтверждения email, если //не через соцсети
    field createUser(&executor, name: String, email: String) -> FieldResult<User> {
        let context = executor.context();
        let user = User {
            id: 0,
            name: name,
            email: email,
        };
        Ok(user)
    }

    //PUT /users/:id - апдейт пользователя
    field updateUser(&executor,id: i32, name: String, email: String) -> FieldResult<User> {
        let context = executor.context();
        let user = User {
            id: 0,
            name: name,
            email: email,
        };
        Ok(user)
    }

    //DELETE /users/:id - удалить пользователя
    field deleteUser(&executor, id: i32) -> FieldResult<()> {
        let context = executor.context();
        Ok(())
    }
    
});
