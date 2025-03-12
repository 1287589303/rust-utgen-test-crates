// Answer 0

#[test]
fn test_from_empty_domain() {
    let host = Host::Domain(String::new());
    let result = HostInternal::from(host);
}

#[test]
fn test_from_non_empty_domain() {
    let host = Host::Domain(String::from("example.com"));
    let result = HostInternal::from(host);
}

