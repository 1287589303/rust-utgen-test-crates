// Answer 0

#[test]
fn test_valid_ascii_domain() {
    let domain = "example.com";
    let _result = domain_to_ascii(domain);
}

#[test]
fn test_invalid_non_ascii_domain() {
    let domain = "exámple.com";
    let _result = domain_to_ascii(domain);
}

#[test]
fn test_domain_with_hyphens() {
    let domain = "example-domain.com";
    let _result = domain_to_ascii(domain);
}

#[test]
fn test_empty_string() {
    let domain = "";
    let _result = domain_to_ascii(domain);
}

#[test]
fn test_domain_exceeding_length_limit() {
    let domain = "a".repeat(254) + ".com"; // 254 characters before the dot
    let _result = domain_to_ascii(&domain);
}

#[test]
fn test_mixed_case_domain() {
    let domain = "ExAmPlE.cOm";
    let _result = domain_to_ascii(domain);
}

