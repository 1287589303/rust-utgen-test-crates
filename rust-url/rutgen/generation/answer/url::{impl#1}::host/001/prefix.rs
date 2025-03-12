// Answer 0

#[test]
fn test_host_ipv6() {
    use crate::net::Ipv6Addr;
    use crate::Url;

    let url = Url {
        serialization: String::from("http://[2001:db8::]/path"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 23,
        host: HostInternal::Ipv6(Ipv6Addr::new(0x20, 0x01, 0x0d, 0xb8, 0x00, 0x00, 0x00, 0x00)),
        port: None,
        path_start: 24,
        query_start: None,
        fragment_start: None,
    };

    let _result = url.host();
}

#[test]
fn test_host_ipv6_with_different_address() {
    use crate::net::Ipv6Addr;
    use crate::Url;

    let url = Url {
        serialization: String::from("http://[::1]/path"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 12,
        host: HostInternal::Ipv6(Ipv6Addr::new(0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01)),
        port: None,
        path_start: 13,
        query_start: None,
        fragment_start: None,
    };

    let _result = url.host();
}

