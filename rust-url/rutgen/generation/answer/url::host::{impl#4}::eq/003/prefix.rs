// Answer 0

#[test]
fn test_eq_ipv6_equal() {
    let self_host = Host::Ipv6(Ipv6Addr::new(0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0001)); // ::1
    let other_host = Host::Ipv6(Ipv6Addr::new(0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0001)); // ::1
    let _ = self_host.eq(&other_host);
}

#[test]
fn test_eq_ipv6_not_equal() {
    let self_host = Host::Ipv6(Ipv6Addr::new(0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0001)); // ::1
    let other_host = Host::Ipv6(Ipv6Addr::new(0x7fff, 0xff00, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0001)); // 7fff:ff00:0:0:0:0:0:1
    let _ = self_host.eq(&other_host);
}

#[test]
fn test_eq_ipv6_edge_case_empty() {
    let self_host = Host::Ipv6(Ipv6Addr::new(0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000)); // ::
    let other_host = Host::Ipv6(Ipv6Addr::new(0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000)); // ::
    let _ = self_host.eq(&other_host);
}

#[test]
fn test_eq_ipv6_fully_expanded() {
    let self_host = Host::Ipv6(Ipv6Addr::new(0x2001, 0x0db8, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0001)); // 2001:db8::1
    let other_host = Host::Ipv6(Ipv6Addr::new(0x2001, 0x0db8, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0001)); // 2001:db8::1
    let _ = self_host.eq(&other_host);
}

