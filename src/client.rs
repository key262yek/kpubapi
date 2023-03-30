pub struct Client {}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }
}

pub struct ClientBuilder {}

impl ClientBuilder {
    fn new() -> Self {
        Self {}
    }
}
