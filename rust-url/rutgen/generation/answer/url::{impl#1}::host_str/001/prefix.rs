// Answer 0

#[test]
fn test_host_str_with_http() {
    let url = Url::parse("http://example.com/index.html").unwrap();
    url.host_str();
}

#[test]
fn test_host_str_with_https() {
    let url = Url::parse("https://example.com/resource").unwrap();
    url.host_str();
}

#[test]
fn test_host_str_with_localhost() {
    let url = Url::parse("http://localhost:3000").unwrap();
    url.host_str();
}

#[test]
fn test_host_str_with_ipv4() {
    let url = Url::parse("ftp://192.168.0.1/download").unwrap();
    url.host_str();
}

#[test]
fn test_host_str_with_ipv6() {
    let url = Url::parse("https://[::1]/path").unwrap();
    url.host_str();
}

#[test]
fn test_host_str_with_port() {
    let url = Url::parse("https://example.com:8080/resource").unwrap();
    url.host_str();
}

