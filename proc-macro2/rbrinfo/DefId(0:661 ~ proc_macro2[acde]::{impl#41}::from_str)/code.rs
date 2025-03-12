fn from_str(repr: &str) -> Result<Self, LexError> {
        match imp::Literal::from_str_checked(repr) {
            Ok(lit) => Ok(Literal::_new(lit)),
            Err(lex) => Err(LexError {
                inner: lex,
                _marker: MARKER,
            }),
        }
    }