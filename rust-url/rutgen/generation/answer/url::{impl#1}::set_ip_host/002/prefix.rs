// Answer 0

#[test]
fn test_set_ip_host_valid_ipv6() {
    let mut url = Url::parse("http://example.com").unwrap();
    let ipv6_address: IpAddr = "2001:db8::ff00:42:8329".parse().unwrap();
    url.set_ip_host(ipv6_address).unwrap();
}

#[test]
fn test_set_ip_host_valid_ipv6_with_different_structure() {
    let mut url = Url::parse("https://example.com/path").unwrap();
    let ipv6_address: IpAddr = "fe80::1ff:fe23:4567:890a".parse().unwrap();
    url.set_ip_host(ipv6_address).unwrap();
}

