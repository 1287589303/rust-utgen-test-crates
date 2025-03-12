fn from(_: ::idna::Errors) -> ParseError {
        ParseError::IdnaError
    }