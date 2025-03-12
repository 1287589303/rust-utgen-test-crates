// Answer 0

#[test]
fn test_write_ipv6_single_zero_sequence() {
    struct MockFormatter {
        should_fail: bool,
    }

    impl fmt::Write for MockFormatter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            if self.should_fail {
                return Err(fmt::Error);
            }
            Ok(())
        }
    }

    let addr = Ipv6Addr::from_segments(0, 0, 0, 0, 1, 2, 3, 4);
    let mut f = MockFormatter { should_fail: true };
    let _ = write_ipv6(&addr, &mut f);
}

#[test]
fn test_write_ipv6_multiple_zero_sequences() {
    struct MockFormatter {
        should_fail: bool,
    }

    impl fmt::Write for MockFormatter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            if self.should_fail {
                return Err(fmt::Error);
            }
            Ok(())
        }
    }

    let addr = Ipv6Addr::from_segments(0, 0, 0, 1, 2, 0, 0, 3);
    let mut f = MockFormatter { should_fail: true };
    let _ = write_ipv6(&addr, &mut f);
}

#[test]
fn test_write_ipv6_all_segments_zero() {
    struct MockFormatter {
        should_fail: bool,
    }

    impl fmt::Write for MockFormatter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            if self.should_fail {
                return Err(fmt::Error);
            }
            Ok(())
        }
    }

    let addr = Ipv6Addr::from_segments(0, 0, 0, 0, 0, 0, 0, 0);
    let mut f = MockFormatter { should_fail: true };
    let _ = write_ipv6(&addr, &mut f);
}

#[test]
fn test_write_ipv6_fully_populated() {
    struct MockFormatter {
        should_fail: bool,
    }

    impl fmt::Write for MockFormatter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            if self.should_fail {
                return Err(fmt::Error);
            }
            Ok(())
        }
    }

    let addr = Ipv6Addr::from_segments(1, 2, 3, 4, 5, 6, 7, 8);
    let mut f = MockFormatter { should_fail: true };
    let _ = write_ipv6(&addr, &mut f);
}

#[test]
fn test_write_ipv6_max_compress() {
    struct MockFormatter {
        should_fail: bool,
    }

    impl fmt::Write for MockFormatter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            if self.should_fail {
                return Err(fmt::Error);
            }
            Ok(())
        }
    }

    let addr = Ipv6Addr::from_segments(0, 0, 0, 0, 0, 0, 0, 1);
    let mut f = MockFormatter { should_fail: true };
    let _ = write_ipv6(&addr, &mut f);
}

