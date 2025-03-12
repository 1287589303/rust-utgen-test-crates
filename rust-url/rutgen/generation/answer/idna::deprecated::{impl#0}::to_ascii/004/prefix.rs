// Answer 0

#[test]
fn test_to_ascii_with_success_passthrough() {
    let mut idna = Idna::new(Config::default()
        .transitional_processing(true)
        .verify_dns_length(true)
        .check_hyphens(false));
    let domain = "valid.domain";
    let mut output = String::new();
    idna.to_ascii(domain, &mut output).unwrap();
}

#[test]
fn test_to_ascii_with_success_wrote_to_sink() {
    let mut idna = Idna::new(Config::default()
        .transitional_processing(true)
        .verify_dns_length(true)
        .check_hyphens(false));
    let domain = "valid.channel";
    let mut output = String::new();
    idna.to_ascii(domain, &mut output).unwrap();
}

#[test]
#[should_panic]
fn test_to_ascii_with_validity_error() {
    let mut idna = Idna::new(Config::default()
        .transitional_processing(true)
        .verify_dns_length(true)
        .check_hyphens(false));
    let domain = "invalid!domain"; // Contains invalid character
    let mut output = String::new();
    idna.to_ascii(domain, &mut output).unwrap();
}

#[test]
#[should_panic]
fn test_to_ascii_with_sink_error() {
    let mut idna = Idna::new(Config::default()
        .transitional_processing(true)
        .verify_dns_length(true)
        .check_hyphens(false));
    let domain = "too.long.domain.name.exceeding.maximum.length.abcdefghijk";
    let mut output = String::new();
    idna.to_ascii(domain, &mut output).unwrap();
}

#[test]
#[should_panic]
fn test_to_ascii_with_invalid_dns_length() {
    let mut idna = Idna::new(Config::default()
        .transitional_processing(true)
        .verify_dns_length(true)
        .check_hyphens(false));
    let domain = "invalid..domain"; // Two consecutive dots indicating empty label
    let mut output = String::new();
    idna.to_ascii(domain, &mut output).unwrap();
}

