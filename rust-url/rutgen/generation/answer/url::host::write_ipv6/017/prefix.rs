// Answer 0

#[test]
fn test_write_ipv6_zero_segments() {
    struct DummyFormatter;

    impl fmt::Write for DummyFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let addr = Ipv6Addr::from_segments([0, 0, 0, 0, 0, 0, 0, 0]);
    let mut formatter = DummyFormatter;
    write_ipv6(&addr, &mut formatter).unwrap();
}

#[test]
fn test_write_ipv6_all_zero_segments() {
    struct DummyFormatter;

    impl fmt::Write for DummyFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let addr = Ipv6Addr::from_segments([0, 0, 0, 0, 0, 0, 0, 0]);
    let mut formatter = DummyFormatter;
    write_ipv6(&addr, &mut formatter).unwrap();
}

