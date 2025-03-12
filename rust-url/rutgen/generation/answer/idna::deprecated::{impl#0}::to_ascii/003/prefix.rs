// Answer 0

#[test]
fn test_to_ascii_valid_passthrough() {
    let mut out = String::new();
    let config = Config::default()
        .transitional_processing(true)
        .verify_dns_length(true)
        .check_hyphens(true);
    let mut idna = Idna::new(config);
    let domain = "example.com";
    idna.to_ascii(domain, &mut out).unwrap();
}

#[test]
fn test_to_ascii_valid_wrote_to_sink() {
    let mut out = String::new();
    let config = Config::default()
        .transitional_processing(false)
        .verify_dns_length(true)
        .check_hyphens(false);
    let mut idna = Idna::new(config);
    let domain = "test-domain.com";
    idna.to_ascii(domain, &mut out).unwrap();
}

#[test]
#[should_panic]
fn test_to_ascii_invalid_validity_error() {
    let mut out = String::new();
    let config = Config::default()
        .transitional_processing(true)
        .verify_dns_length(true)
        .check_hyphens(true);
    let mut idna = Idna::new(config);
    let domain = "invalid_domain_with_\"quotes\".com";
    idna.to_ascii(domain, &mut out).unwrap();
}

#[test]
#[should_panic]
fn test_to_ascii_invalid_sink_error() {
    let mut out = String::new();
    let config = Config::default()
        .transitional_processing(true)
        .verify_dns_length(false)
        .check_hyphens(false);
    let mut idna = Idna::new(config);
    let domain = "invalid..domain.com";
    idna.to_ascii(domain, &mut out).unwrap();
}

