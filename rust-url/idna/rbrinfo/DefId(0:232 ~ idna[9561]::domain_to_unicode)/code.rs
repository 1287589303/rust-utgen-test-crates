pub fn domain_to_unicode(domain: &str) -> (String, Result<(), Errors>) {
    let (cow, result) = Uts46::new().to_unicode(
        domain.as_bytes(),
        uts46::AsciiDenyList::EMPTY,
        uts46::Hyphens::Allow,
    );
    (cow.into_owned(), result)
}