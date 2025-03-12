fn domain_to_ascii(domain: &[u8]) -> Result<Cow<'_, str>, ParseError> {
        idna::domain_to_ascii_cow(domain, idna::AsciiDenyList::URL).map_err(Into::into)
    }