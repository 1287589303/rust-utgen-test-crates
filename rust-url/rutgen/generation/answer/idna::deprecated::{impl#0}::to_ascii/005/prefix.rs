// Answer 0

#[test]
fn test_to_ascii_success_passthrough() {
    let config = Config {
        use_std3_ascii_rules: false,
        transitional_processing: false,
        verify_dns_length: false,
        check_hyphens: false,
    };
    let mut idna = Idna::new(config);
    let domain = "valid-domain.com";
    let mut out = String::new();

    idna.to_ascii(domain, &mut out).unwrap();
}

#[test]
fn test_to_ascii_success_wrote_to_sink() {
    let config = Config {
        use_std3_ascii_rules: false,
        transitional_processing: false,
        verify_dns_length: false,
        check_hyphens: false,
    };
    let mut idna = Idna::new(config);
    let domain = "xn--80akhbyknj4f"; // Punycode for validation
    let mut out = String::new();

    idna.to_ascii(domain, &mut out).unwrap();
}

#[test]
fn test_to_ascii_validity_error() {
    let config = Config {
        use_std3_ascii_rules: false,
        transitional_processing: false,
        verify_dns_length: false,
        check_hyphens: false,
    };
    let mut idna = Idna::new(config);
    let domain = "invalid--domain.com"; // Invalid domain with double hyphen
    let mut out = String::new();

    let result = idna.to_ascii(domain, &mut out);
    assert!(result.is_err());
}

#[test]
fn test_to_ascii_sink_error() {
    let config = Config {
        use_std3_ascii_rules: false,
        transitional_processing: false,
        verify_dns_length: false,
        check_hyphens: false,
    };
    let mut idna = Idna::new(config);
    let domain = ""; // An empty domain string
    let mut out = String::new();

    let result = idna.to_ascii(domain, &mut out);
    assert!(result.is_err());
}

