pub fn query_pairs(&self) -> form_urlencoded::Parse<'_> {
        form_urlencoded::parse(self.query().unwrap_or("").as_bytes())
    }