// Answer 0

#[test]
fn test_parse_valid_ipv4_host() {
    let input = "192.168.1.1";
    let result = Host::parse(input);
}

#[test]
fn test_parse_percent_encoded_ipv4_host() {
    let input = "192.168.1%2E1"; // represents 192.168.1.1
    let result = Host::parse(input);
}

#[test]
fn test_parse_ipv4_host_with_valid_ascii() {
    let input = "example.com.123";
    let result = Host::parse(input);
}

#[test]
fn test_parse_ipv4_host_with_boundary_values() {
    let input = "255.255.255.255";
    let result = Host::parse(input);
}

#[test]
fn test_parse_mixed_encoded_ipv4_host() {
    let input = "example.com%2E0";
    let result = Host::parse(input);
}

#[test]
fn test_parse_ipv4_host_with_special_characters() {
    let input = "my-host.com.42"; // contains valid ASCII characters
    let result = Host::parse(input);
}

#[test]
fn test_parse_domain_with_ending_number() {
    let input = "valid-domain-123.com";
    let result = Host::parse(input);
}

