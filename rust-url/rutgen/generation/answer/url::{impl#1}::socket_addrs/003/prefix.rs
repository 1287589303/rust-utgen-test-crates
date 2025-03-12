// Answer 0

#[test]
fn test_socket_addrs_with_ipv6_address_and_valid_port() {
    let url_str = "http://[2001:db8::1]:8080/";
    let url = url::Url::parse(url_str).unwrap();

    let addrs = url.socket_addrs(|| Some(8080)).unwrap();
}

#[test]
fn test_socket_addrs_with_ipv6_address_and_no_port() {
    let url_str = "http://[2001:db8::1]/";
    let url = url::Url::parse(url_str).unwrap();

    let addrs = url.socket_addrs(|| Some(80)).unwrap();
}

#[test]
fn test_socket_addrs_with_ipv6_address_and_max_port() {
    let url_str = "http://[2001:db8::1]:65535/";
    let url = url::Url::parse(url_str).unwrap();

    let addrs = url.socket_addrs(|| None).unwrap();
}

#[test]
fn test_socket_addrs_with_ipv6_address_and_min_port() {
    let url_str = "http://[2001:db8::1]:1/";
    let url = url::Url::parse(url_str).unwrap();

    let addrs = url.socket_addrs(|| None).unwrap();
}

