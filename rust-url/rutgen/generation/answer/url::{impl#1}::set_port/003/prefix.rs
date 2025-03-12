// Answer 0

#[test]
fn test_set_port_to_default_port() {
    let mut url = Url::parse("http://example.com:80/").unwrap();
    let result = url.set_port(Some(80));
    assert!(result.is_ok());
}

#[test]
fn test_set_port_to_default_https_port() {
    let mut url = Url::parse("https://example.com:443/").unwrap();
    let result = url.set_port(Some(443));
    assert!(result.is_ok());
}

#[test]
fn test_set_port_with_custom_port_but_default_exists() {
    let mut url = Url::parse("ftp://example.com:21/").unwrap();
    let result = url.set_port(Some(21));
    assert!(result.is_ok());
}

#[test]
fn test_set_port_with_custom_port() {
    let mut url = Url::parse("http://example.com:80/").unwrap();
    let result = url.set_port(Some(8080));
    assert!(result.is_ok());
}

