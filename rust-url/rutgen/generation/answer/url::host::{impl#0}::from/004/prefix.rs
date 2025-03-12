// Answer 0

#[test]
fn test_domain_non_empty() {
    let host = Host::Domain("example.com".to_string());
    let result: HostInternal = host.into();
}

#[test]
fn test_another_domain_non_empty() {
    let host = Host::Domain("test.host".to_string());
    let result: HostInternal = host.into();
}

#[test]
fn test_numeric_domain() {
    let host = Host::Domain("1234".to_string());
    let result: HostInternal = host.into();
}

#[test]
fn test_special_character_domain() {
    let host = Host::Domain("my-domain.com".to_string());
    let result: HostInternal = host.into();
}

