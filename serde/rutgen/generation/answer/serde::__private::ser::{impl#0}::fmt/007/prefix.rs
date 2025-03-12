// Answer 0

#[test]
fn test_unsupported_char() {
    use std::fmt::{self, Display};

    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let formatter = &mut TestFormatter;
    let unsupported_char = Unsupported::Char;

    unsupported_char.fmt(formatter).unwrap();
}

#[test]
fn test_unsupported_string() {
    use std::fmt::{self, Display};

    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let formatter = &mut TestFormatter;
    let unsupported_string = Unsupported::String;

    unsupported_string.fmt(formatter).unwrap();
} 

#[test]
fn test_unsupported_integer() {
    use std::fmt::{self, Display};

    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let formatter = &mut TestFormatter;
    let unsupported_integer = Unsupported::Integer;

    unsupported_integer.fmt(formatter).unwrap();
} 

#[test]
fn test_unsupported_float() {
    use std::fmt::{self, Display};

    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let formatter = &mut TestFormatter;
    let unsupported_float = Unsupported::Float;

    unsupported_float.fmt(formatter).unwrap();
} 

#[test]
fn test_unsupported_boolean() {
    use std::fmt::{self, Display};

    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let formatter = &mut TestFormatter;
    let unsupported_boolean = Unsupported::Boolean;

    unsupported_boolean.fmt(formatter).unwrap();
} 

#[test]
fn test_unsupported_sequence() {
    use std::fmt::{self, Display};

    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let formatter = &mut TestFormatter;
    let unsupported_sequence = Unsupported::Sequence;

    unsupported_sequence.fmt(formatter).unwrap();
} 

#[test]
fn test_unsupported_optional() {
    use std::fmt::{self, Display};

    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let formatter = &mut TestFormatter;
    let unsupported_optional = Unsupported::Optional;

    unsupported_optional.fmt(formatter).unwrap();
} 

#[test]
fn test_unsupported_tuple() {
    use std::fmt::{self, Display};

    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let formatter = &mut TestFormatter;
    let unsupported_tuple = Unsupported::Tuple;

    unsupported_tuple.fmt(formatter).unwrap();
} 

#[test]
fn test_unsupported_tuple_struct() {
    use std::fmt::{self, Display};

    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let formatter = &mut TestFormatter;
    let unsupported_tuple_struct = Unsupported::TupleStruct;

    unsupported_tuple_struct.fmt(formatter).unwrap();
} 

#[cfg(not(any(feature = "std", feature = "alloc")))]
#[test]
fn test_unsupported_enum() {
    use std::fmt::{self, Display};

    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let formatter = &mut TestFormatter;
    let unsupported_enum = Unsupported::Enum;

    unsupported_enum.fmt(formatter).unwrap();
}

