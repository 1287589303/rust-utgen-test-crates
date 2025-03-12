pub fn verify_dns_length(domain_name: &str, allow_trailing_dot: bool) -> bool {
    let bytes = domain_name.as_bytes();
    debug_assert!(bytes.is_ascii());
    let domain_name_without_trailing_dot = if let Some(without) = bytes.strip_suffix(b".") {
        if !allow_trailing_dot {
            return false;
        }
        without
    } else {
        bytes
    };
    if domain_name_without_trailing_dot.len() > 253 {
        return false;
    }
    for label in domain_name_without_trailing_dot.split(|b| *b == b'.') {
        if label.is_empty() {
            return false;
        }
        if label.len() > 63 {
            return false;
        }
    }
    true
}