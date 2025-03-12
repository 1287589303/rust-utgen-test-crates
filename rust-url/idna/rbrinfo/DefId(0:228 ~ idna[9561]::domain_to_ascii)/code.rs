pub fn domain_to_ascii(domain: &str) -> Result<String, Errors> {
    domain_to_ascii_cow(domain.as_bytes(), AsciiDenyList::EMPTY).map(|cow| cow.into_owned())
}