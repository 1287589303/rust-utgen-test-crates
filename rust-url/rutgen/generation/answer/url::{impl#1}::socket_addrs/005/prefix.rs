// Answer 0

#[test]
fn test_socket_addrs_valid_domain_default_port() {
    let url = url::Url::parse("https://example.com").unwrap();
    let addrs = url.socket_addrs(|| None).unwrap();
}

#[test]
fn test_socket_addrs_valid_domain_specific_port() {
    let url = url::Url::parse("http://example.org:8080").unwrap();
    let addrs = url.socket_addrs(|| None).unwrap();
}

#[test]
fn test_socket_addrs_valid_domain_with_credentials() {
    let url = url::Url::parse("ftp://username:password@example.net:21").unwrap();
    let addrs = url.socket_addrs(|| None).unwrap();
}

#[test]
fn test_socket_addrs_valid_ipv4_address() {
    let url = url::Url::parse("http://192.168.1.1").unwrap();
    let addrs = url.socket_addrs(|| Some(80)).unwrap();
}

#[test]
#[should_panic(expected = "No host name in the URL")]
fn test_socket_addrs_missing_host() {
    let url = url::Url::parse("http:///").unwrap();
    let addrs = url.socket_addrs(|| None).unwrap();
}

#[test]
#[should_panic(expected = "No port number in the URL")]
fn test_socket_addrs_missing_port() {
    let url = url::Url::parse("http://example.com").unwrap();
    let addrs = url.socket_addrs(|| None).unwrap();
}

#[test]
fn test_socket_addrs_invalid_domain() {
    let url = url::Url::parse("http://invalid_domain").unwrap();
    let addrs = url.socket_addrs(|| None).unwrap();
}

