pub fn domain_to_ascii_strict(domain: &str) -> Result<String, Errors> {
    Uts46::new()
        .to_ascii(
            domain.as_bytes(),
            uts46::AsciiDenyList::STD3,
            uts46::Hyphens::Check,
            uts46::DnsLength::Verify,
        )
        .map(|cow| cow.into_owned())
}