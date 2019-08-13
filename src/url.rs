use crate::regexes::{
    DID_REGEX, DID_URL_PARAMS_REGEX, DID_URL_PARAM_REGEX, PATH_ABEMPTY_REGEX, PATH_PARAM_REGEX,
};
use crate::utils::empty_to_none;
use crate::DecentralizedIdentifer;
use core::ops::Index;
use regex::Regex;

#[derive(Clone, Debug)]
pub struct DecentralizedIdentiferUrl {
    did: DecentralizedIdentifer,
    params: DecentralizedIdentiferParams,
    path: Option<String>,
    query: Option<String>,
    fragment: Option<String>,
}

#[derive(Clone, Debug)]
pub struct DecentralizedIdentiferParams(Vec<DecentralizedIdentiferParam>);

#[derive(Clone, Debug)]
pub struct DecentralizedIdentiferParam(String, Option<String>);

impl DecentralizedIdentiferUrl {
    pub fn decode(input: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(&format!(
                r"(?x)^
                (?P<did>{})
                (?P<params>{})
                (?P<path>{})?
                (?:\?(?P<query>{}))?
                (?:\#(?P<fragment>{}))?
                $",
                DID_REGEX,
                DID_URL_PARAMS_REGEX,
                PATH_ABEMPTY_REGEX,
                PATH_PARAM_REGEX,
                PATH_PARAM_REGEX
            ))
            .unwrap();
        }
        let caps = RE.captures(input).unwrap();
        println!("{:#?}", caps);
        DecentralizedIdentiferUrl::from_map(&caps)
    }

    pub fn from_map<M>(input: &M) -> Self
    where
        M: Index<&'static str, Output = str> + std::fmt::Debug, // TODO more inclusive output
    {
        // TODO fix this! this will fail if fragment is empty
        DecentralizedIdentiferUrl {
            did: DecentralizedIdentifer::from_map(input),
            params: DecentralizedIdentiferParams::decode(&input["params"]),
            path: empty_to_none(Some(input["path"].to_owned())),
            query: Some(input["query"].to_owned()),
            fragment: Some(input["fragment"].to_owned()),
        }
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
        lazy_static! {
            static ref RE: Regex = Regex::new(&format!("^{}$", DID_URL_PARAM_REGEX)).unwrap();
        }
        let caps = RE.captures(input).unwrap();
        DecentralizedIdentiferParam::from_map(&caps)
    }

    pub fn from_map<M>(input: &M) -> Self
    where
        M: Index<&'static str, Output = str> + std::fmt::Debug,
    {
        DecentralizedIdentiferParam(
            input["name"].to_string(),
            empty_to_none(Some(input["value"].to_string())),
        )
    }
}
