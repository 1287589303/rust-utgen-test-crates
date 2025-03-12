// Answer 0

#[test]
fn test_to_owned_ipv4_valid() {
    let ipv4_address = crate::net::Ipv4Addr::new(192, 168, 1, 1);
    let host = Host::Ipv4(ipv4_address);
    let _result = host.to_owned();
}

#[test]
fn test_to_owned_ipv4_edge_case_zero() {
    let ipv4_address = crate::net::Ipv4Addr::new(0, 0, 0, 0);
    let host = Host::Ipv4(ipv4_address);
    let _result = host.to_owned();
}

#[test]
fn test_to_owned_ipv4_edge_case_max() {
    let ipv4_address = crate::net::Ipv4Addr::new(255, 255, 255, 255);
    let host = Host::Ipv4(ipv4_address);
    let _result = host.to_owned();
}

