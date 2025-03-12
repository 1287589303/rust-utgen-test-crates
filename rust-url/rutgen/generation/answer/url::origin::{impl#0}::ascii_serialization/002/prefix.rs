// Answer 0

#[test]
fn test_ascii_serialization_http_case() {
    let scheme = "http".to_owned();
    let host = Host::Domain("example.com".to_owned());
    let port = 8080;
    let origin = Origin::Tuple(scheme, host, port);
    let _ = origin.ascii_serialization();
}

#[test]
fn test_ascii_serialization_ftp_case() {
    let scheme = "ftp".to_owned();
    let host = Host::Ipv4("192.168.1.1".parse().unwrap());
    let port = 22;
    let origin = Origin::Tuple(scheme, host, port);
    let _ = origin.ascii_serialization();
}

#[test]
fn test_ascii_serialization_https_case() {
    let scheme = "https".to_owned();
    let host = Host::Ipv6("::1".parse().unwrap());
    let port = 4443;
    let origin = Origin::Tuple(scheme, host, port);
    let _ = origin.ascii_serialization();
}

