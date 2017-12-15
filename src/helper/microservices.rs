use juniper;
use helper::users::UsersMicroservice;
pub struct Microservices {
    pub users: UsersMicroservice,
}

impl Microservices {
    pub fn new() -> Self {
        let users = UsersMicroservice::new();
        Microservices { users: users }
    }
    pub fn apply(&mut self, config: String) {
        self.users.apply(config);
    }
}

impl juniper::Context for Microservices {}


#[cfg(test)]
mod tests {
    use helper::microservices::Microservices;
    use helper::users::UsersMicroservice;

    #[test]
    fn can_create_microservices() {
        let m: Microservices<UsersMicroservice> = Microservices::new();
    }

}
