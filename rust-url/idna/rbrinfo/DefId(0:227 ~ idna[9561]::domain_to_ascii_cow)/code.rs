pub fn domain_to_ascii_cow(
    domain: &[u8],
    ascii_deny_list: AsciiDenyList,
) -> Result<Cow<'_, str>, Errors> {
    Uts46::new().to_ascii(
        domain,
        ascii_deny_list,
        uts46::Hyphens::Allow,
        uts46::DnsLength::Ignore,
    )
}