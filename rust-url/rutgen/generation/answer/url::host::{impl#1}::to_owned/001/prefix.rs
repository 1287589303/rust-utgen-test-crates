// Answer 0

#[test]
fn test_to_owned_ipv6_address_full_compression() {
    let address = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1); // Represents ::1
    let host = Host::Ipv6(address);
    let _result = host.to_owned();
}

#[test]
fn test_to_owned_ipv6_address_loopback() {
    let address = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0); // Represents ::
    let host = Host::Ipv6(address);
    let _result = host.to_owned();
}

#[test]
fn test_to_owned_ipv6_address_valid() {
    let address = Ipv6Addr::new(0x20, 0x01, 0x0dB8, 0x0000, 0x0000, 0x0000, 0x0000, 0x0001); // Represents 2001:db8::1
    let host = Host::Ipv6(address);
    let _result = host.to_owned();
}

#[test]
fn test_to_owned_ipv6_address_full_bits() {
    let address = Ipv6Addr::new(0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF); // Represents an all 1s address
    let host = Host::Ipv6(address);
    let _result = host.to_owned();
}

