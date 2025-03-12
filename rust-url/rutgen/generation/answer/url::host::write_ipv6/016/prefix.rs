// Answer 0

#[test]
fn test_write_ipv6_no_zero_sequence() {
    struct TestIpv6Addr {
        segments: [u16; 8],
    }

    impl Ipv6Addr for TestIpv6Addr {
        fn segments(&self) -> &[u16; 8] {
            &self.segments
        }
    }

    let addr = TestIpv6Addr {
        segments: [1, 2, 3, 4, 5, 6, 7, 8],
    };
    let mut output = String::new();
    let mut formatter = Formatter::new(&mut output);
    let _ = write_ipv6(&addr, &mut formatter);
}

#[test]
fn test_write_ipv6_no_leading_zero() {
    struct TestIpv6Addr {
        segments: [u16; 8],
    }

    impl Ipv6Addr for TestIpv6Addr {
        fn segments(&self) -> &[u16; 8] {
            &self.segments
        }
    }

    let addr = TestIpv6Addr {
        segments: [0, 2, 3, 4, 5, 6, 7, 8],
    };
    let mut output = String::new();
    let mut formatter = Formatter::new(&mut output);
    let _ = write_ipv6(&addr, &mut formatter);
}

#[test]
fn test_write_ipv6_no_trailing_zero() {
    struct TestIpv6Addr {
        segments: [u16; 8],
    }

    impl Ipv6Addr for TestIpv6Addr {
        fn segments(&self) -> &[u16; 8] {
            &self.segments
        }
    }

    let addr = TestIpv6Addr {
        segments: [1, 2, 3, 4, 5, 6, 7, 0],
    };
    let mut output = String::new();
    let mut formatter = Formatter::new(&mut output);
    let _ = write_ipv6(&addr, &mut formatter);
}

#[test]
fn test_write_ipv6_all_non_zero() {
    struct TestIpv6Addr {
        segments: [u16; 8],
    }

    impl Ipv6Addr for TestIpv6Addr {
        fn segments(&self) -> &[u16; 8] {
            &self.segments
        }
    }

    let addr = TestIpv6Addr {
        segments: [1, 1, 1, 1, 1, 1, 1, 1],
    };
    let mut output = String::new();
    let mut formatter = Formatter::new(&mut output);
    let _ = write_ipv6(&addr, &mut formatter);
}

