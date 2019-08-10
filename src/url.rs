use crate::regexes::{DID_REGEX, DID_URL_PARAMS_REGEX};
use crate::DecentralizedIdentifer;
use core::ops::Index;
use regex::Regex;

#[derive(Clone, Debug)]
pub struct DecentralizedIdentiferUrl {
    did: DecentralizedIdentifer,
    params: DecentralizedIdentiferParams,
}

#[derive(Clone, Debug)]
pub struct DecentralizedIdentiferParams(Vec<DecentralizedIdentiferParam>);

#[derive(Clone, Debug)]
pub struct DecentralizedIdentiferParam(String, String);

impl DecentralizedIdentiferUrl {
    pub fn decode(input: &str) {
        lazy_static! {
            static ref RE: Regex = Regex::new(&format!(
                "^(?P<did>{})(?P<params>{})(.*)$",
                DID_REGEX, DID_URL_PARAMS_REGEX
            ))
            .unwrap();
        }
        DecentralizedIdentiferUrl::from_map(&RE.captures(input).unwrap())
    }

    pub fn from_map<'a, M>(input: &M)
    where
        M: Index<&'static str, Output = str> + std::fmt::Debug, // TODO more inclusive output
    {
        let did = DecentralizedIdentifer::from_map(input);
        println!("{:?}", did);
        println!("{:?}", input);
        DecentralizedIdentiferParams::decode(&input["params"]);
    }
}

impl DecentralizedIdentiferParams {
    pub fn decode(input: &str) -> Self {
        DecentralizedIdentiferParams(
            input
                .split(';')
                .skip(1)
                .map(DecentralizedIdentiferParam::decode)
                .collect(),
        )
    }
}

impl DecentralizedIdentiferParam {
    pub fn decode(input: &str) -> Self {
        println!("{}", input);
        DecentralizedIdentiferParam(String::from(""), String::from(""))
    }
}
