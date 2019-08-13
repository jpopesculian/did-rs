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
