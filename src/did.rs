use regex::Regex;

const DID_REGEX: &str = r"(?x)
^did:                   # start protocol
(?P<method>
    [a-z0-9]            # method-char        = %x61-7A / DIGIT
    +                   # method-name        = 1*method-char
):
(?P<ids>
    (?::?
        [A-Za-z0-9\.-_] # idchar             = ALPHA / DIGIT / . / - / _
    *)*                 # method-specific-id = *idchar *( : *idchar )
)$                      # end
";

#[derive(Default, Clone, Debug)]
pub struct DecentralizedIdentifer {
    method: String,
    identifiers: Vec<String>,
}

impl DecentralizedIdentifer {
    pub fn new(method: &str) -> Self {
        DecentralizedIdentifer {
            method: method.to_owned(),
            ..DecentralizedIdentifer::default()
        }
    }

    pub fn add_identifier(mut self, identifier: &str) -> Self {
        // TODO validation
        self.identifiers.push(identifier.to_owned());
        self
    }

    pub fn decode(input: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(DID_REGEX).unwrap();
        }
        // TODO graceful error handling
        let caps = RE.captures(input).unwrap();
        DecentralizedIdentifer {
            method: caps["method"].to_owned(),
            identifiers: caps["ids"].split(':').map(|id| id.to_owned()).collect(),
        }
    }

    pub fn encode(&self) -> String {
        format!("did:{}:{}", self.method, self.identifiers.join(":"))
    }
}
