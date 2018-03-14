use std::fmt;
use std::str::FromStr;

use juniper::FieldError;
use base64::{decode, encode};
use stq_routes::model::Model;
use stq_routes::service::Service;

use config::Config;

pub struct ID {
    pub service: Service,
    pub model: Model,
    pub raw_id: i32,
}

impl fmt::Display for ID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            encode(&*format!("{}|{}|{}", self.service, self.model, self.raw_id))
        )
    }
}

impl FromStr for ID {
    type Err = FieldError;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        let base64 = decode(&*id).map_err(|err| {
            FieldError::new(
                "Id parsing error",
                graphql_value!({ "code": 300, "details": { err.to_string() }}),
            )
        })?;

        let id = String::from_utf8(base64).map_err(|err| {
            FieldError::new(
                "Id parsing error",
                graphql_value!({ "code": 300, "details": { err.to_string() }}),
            )
        })?;

        let v: Vec<&str> = id.split('|').collect();
        if v.len() != 3 {
            return Err(FieldError::new(
                "Id parsing error",
                graphql_value!({ "code": 300, "details": { "can not resolve service, model or id" }}),
            ));
        }

        let service = Service::from_str(v[0])?;
        let model = Model::from_str(v[1])?;
        let raw_id = v[2].parse::<i32>().map_err(|err| {
            FieldError::new(
                "Id parsing error",
                graphql_value!({ "code": 300, "details": { err.to_string() }}),
            )
        })?;
        Ok(ID::new(service, model, raw_id))
    }
}

impl ID {
    pub fn new(service: Service, model: Model, id: i32) -> ID {
        ID {
            service: service,
            model: model,
            raw_id: id,
        }
    }

    pub fn url(self, config: &Config) -> String {
        format!(
            "{}/{}/{}",
            config.service_url(self.service),
            self.model.to_url(),
            self.raw_id
        )
    }
}
