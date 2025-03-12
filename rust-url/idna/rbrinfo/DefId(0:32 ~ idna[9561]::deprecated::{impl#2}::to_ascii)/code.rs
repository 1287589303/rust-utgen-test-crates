pub fn to_ascii(self, domain: &str) -> Result<String, Errors> {
        let mut result = String::with_capacity(domain.len());
        let mut codec = Idna::new(self);
        codec.to_ascii(domain, &mut result).map(|()| result)
    }