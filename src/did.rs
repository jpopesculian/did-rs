#[derive(Clone, Debug)]
pub struct DecentralizedIdentifer {
    method: String,
    identifiers: Vec<String>,
}

impl DecentralizedIdentifer {
    pub fn new(method: &str) -> Self {
        DecentralizedIdentifer {
            method: method.to_owned(),
            identifiers: vec![],
        }
    }

    pub fn add_identifier(mut self, identifier: &str) -> Self {
        // TODO validation
        self.identifiers.push(identifier.to_owned());
        self
    }

    pub fn remove_identifier(mut self, identifier: &str) -> Self {
        self.identifiers.retain(|id| id != identifier);
        self
    }

    pub fn encode(&self) -> String {
        format!("did:{}:{}", self.method, self.identifiers.join(":"))
    }
}
