use juniper;
use juniper::FieldResult;
use context::{Graphql as Context};
use hyper::{Method, Request, Response};
use std::io;
use futures::{Canceled, Future, Stream};
use serde_json::Value;
use serde_json;
use hyper::client::{Client};
use tokio_core::reactor::*;
use futures::oneshot;


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
        
        let url = format!("{}users/{}", context.settings.users_microservice.url, id);
        let req = Request::new(Method::Get, url.parse()?);

        let res = send_request(&*context.tokio_remote, req)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        let values = get_value_from_body(&*context.tokio_remote, res)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        let name = values["name"].as_str()
        .ok_or(io::Error::new(io::ErrorKind::Other,"There is no name field!"))?; 
        let email = values["email"].as_str()
        .ok_or(io::Error::new(io::ErrorKind::Other,"There is no email field!"))?; 


        let user = User {
            id: id,
            name: name.to_string(),
            email: email.to_string(),
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


fn send_request(remote: &Remote, request: Request) -> Result<Response, Canceled> {
    let (tx, rx) = oneshot();
    remote.spawn(|handle| {
        let client = Client::new(&handle);
        client
            .request(request)
            .map_err(|_err| ())
            .and_then(|resp| {
                tx.send(resp).unwrap();
                Ok(())
            })
            .or_else(|_err| Err(()))
    });
    rx.wait()
}

fn get_value_from_body(remote: &Remote, responce: Response) -> Result<Value, Canceled> {
    let (tx, rx) = oneshot();
    remote.spawn(|_| {
        responce
            .body()
            .concat2()
            .and_then(move |body| {
                let v: Value = serde_json::from_slice(&body)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                tx.send(v).unwrap();
                Ok(())
            })
            .or_else(|_err| Err(()))
    });
    rx.wait()
}
