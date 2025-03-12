// Answer 0

#[test]
fn test_set_port_valid() {
    let mut url = Url::parse("ssh://example.net:2048/").unwrap();
    url.set_port(Some(4096)).unwrap();
}

#[test]
fn test_set_port_with_none() {
    let mut url = Url::parse("http://example.com:80/").unwrap();
    url.set_port(None).unwrap();
}

#[test]
fn test_set_port_with_non_default() {
    let mut url = Url::parse("https://example.org:443/").unwrap();
    url.set_port(Some(2048)).unwrap();
}

#[test]
fn test_set_port_with_minimum() {
    let mut url = Url::parse("ftp://example.com:21/").unwrap();
    url.set_port(Some(1)).unwrap();
}

#[test]
fn test_set_port_with_maximum() {
    let mut url = Url::parse("http://example.com:80/").unwrap();
    url.set_port(Some(65535)).unwrap();
}

