#[derive(GraphQLObject)]
#[graphql(description = "Information about a user")]
pub struct User {
    #[graphql(description = "The person's id")] pub id: i32,
    #[graphql(description = "The person's full name, including both first and last names")]
    pub name: String,
    #[graphql(description = "The person's email address")] pub email: String,
}

pub struct UsersMicroservice {
    config: Option<String>,
}

#[allow(unused)]
impl UsersMicroservice {
    pub fn new() -> Self {
        UsersMicroservice { config: None }
    }

    pub fn apply(&mut self, config: String) {
        self.config = Some(config);
    }

    //GET /users/:id - детали по пользователю
    pub fn get_by_id(&self, id: i32) -> User {
        User {
            id: 1,
            name: String::from("Luke"),
            email: String::from("example@mail.com"),
        }
    }
    //GET /users?from=<int>&to=<int> - список пользователей
    pub fn get_users(&self, from: i32, to: i32) -> Vec<User> {
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
        vec![user1, user2]
    }
    //POST /users - создать пользователя. + Механизм для подтверждения email, если //не через соцсети
    pub fn create_user(&self, name: String, email: String) -> User {
        User {
            id: 0,
            name: name,
            email: email,
        }
    }
    //PUT /users/:id - апдейт пользователя
    pub fn update_user(&self, id: i32, name: String, email: String) -> User {
        User {
            id: id,
            name: name,
            email: email,
        }
    }
    //DELETE /users/:id - удалить пользователя
    pub fn delete_user(&self, id: i32) {
        ()
    }
}
