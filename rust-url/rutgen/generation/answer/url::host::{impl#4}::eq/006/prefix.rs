// Answer 0

#[test]
fn test_eq_domain_mismatch() {
    let host_a = Host::Domain(String::from("example.com"));
    let host_b = Host::Domain(String::from("test.com"));
    let _ = host_a.eq(&host_b);
}

#[test]
fn test_eq_domain_ipv4_mismatch() {
    let host_a = Host::Domain(String::from("example.com"));
    let host_b = Host::Ipv4(Ipv4Addr::new(192, 168, 1, 1));
    let _ = host_a.eq(&host_b);
}

#[test]
fn test_eq_domain_ipv6_mismatch() {
    let host_a = Host::Domain(String::from("example.com"));
    let host_b = Host::Ipv6(Ipv6Addr::new(0x20, 0x01, 0x0dB8, 0, 0, 0, 0, 0x1));
    let _ = host_a.eq(&host_b);
}

#[test]
fn test_eq_ipv4_mismatch() {
    let host_a = Host::Ipv4(Ipv4Addr::new(192, 168, 1, 1));
    let host_b = Host::Ipv4(Ipv4Addr::new(10, 0, 0, 1));
    let _ = host_a.eq(&host_b);
}

#[test]
fn test_eq_ipv4_domain_mismatch() {
    let host_a = Host::Ipv4(Ipv4Addr::new(192, 168, 1, 1));
    let host_b = Host::Domain(String::from("example.com"));
    let _ = host_a.eq(&host_b);
}

#[test]
fn test_eq_ipv6_mismatch() {
    let host_a = Host::Ipv6(Ipv6Addr::new(0x20, 0x01, 0x0D, 0xB8, 0, 0, 0, 1));
    let host_b = Host::Ipv6(Ipv6Addr::new(0x20, 0x01, 0x0D, 0xB8, 0, 0, 0, 2));
    let _ = host_a.eq(&host_b);
}

#[test]
fn test_eq_ipv6_domain_mismatch() {
    let host_a = Host::Ipv6(Ipv6Addr::new(0x20, 0x01, 0x0D, 0xB8, 0, 0, 0, 1));
    let host_b = Host::Domain(String::from("example.com"));
    let _ = host_a.eq(&host_b);
}

#[test]
fn test_eq_ipv6_ipv4_mismatch() {
    let host_a = Host::Ipv6(Ipv6Addr::new(0x20, 0x01, 0x0D, 0xB8, 0, 0, 0, 1));
    let host_b = Host::Ipv4(Ipv4Addr::new(192, 168, 1, 1));
    let _ = host_a.eq(&host_b);
}

#[test]
fn test_eq_empty_domain_mismatch() {
    let host_a = Host::Domain(String::from(""));
    let host_b = Host::Domain(String::from("not_empty.com"));
    let _ = host_a.eq(&host_b);
}

#[test]
fn test_eq_special_char_domain_mismatch() {
    let host_a = Host::Domain(String::from("example.com!"));
    let host_b = Host::Domain(String::from("example.com@"));
    let _ = host_a.eq(&host_b);
}

