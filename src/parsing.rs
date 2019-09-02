use crate::did::DecentralizedIdentifer;
use crate::did_url::{
    DecentralizedIdentiferParam, DecentralizedIdentiferParams, DecentralizedIdentiferPath,
    DecentralizedIdentiferUrl,
};
use crate::utils::empty_to_none;
use crate::{Error, Result};
use regex::{Captures, Regex};

pub const DID_REGEX: &str = r"(?x)
did:                    # start protocol
(?P<method>
    [a-z0-9]            # method-char        = %x61-7A / DIGIT
    +                   # method-name        = 1*method-char
)
(?P<ids>
    (?::
        [A-Za-z0-9\.\-_] # idchar             = ALPHA / DIGIT / '.' / '-' / '_'
    *)*                 # method-specific-id = *idchar *( ':' *idchar )
)                       # end
";

pub const DID_URL_PARAMS_REGEX: &str = r"(?x)
(?:;
    (?:[A-Za-z0-9\.\-_:]|%[0-9a-fA-F][0-9a-fA-F])
       # parma-char  = ALPHA / DIGIT / '.' / '-' / '_' / ':' / pct-encoded
    +  # param-name  = 1*param-char
    =?
    (?:[A-Za-z0-9\.\-_:]|%[0-9a-fA-F][0-9a-fA-F])
    *  # param-value = *param-char
       # param       = param-name [ '=' param-vaue ]
)*     # params      = *( ';' param )
";

pub const DID_URL_PARAM_REGEX: &str = r"(?x)
(?P<name>
    (?:[A-Za-z0-9\.\-_:]|%[0-9a-fA-F][0-9a-fA-F])
        # parma-char  = ALPHA / DIGIT / '.' / '-' / '_' / ':' / pct-encoded
    +   # param-name  = 1*param-char
)
=?
(?P<value>
    (?:[A-Za-z0-9\.\-_:]|%[0-9a-fA-F][0-9a-fA-F])
    *   # param-value = *param-char
        # param       = param-name [ '=' param-vaue ]
)
";

pub const PATH_ABEMPTY_REGEX: &str = r"(?x)
(?:/
    (?:                          # pchar        =
        [A-Za-z0-9\-\._~:@]|     #                unreserved / ':' / '@' /
        %[0-9a-fA-F][0-9a-fA-F]| #                pct-encoded
        [!$&'\(\)\*\+,;=]        #                sub-delims
    )*                           # segment      = *pchar
)*                               # path-abempty = *( '/' segment )
";

pub const PATH_PARAM_REGEX: &str = r"(?x)
(?:                          # pchar        =
    [A-Za-z0-9\-\._~:@/\?]|  #                unreserved / ':' / '@' / '?' / '/'
    %[0-9a-fA-F][0-9a-fA-F]| #                pct-encoded
    [!$&'\(\)\*\+,;=]        #                sub-delims
)*                           # segment      = *pchar
";

impl DecentralizedIdentifer {
    pub fn decode(input: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(&format!("^{}$", DID_REGEX)).unwrap();
        }
        DecentralizedIdentifer::from_captures(&RE.captures(input).unwrap())
    }

    fn from_captures(input: &Captures) -> Result<Self> {
        let did = if let Some(method) = input.name("method") {
            DecentralizedIdentifer::new(method.as_str())
        } else {
            return Err(Error::CouldNotParse("no method".into()));
        };
        Ok(match &input["ids"] {
            ":" => did,
            ids => ids
                .split(':')
                .skip(1)
                .fold(did, |did, id| did.add_identifier(id)),
        })
    }
}

impl DecentralizedIdentiferUrl {
    pub fn decode(input: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(&format!(
                r"(?x)^
                (?P<did>{})
                (?P<params>{})
                (?P<url>
                    (?P<path>{})?
                    (?:\?(?P<query>{}))?
                    (?:\#(?P<fragment>{}))?
                )
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
        DecentralizedIdentiferUrl::from_captures(&caps)
    }

    pub fn from_captures(input: &Captures) -> Result<Self> {
        let mut url = DecentralizedIdentiferUrl::new(DecentralizedIdentifer::from_captures(input)?);
        if let Some(params) = input.name("params") {
            url.set_params(DecentralizedIdentiferParams::decode(params.as_str(), ';')?);
        }
        url.set_path(DecentralizedIdentiferPath::from_captures(input)?);
        Ok(url)
    }
}

impl DecentralizedIdentiferPath {
    pub fn from_captures(input: &Captures) -> Result<Self> {
        let mut path = DecentralizedIdentiferPath::default();
        path.set_path(empty_to_none(
            input.name("path").map(|p| p.as_str().to_owned()),
        ));
        if let Some(query) = input.name("query") {
            path.set_params(DecentralizedIdentiferParams::decode(query.as_str(), '&')?);
        }
        path.set_fragment(empty_to_none(
            input.name("fragment").map(|f| f.as_str().to_owned()),
        ));
        Ok(path)
    }
}

impl DecentralizedIdentiferParams {
    pub fn decode(input: &str, separator: char) -> Result<Self> {
        let params = DecentralizedIdentiferParams::default();
        input
            .split(separator)
            .filter(|s| !s.is_empty())
            .map(DecentralizedIdentiferParam::decode)
            .fold(Ok(params), |res, param| {
                param.and_then(|param| {
                    res.and_then(|mut params| {
                        params.push(param);
                        Ok(params)
                    })
                })
            })
    }
}

impl DecentralizedIdentiferParam {
    pub fn decode(input: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(&format!("^{}$", DID_URL_PARAM_REGEX)).unwrap();
        }
        let caps = RE.captures(input).unwrap();
        DecentralizedIdentiferParam::from_captures(&caps)
    }

    pub fn from_captures(input: &Captures) -> Result<Self> {
        let name = if let Some(name) = input.name("name") {
            name.as_str()
        } else {
            return Err(Error::CouldNotParse("param has no name".to_owned()));
        };
        let value = empty_to_none(input.name("value").map(|x| x.as_str()));
        Ok(DecentralizedIdentiferParam::new(name, value))
    }
}
