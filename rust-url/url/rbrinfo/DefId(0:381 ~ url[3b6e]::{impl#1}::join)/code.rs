pub fn join(&self, input: &str) -> Result<Url, crate::ParseError> {
        Url::options().base_url(Some(self)).parse(input)
    }