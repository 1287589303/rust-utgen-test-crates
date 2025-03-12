// Answer 0

#[test]
fn test_fmt_ipv6_full_representation() {
    struct MockFormatter;
    
    impl fmt::Write for MockFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let ipv6_address = Ipv6Addr::new(0x2001, 0x0db8, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0001);
    let host = Host::Ipv6(ipv6_address);
    let mut formatter = MockFormatter;

    let _ = host.fmt(&mut formatter);
}

#[test]
fn test_fmt_ipv6_compressed_representation() {
    struct MockFormatter;
    
    impl fmt::Write for MockFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let ipv6_address = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1); // ::1
    let host = Host::Ipv6(ipv6_address);
    let mut formatter = MockFormatter;

    let _ = host.fmt(&mut formatter);
}

#[test]
fn test_fmt_ipv6_unspecified_address() {
    struct MockFormatter;
    
    impl fmt::Write for MockFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let ipv6_address = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0); // ::
    let host = Host::Ipv6(ipv6_address);
    let mut formatter = MockFormatter;

    let _ = host.fmt(&mut formatter);
}

#[test]
fn test_fmt_ipv6_multiple_leading_zeros() {
    struct MockFormatter;
    
    impl fmt::Write for MockFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let ipv6_address = Ipv6Addr::new(0x0001, 0x0db8, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0001); // 2001:db8::1
    let host = Host::Ipv6(ipv6_address);
    let mut formatter = MockFormatter;

    let _ = host.fmt(&mut formatter);
}

