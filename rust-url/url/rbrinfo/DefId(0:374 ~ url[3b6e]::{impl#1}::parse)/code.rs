pub fn parse(input: &str) -> Result<Url, crate::ParseError> {
        Url::options().parse(input)
    }