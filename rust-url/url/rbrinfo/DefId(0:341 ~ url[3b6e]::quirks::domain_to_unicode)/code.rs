pub fn domain_to_unicode(domain: &str) -> String {
    match Host::parse(domain) {
        Ok(Host::Domain(ref domain)) => {
            let (unicode, _errors) = idna::domain_to_unicode(domain);
            unicode
        }
        _ => String::new(),
    }
}