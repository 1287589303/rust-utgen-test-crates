// Answer 0

#[test]
fn test_from_ipv6_valid_compressed() {
    let host = Host::Ipv6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)); // "::1"
    let _result: HostInternal = HostInternal::from(host);
}

#[test]
fn test_from_ipv6_valid_full() {
    let host = Host::Ipv6(Ipv6Addr::new(8193, 3512, 0, 66, 0, 35342, 14016, 29552)); // "2001:0db8:0000:0042:0000:8a2e:0370:7334"
    let _result: HostInternal = HostInternal::from(host);
}

#[test]
fn test_from_ipv6_mixed_case() {
    let host = Host::Ipv6(Ipv6Addr::new(8193, 3512, 0, 66, 0, 35342, 14016, 29552)); // "2001:0DB8:0000:0042:0000:8A2E:0370:7334"
    let _result: HostInternal = HostInternal::from(host);
}

#[test]
fn test_from_ipv6_leading_zeros() {
    let host = Host::Ipv6(Ipv6Addr::new(8193, 3512, 0, 66, 0, 35342, 0, 29552)); // "2001:0db8:0:42:0:8a2e:0:7334"
    let _result: HostInternal = HostInternal::from(host);
}

