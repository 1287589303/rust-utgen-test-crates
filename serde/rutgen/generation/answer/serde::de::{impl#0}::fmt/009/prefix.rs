// Answer 0

#[test]
fn test_unexpected_newtype_struct() {
    use crate::Unexpected;

    let unexpected = Unexpected::NewtypeStruct;
    let mut formatter = std::fmt::Formatter::new();
    let _ = unexpected.fmt(&mut formatter);
}

#[test]
fn test_unexpected_newtype_struct_display() {
    use crate::Unexpected;

    let unexpected = Unexpected::NewtypeStruct;
    let mut formatter = std::fmt::Formatter::new();
    let _ = unexpected.fmt(&mut formatter);
}

