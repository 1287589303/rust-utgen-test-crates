// Answer 0

#[test]
fn test_host_ipv4_0_0_0_0() {
    let url = Url {
        serialization: String::from("http://0.0.0.0"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 11,
        host: HostInternal::Ipv4(Ipv4Addr::new(0, 0, 0, 0)),
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let _ = url.host();
}

#[test]
fn test_host_ipv4_255_255_255_255() {
    let url = Url {
        serialization: String::from("http://255.255.255.255"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 11,
        host: HostInternal::Ipv4(Ipv4Addr::new(255, 255, 255, 255)),
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let _ = url.host();
}

#[test]
fn test_host_ipv4_middle_range() {
    let url = Url {
        serialization: String::from("http://192.168.1.1"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 11,
        host: HostInternal::Ipv4(Ipv4Addr::new(192, 168, 1, 1)),
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let _ = url.host();
}

#[test]
fn test_host_ipv4_edge_cases() {
    let url_edge_low = Url {
        serialization: String::from("http://1.1.1.1"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 11,
        host: HostInternal::Ipv4(Ipv4Addr::new(1, 1, 1, 1)),
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let _ = url_edge_low.host();

    let url_edge_high = Url {
        serialization: String::from("http://255.255.255.255"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 11,
        host: HostInternal::Ipv4(Ipv4Addr::new(255, 255, 255, 255)),
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let _ = url_edge_high.host();
}

