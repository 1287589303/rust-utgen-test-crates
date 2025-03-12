// Answer 0

#[test]
fn test_set_port_valid() {
    let mut url = Url::parse("http://example.com:8080/").unwrap();
    let result = url.set_port(Some(9090));
}

#[test]
fn test_set_port_https() {
    let mut url = Url::parse("https://example.com:8443/").unwrap();
    let result = url.set_port(Some(2048));
}

#[test]
fn test_set_port_with_non_default() {
    let mut url = Url::parse("ftp://example.com:21/").unwrap();
    let result = url.set_port(Some(2121));
}

