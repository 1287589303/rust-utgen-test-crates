// Answer 0

#[test]
fn test_domain_to_ascii_valid_ascii() {
    let domain: &[u8] = b"example.com";
    let _ = Host::<String>::domain_to_ascii(domain);
}

#[test]
fn test_domain_to_ascii_valid_non_ascii() {
    let domain: &[u8] = b"xn--ls8h.XN--80AQEOL";
    let _ = Host::<String>::domain_to_ascii(domain);
}

#[test]
fn test_domain_to_ascii_empty() {
    let domain: &[u8] = b"";
    let _ = Host::<String>::domain_to_ascii(domain).expect_err("Expected error for empty domain");
}

#[test]
fn test_domain_to_ascii_with_control_characters() {
    let domain: &[u8] = b"example\0.com";
    let _ = Host::<String>::domain_to_ascii(domain).expect_err("Expected error for domain with control characters");
}

#[test]
fn test_domain_to_ascii_invalid_idna_characters() {
    let domain: &[u8] = b"ex@ample.com";
    let _ = Host::<String>::domain_to_ascii(domain).expect_err("Expected error for domain with invalid IDNA characters");
}

