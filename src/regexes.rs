pub const DID_REGEX: &str = r"(?x)
did:                    # start protocol
(?P<method>
    [a-z0-9]            # method-char        = %x61-7A / DIGIT
    +                   # method-name        = 1*method-char
)
(?P<ids>
    (?::
        [A-Za-z0-9-_\.] # idchar             = ALPHA / DIGIT / '.' / '-' / '_'
    *)*                 # method-specific-id = *idchar *( ':' *idchar )
)                       # end
";

pub const DID_URL_PARAMS_REGEX: &str = r"(?x)
(?:;
    [A-Za-z0-9\.-_:] # parma-char  = ALPHA / DIGIT / '.' / '-' / '_' / ':' / pct-encoded (TODO)
    +                # param-name  = 1*param-char
    =?
    [A-Za-z0-9\.-_:] # param-value = *param-char
    *                # param       = param-name [ '=' param-vaue ]
)*                   # params      = *( ';' param )
";
