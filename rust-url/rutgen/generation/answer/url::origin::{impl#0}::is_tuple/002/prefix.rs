// Answer 0

#[test]
fn test_origin_tuple_http_domain() {
    let origin = Origin::Tuple("http".to_string(), Host::Domain("example.com".to_string()), 80);
    let result = origin.is_tuple();
}

#[test]
fn test_origin_tuple_https_ipv4() {
    let origin = Origin::Tuple("https".to_string(), Host::Ipv4(Ipv4Addr::new(192, 168, 1, 1)), 443);
    let result = origin.is_tuple();
}

