fn from_str(input: &str) -> Result<Url, crate::ParseError> {
        Url::parse(input)
    }