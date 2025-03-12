// Answer 0

#[test]
fn test_unsupported_boolean() {
    let value = Unsupported::Boolean;
    let mut formatter = std::fmt::Formatter::new();
    let _ = value.fmt(&mut formatter);
}

#[test]
fn test_unsupported_integer() {
    let value = Unsupported::Integer;
    let mut formatter = std::fmt::Formatter::new();
    let _ = value.fmt(&mut formatter);
}

#[test]
fn test_unsupported_float() {
    let value = Unsupported::Float;
    let mut formatter = std::fmt::Formatter::new();
    let _ = value.fmt(&mut formatter);
}

#[test]
fn test_unsupported_char() {
    let value = Unsupported::Char;
    let mut formatter = std::fmt::Formatter::new();
    let _ = value.fmt(&mut formatter);
}

#[test]
fn test_unsupported_string() {
    let value = Unsupported::String;
    let mut formatter = std::fmt::Formatter::new();
    let _ = value.fmt(&mut formatter);
}

#[test]
fn test_unsupported_byte_array() {
    let value = Unsupported::ByteArray;
    let mut formatter = std::fmt::Formatter::new();
    let _ = value.fmt(&mut formatter);
}

#[test]
fn test_unsupported_optional() {
    let value = Unsupported::Optional;
    let mut formatter = std::fmt::Formatter::new();
    let _ = value.fmt(&mut formatter);
}

#[test]
fn test_unsupported_sequence() {
    let value = Unsupported::Sequence;
    let mut formatter = std::fmt::Formatter::new();
    let _ = value.fmt(&mut formatter);
}

#[test]
fn test_unsupported_tuple() {
    let value = Unsupported::Tuple;
    let mut formatter = std::fmt::Formatter::new();
    let _ = value.fmt(&mut formatter);
}

#[test]
fn test_unsupported_tuple_struct() {
    let value = Unsupported::TupleStruct;
    let mut formatter = std::fmt::Formatter::new();
    let _ = value.fmt(&mut formatter);
}

#[cfg(not(any(feature = "std", feature = "alloc")))]
#[test]
fn test_unsupported_enum() {
    let value = Unsupported::Enum;
    let mut formatter = std::fmt::Formatter::new();
    let _ = value.fmt(&mut formatter);
}

