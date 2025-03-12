// Answer 0

#[test]
fn test_parse_valid_domain_name() {
    let input = "example.com";
    let result = Host::parse(input);
}

#[test]
fn test_parse_valid_domain_name_with_hyphen() {
    let input = "my-site";
    let result = Host::parse(input);
}

#[test]
fn test_parse_valid_domain_name_with_number() {
    let input = "test-domain-abc"; // number is not at the end
    let result = Host::parse(input);
}

#[test]
fn test_parse_valid_domain_name_with_mixed_case() {
    let input = "ExAmPle.Com";
    let result = Host::parse(input);
}

#[test]
fn test_parse_valid_domain_name_with_subdomain() {
    let input = "sub.example.com";
    let result = Host::parse(input);
}

