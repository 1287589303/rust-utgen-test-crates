// Answer 0

#[test]
fn test_write_ipv6_non_zero_start() {
    struct MockFormatter {
        output: String,
    }

    impl fmt::Write for MockFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let segments = [0x0001, 0x0002, 0x0003, 0x0004, 0x0005, 0x0006, 0x0007, 0x0008];
    let addr = Ipv6Addr::new(segments);
    let mut formatter = MockFormatter { output: String::new() };

    let _ = write_ipv6(&addr, &mut formatter);
}

#[test]
fn test_write_ipv6_compress_not_at_position() {
    struct MockFormatter {
        output: String,
    }

    impl fmt::Write for MockFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let segments = [0x0000, 0x0001, 0x0002, 0x0003, 0x0004, 0x0005, 0x0006, 0x0008];
    let addr = Ipv6Addr::new(segments);
    let mut formatter = MockFormatter { output: String::new() };

    let _ = write_ipv6(&addr, &mut formatter);
}

#[test]
fn test_write_ipv6_invalid_segment() {
    struct MockFormatter {
        output: String,
    }

    impl fmt::Write for MockFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let segments = [0x0000, 0x0000, 0x0000, 0x0000, 0x0001, 0x0002, 0x0003, 0xFFFF];
    let addr = Ipv6Addr::new(segments);
    let mut formatter = MockFormatter { output: String::new() };

    let _ = write_ipv6(&addr, &mut formatter);
}

#[test]
fn test_write_ipv6_max_value_segments() {
    struct MockFormatter {
        output: String,
    }

    impl fmt::Write for MockFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let segments = [0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF];
    let addr = Ipv6Addr::new(segments);
    let mut formatter = MockFormatter { output: String::new() };

    let _ = write_ipv6(&addr, &mut formatter);
}

