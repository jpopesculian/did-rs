use crate::did::DecentralizedIdentifer;
use crate::did_url::{
    DecentralizedIdentiferParam, DecentralizedIdentiferParams, DecentralizedIdentiferPath,
    DecentralizedIdentiferUrl,
};
use crate::utils::empty_to_none;
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
    pub fn decode(input: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(&format!("^{}$", DID_REGEX)).unwrap();
        }
        DecentralizedIdentifer::from_captures(&RE.captures(input).unwrap())
    }

    fn from_captures(input: &Captures) -> Self {
        let did = DecentralizedIdentifer::new(&input["method"]);
        match &input["ids"] {
            ":" => did,
            ids => ids
                .split(':')
                .skip(1)
                .fold(did, |did, id| did.add_identifier(id)),
        }
    }
}

impl DecentralizedIdentiferUrl {
    pub fn decode(input: &str) -> Self {
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

    pub fn from_captures(input: &Captures) -> Self {
        let mut url = DecentralizedIdentiferUrl::new(DecentralizedIdentifer::from_captures(input));
        url.set_params(DecentralizedIdentiferParams::decode(&input["params"], ';'));
        url.set_path(DecentralizedIdentiferPath::from_captures(input));
        url
    }
}

impl DecentralizedIdentiferPath {
    pub fn from_captures(input: &Captures) -> Self {
        let mut path = DecentralizedIdentiferPath::default();
        path.set_path(empty_to_none(Some(input["path"].to_owned())));
        path.set_params(DecentralizedIdentiferParams::decode(&input["query"], '&'));
        path.set_fragment(empty_to_none(Some(input["fragment"].to_owned())));
        path
    }
}

impl DecentralizedIdentiferParams {
    pub fn decode(input: &str, separator: char) -> Self {
        let mut params = DecentralizedIdentiferParams::default();
        input
            .split(separator)
            .filter(|s| !s.is_empty())
            .map(DecentralizedIdentiferParam::decode)
            .for_each(|param| params.add_param(param));
        params
    }
}

impl DecentralizedIdentiferParam {
    pub fn decode(input: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(&format!("^{}$", DID_URL_PARAM_REGEX)).unwrap();
        }
        let caps = RE.captures(input).unwrap();
        DecentralizedIdentiferParam::from_captures(&caps)
    }

    pub fn from_captures(input: &Captures) -> Self {
        DecentralizedIdentiferParam::new(&input["name"], empty_to_none(Some(&input["value"])))
    }
}
