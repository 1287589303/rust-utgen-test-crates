// Answer 0

#[test]
fn test_port_or_known_default_http() {
    let url = Url::parse("http://example.com").unwrap();
    url.port_or_known_default(); // Expected Some(80)

    let url = Url::parse("http://example.com:8080").unwrap();
    url.port_or_known_default(); // Expected Some(8080)
}

#[test]
fn test_port_or_known_default_https() {
    let url = Url::parse("https://example.com").unwrap();
    url.port_or_known_default(); // Expected Some(443)

    let url = Url::parse("https://example.com:8443").unwrap();
    url.port_or_known_default(); // Expected Some(8443)
}

#[test]
fn test_port_or_known_default_ws() {
    let url = Url::parse("ws://example.com").unwrap();
    url.port_or_known_default(); // Expected Some(80)

    let url = Url::parse("ws://example.com:1234").unwrap();
    url.port_or_known_default(); // Expected Some(1234)
}

#[test]
fn test_port_or_known_default_wss() {
    let url = Url::parse("wss://example.com").unwrap();
    url.port_or_known_default(); // Expected Some(443)

    let url = Url::parse("wss://example.com:5678").unwrap();
    url.port_or_known_default(); // Expected Some(5678)
}

#[test]
fn test_port_or_known_default_ftp() {
    let url = Url::parse("ftp://example.com").unwrap();
    url.port_or_known_default(); // Expected Some(21)

    let url = Url::parse("ftp://example.com:2121").unwrap();
    url.port_or_known_default(); // Expected Some(2121)
}

#[test]
fn test_port_or_known_default_invalid_url() {
    let url = Url::parse("foo://example.com").unwrap();
    url.port_or_known_default(); // Expected None

    let url = Url::parse("foo://example.com:1456").unwrap();
    url.port_or_known_default(); // Expected Some(1456)
}

#[test]
fn test_port_or_known_default_no_scheme() {
    let url = Url::parse("//example.com").unwrap();
    url.port_or_known_default(); // Expected None

    let url = Url::parse("//example.com:1234").unwrap();
    url.port_or_known_default(); // Expected Some(1234)
}

#[test]
fn test_port_or_known_default_empty_string() {
    let url = Url::parse("").unwrap();
    url.port_or_known_default(); // Expected None
}

#[test]
fn test_port_or_known_default_malformed_url() {
    let url = Url::parse("://").unwrap();
    url.port_or_known_default(); // Expected None

    let url = Url::parse("htp://example.com").unwrap();
    url.port_or_known_default(); // Expected None
}

#[test]
fn test_port_or_known_default_max_port() {
    let url = Url::parse("http://example.com:65535").unwrap();
    url.port_or_known_default(); // Expected Some(65535)
}

