// Answer 0

#[test]
fn test_eq_domain_equal() {
    let host_a = Host::Domain("example.com".to_string());
    let host_b = Host::Domain("example.com".to_string());
    let _result = host_a.eq(&host_b);
}

#[test]
fn test_eq_domain_inequal() {
    let host_a = Host::Domain("example.com".to_string());
    let host_b = Host::Domain("example.org".to_string());
    let _result = host_a.eq(&host_b);
}

