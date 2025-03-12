// Answer 0

#[test]
fn test_ascii_class_upper() {
    use crate::ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Upper;
    let result: Vec<(u8, u8)> = ascii_class(&kind).collect();
}

#[test]
fn test_ascii_class_upper_boundary() {
    use crate::ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Upper;
    let result: Vec<(u8, u8)> = ascii_class(&kind).collect();
}

