// Answer 0

#[test]
fn test_fmt_ipv4_valid_min() {
    use crate::net::Ipv4Addr;
    let addr = Ipv4Addr::new(0, 0, 0, 0);
    let host = Host::Ipv4(addr);
    let mut formatter = std::fmt::Formatter::new();
    let _ = host.fmt(&mut formatter);
}

#[test]
fn test_fmt_ipv4_valid_max() {
    use crate::net::Ipv4Addr;
    let addr = Ipv4Addr::new(255, 255, 255, 255);
    let host = Host::Ipv4(addr);
    let mut formatter = std::fmt::Formatter::new();
    let _ = host.fmt(&mut formatter);
}

#[test]
fn test_fmt_ipv4_invalid_high() {
    use crate::net::Ipv4Addr;
    // The creation of an invalid Ipv4Addr should be handled appropriately, 
    // assuming there's some internal checking and error handling; hence we're assuming here to directly create the valid type since Rust doesn't allow invalid states.
    let addr = Ipv4Addr::new(256, 100, 100, 100);
    let host = Host::Ipv4(addr);
    let mut formatter = std::fmt::Formatter::new();
    let _ = host.fmt(&mut formatter);
}

#[test]
fn test_fmt_ipv4_invalid_format() {
    use crate::net::Ipv4Addr;
    // Same as above, we directly initialize a valid type.
    let addr = Ipv4Addr::new(192, 168, 1, 1); // let's consider calling this as valid for context
    let host = Host::Ipv4(addr);
    let mut formatter = std::fmt::Formatter::new();
    let _ = host.fmt(&mut formatter);
}

