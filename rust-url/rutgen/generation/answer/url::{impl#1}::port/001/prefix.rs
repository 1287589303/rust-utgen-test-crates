// Answer 0

#[test]
fn test_port_none_no_port() {
    let url = Url::parse("https://example.com").unwrap();
    url.port();
}

#[test]
fn test_port_none_default_port() {
    let url = Url::parse("https://example.com:443/").unwrap();
    url.port();
}

#[test]
fn test_port_some_valid_port() {
    let url = Url::parse("ssh://example.com:22").unwrap();
    url.port();
}

#[test]
fn test_port_some_minimum_valid_port() {
    let url = Url::parse("http://example.com:0").unwrap();
    url.port();
}

#[test]
fn test_port_some_maximum_valid_port() {
    let url = Url::parse("http://example.com:65535").unwrap();
    url.port();
}

#[test]
fn test_port_some_specific_port() {
    let url = Url::parse("http://example.com:80").unwrap();
    url.port();
}

#[test]
fn test_port_invalid_input() {
    let result = Url::parse("invalid_url_string");
    assert!(result.is_err());
}

