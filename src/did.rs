use crate::regexes::DID_REGEX;
use core::ops::Index;
use regex::Regex;

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

    pub fn decode(input: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(&format!("^{}$", DID_REGEX)).unwrap();
        }
        DecentralizedIdentifer::from_map(&RE.captures(input).unwrap())
    }

    pub fn from_map<M>(input: &M) -> Self
    where
        M: Index<&'static str, Output = str>, // TODO more inclusive output
    {
        // TODO graceful error handling
        let identifiers: Vec<String> = match &input["ids"] {
            ":" => vec![],
            ids => ids.split(':').skip(1).map(|id| id.to_owned()).collect(),
        };
        DecentralizedIdentifer {
            method: input["method"].to_owned(),
            identifiers,
        }
    }

    pub fn encode(&self) -> String {
        format!("did:{}:{}", self.method, self.identifiers.join(":"))
    }
}
