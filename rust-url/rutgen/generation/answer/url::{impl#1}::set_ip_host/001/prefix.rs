// Answer 0

#[test]
fn test_set_ip_host_with_mailto() {
    let mut url = Url::parse("mailto:rms@example.com").unwrap();
    let result = url.set_ip_host("127.0.0.1".parse().unwrap());
}

#[test]
fn test_set_ip_host_with_invalid_mailto_ip() {
    let mut url = Url::parse("mailto:example@example.com").unwrap();
    let result = url.set_ip_host("192.168.1.1".parse().unwrap());
}

#[test]
fn test_set_ip_host_with_disabled_base() {
    let mut url = Url::parse("mailto:user@example.com").unwrap();
    let result = url.set_ip_host("2001:db8::ff00:42:8329".parse().unwrap());
}

