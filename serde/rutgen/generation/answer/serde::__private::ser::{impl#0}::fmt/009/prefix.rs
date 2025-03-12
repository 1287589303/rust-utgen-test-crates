// Answer 0

#[test]
fn test_unsupported_boolean() {
    let unsupported = Unsupported::Boolean;
    let mut formatter = fmt::Formatter::new();
    let _ = unsupported.fmt(&mut formatter);
}

#[test]
fn test_unsupported_integer() {
    let unsupported = Unsupported::Integer;
    let mut formatter = fmt::Formatter::new();
    let _ = unsupported.fmt(&mut formatter);
}

#[test]
fn test_unsupported_float() {
    let unsupported = Unsupported::Float;
    let mut formatter = fmt::Formatter::new();
    let _ = unsupported.fmt(&mut formatter);
}

#[test]
fn test_unsupported_char() {
    let unsupported = Unsupported::Char;
    let mut formatter = fmt::Formatter::new();
    let _ = unsupported.fmt(&mut formatter);
}

#[test]
fn test_unsupported_string() {
    let unsupported = Unsupported::String;
    let mut formatter = fmt::Formatter::new();
    let _ = unsupported.fmt(&mut formatter);
}

#[test]
fn test_unsupported_byte_array() {
    let unsupported = Unsupported::ByteArray;
    let mut formatter = fmt::Formatter::new();
    let _ = unsupported.fmt(&mut formatter);
}

#[test]
fn test_unsupported_optional() {
    let unsupported = Unsupported::Optional;
    let mut formatter = fmt::Formatter::new();
    let _ = unsupported.fmt(&mut formatter);
}

#[test]
fn test_unsupported_sequence() {
    let unsupported = Unsupported::Sequence;
    let mut formatter = fmt::Formatter::new();
    let _ = unsupported.fmt(&mut formatter);
}

#[test]
fn test_unsupported_tuple() {
    let unsupported = Unsupported::Tuple;
    let mut formatter = fmt::Formatter::new();
    let _ = unsupported.fmt(&mut formatter);
}

#[test]
fn test_unsupported_tuple_struct() {
    let unsupported = Unsupported::TupleStruct;
    let mut formatter = fmt::Formatter::new();
    let _ = unsupported.fmt(&mut formatter);
}

#[cfg(not(any(feature = "std", feature = "alloc")))]
#[test]
fn test_unsupported_enum() {
    let unsupported = Unsupported::Enum;
    let mut formatter = fmt::Formatter::new();
    let _ = unsupported.fmt(&mut formatter);
}

