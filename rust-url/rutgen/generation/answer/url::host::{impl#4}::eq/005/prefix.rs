// Answer 0

#[test]
fn test_eq_ipv4_equal_edge_case_zero() {
    let ip1 = Ipv4Addr::new(0, 0, 0, 0);
    let ip2 = Ipv4Addr::new(0, 0, 0, 0);
    let host1 = Host::Ipv4(ip1);
    let host2 = Host::Ipv4(ip2);
    let _ = host1.eq(&host2);
}

#[test]
fn test_eq_ipv4_equal_edge_case_max() {
    let ip1 = Ipv4Addr::new(255, 255, 255, 255);
    let ip2 = Ipv4Addr::new(255, 255, 255, 255);
    let host1 = Host::Ipv4(ip1);
    let host2 = Host::Ipv4(ip2);
    let _ = host1.eq(&host2);
}

#[test]
fn test_eq_ipv4_equal_mid_range() {
    let ip1 = Ipv4Addr::new(128, 0, 0, 1);
    let ip2 = Ipv4Addr::new(128, 0, 0, 1);
    let host1 = Host::Ipv4(ip1);
    let host2 = Host::Ipv4(ip2);
    let _ = host1.eq(&host2);
}

#[test]
fn test_eq_ipv4_not_equal_different() {
    let ip1 = Ipv4Addr::new(192, 168, 1, 1);
    let ip2 = Ipv4Addr::new(10, 0, 0, 1);
    let host1 = Host::Ipv4(ip1);
    let host2 = Host::Ipv4(ip2);
    let _ = host1.eq(&host2);
}

#[test]
fn test_eq_ipv4_different_class_a_class_c() {
    let ip1 = Ipv4Addr::new(10, 0, 0, 1);
    let ip2 = Ipv4Addr::new(172, 16, 0, 1);
    let host1 = Host::Ipv4(ip1);
    let host2 = Host::Ipv4(ip2);
    let _ = host1.eq(&host2);
}

