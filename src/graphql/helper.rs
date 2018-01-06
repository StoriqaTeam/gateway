use std::fmt;
use base64::encode as encode_to_base64;
use base64::decode as decode_from_base64;
use juniper::FieldError;
use std::str::FromStr;

pub enum Services {
    Users,
}

impl fmt::Display for Services {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Services::Users => write!(f, "users"),
        }
    }
}

impl FromStr for Services {
    type Err = FieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "users" => Ok(Services::Users),
            _ => {
                return Err(FieldError::new(
                    "Unknown service",
                    graphql_value!({ "code": 300, "details": { 
                        format!("Can not resolve service name. Unknown service: '{}'", s) 
                        }}),
                ))
            }
        }
    }
}

pub enum Models {
    User,
}

impl fmt::Display for Models {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Models::User => write!(f, "user"),
        }
    }
}

impl FromStr for Models {
    type Err = FieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "user" => Ok(Models::User),
            _ => {
                return Err(FieldError::new(
                    "Unknown model",
                    graphql_value!({ "code": 300, "details": { 
                        format!("Can not resolve model name. Unknown model: '{}'", s) 
                        }}),
                ))
            }
        }
    }
}

pub struct ID {
    pub service: Services,
    pub model: Models,
    pub raw_id: i32,
}

impl fmt::Display for ID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            encode_to_base64(&*format!("{}_{}_{}", self.service, self.model, self.raw_id))
        )
    }
}

impl FromStr for ID {
    type Err = FieldError;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        let base64 = decode_from_base64(&*id).map_err(|err| {
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

        let v: Vec<&str> = id.split('_').collect();
        if v.len() != 3 {
            return Err(FieldError::new(
                "Id parsing error",
                graphql_value!({ "code": 300, "details": { "can not resolve service, model or id" }}),
            ));
        }

        let service = Services::from_str(v[0])?;
        let model = Models::from_str(v[1])?;
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
    pub fn new(service: Services, model: Models, id: i32) -> ID {
        ID {
            service: service,
            model: model,
            raw_id: id,
        }
    }
}
