// Answer 0

#[test]
fn test_unsupported_tuple() {
    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let formatter = &mut TestFormatter;
    let unsupported = Unsupported::Tuple;
    unsupported.fmt(formatter).unwrap();
}

#[test]
fn test_unsupported_boolean() {
    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let formatter = &mut TestFormatter;
    let unsupported = Unsupported::Boolean;
    unsupported.fmt(formatter).unwrap();
}

#[test]
fn test_unsupported_integer() {
    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let formatter = &mut TestFormatter;
    let unsupported = Unsupported::Integer;
    unsupported.fmt(formatter).unwrap();
}

#[test]
fn test_unsupported_float() {
    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let formatter = &mut TestFormatter;
    let unsupported = Unsupported::Float;
    unsupported.fmt(formatter).unwrap();
}

#[test]
fn test_unsupported_char() {
    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let formatter = &mut TestFormatter;
    let unsupported = Unsupported::Char;
    unsupported.fmt(formatter).unwrap();
}

#[test]
fn test_unsupported_string() {
    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let formatter = &mut TestFormatter;
    let unsupported = Unsupported::String;
    unsupported.fmt(formatter).unwrap();
}

#[test]
fn test_unsupported_byte_array() {
    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let formatter = &mut TestFormatter;
    let unsupported = Unsupported::ByteArray;
    unsupported.fmt(formatter).unwrap();
}

#[test]
fn test_unsupported_optional() {
    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let formatter = &mut TestFormatter;
    let unsupported = Unsupported::Optional;
    unsupported.fmt(formatter).unwrap();
}

#[test]
fn test_unsupported_sequence() {
    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let formatter = &mut TestFormatter;
    let unsupported = Unsupported::Sequence;
    unsupported.fmt(formatter).unwrap();
}

#[test]
fn test_unsupported_tuple_struct() {
    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let formatter = &mut TestFormatter;
    let unsupported = Unsupported::TupleStruct;
    unsupported.fmt(formatter).unwrap();
}

#[cfg(not(any(feature = "std", feature = "alloc")))]
#[test]
fn test_unsupported_enum() {
    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let formatter = &mut TestFormatter;
    let unsupported = Unsupported::Enum;
    unsupported.fmt(formatter).unwrap();
}

