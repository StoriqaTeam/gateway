#[derive(Clone)]
pub struct Pool {
    base_url: String,
}

impl Pool {
    pub fn new(base_url: String) -> Self {
        Pool { base_url: base_url }
    }
}


#[cfg(test)]
mod tests {
    use pool::Pool;

    #[test]
    fn can_create_pool() {
        let pool = Pool::new("http://0.0.0.0:8000/");
    }

}
