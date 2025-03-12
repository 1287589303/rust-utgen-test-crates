// Answer 0

#[test]
fn test_to_ascii_valid_ascii() {
    let config = Config::default();
    let domain = "example.com";
    let _ = config.to_ascii(domain);
}

#[test]
fn test_to_ascii_valid_idn() {
    let config = Config::default();
    let domain = "xn--exmple-cua.com";
    let _ = config.to_ascii(domain);
}

#[test]
fn test_to_ascii_empty_string() {
    let config = Config::default();
    let domain = "";
    let _ = config.to_ascii(domain);
}

#[test]
fn test_to_ascii_with_hyphens() {
    let config = Config::default();
    let domain = "example-domain.com";
    let _ = config.to_ascii(domain);
}

#[test]
fn test_to_ascii_exceeding_dns_length() {
    let config = Config::default();
    let domain = "a".repeat(254) + ".com"; // Over 253 characters
    let _ = config.to_ascii(&domain);
}

#[test]
fn test_to_ascii_invalid_domain() {
    let config = Config::default();
    let domain = "!!!";
    let _ = config.to_ascii(domain);
}

