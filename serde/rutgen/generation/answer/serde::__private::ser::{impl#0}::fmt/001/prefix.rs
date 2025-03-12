// Answer 0

#[test]
fn test_unsupported_tuple_struct() {
    use std::fmt;

    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let unsupported = Unsupported::TupleStruct;
    let mut formatter = TestFormatter;
    let _ = unsupported.fmt(&mut formatter);
}

#[test]
fn test_unsupported_boolean() {
    use std::fmt;

    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let unsupported = Unsupported::Boolean;
    let mut formatter = TestFormatter;
    let _ = unsupported.fmt(&mut formatter);
}

#[test]
fn test_unsupported_integer() {
    use std::fmt;

    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let unsupported = Unsupported::Integer;
    let mut formatter = TestFormatter;
    let _ = unsupported.fmt(&mut formatter);
}

#[test]
fn test_unsupported_float() {
    use std::fmt;

    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let unsupported = Unsupported::Float;
    let mut formatter = TestFormatter;
    let _ = unsupported.fmt(&mut formatter);
}

#[test]
fn test_unsupported_char() {
    use std::fmt;

    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let unsupported = Unsupported::Char;
    let mut formatter = TestFormatter;
    let _ = unsupported.fmt(&mut formatter);
}

#[test]
fn test_unsupported_string() {
    use std::fmt;

    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let unsupported = Unsupported::String;
    let mut formatter = TestFormatter;
    let _ = unsupported.fmt(&mut formatter);
}

#[test]
fn test_unsupported_byte_array() {
    use std::fmt;

    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let unsupported = Unsupported::ByteArray;
    let mut formatter = TestFormatter;
    let _ = unsupported.fmt(&mut formatter);
}

#[test]
fn test_unsupported_optional() {
    use std::fmt;

    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let unsupported = Unsupported::Optional;
    let mut formatter = TestFormatter;
    let _ = unsupported.fmt(&mut formatter);
}

#[test]
fn test_unsupported_sequence() {
    use std::fmt;

    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let unsupported = Unsupported::Sequence;
    let mut formatter = TestFormatter;
    let _ = unsupported.fmt(&mut formatter);
}

#[test]
fn test_unsupported_tuple() {
    use std::fmt;

    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let unsupported = Unsupported::Tuple;
    let mut formatter = TestFormatter;
    let _ = unsupported.fmt(&mut formatter);
}

