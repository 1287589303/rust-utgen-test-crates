// Answer 0

#[test]
fn test_ascii_class_xdigit() {
    use crate::ast::ClassAsciiKind;
    
    let kind = ClassAsciiKind::Xdigit;
    let iterator = ascii_class(&kind);
    let result: Vec<(u8, u8)> = iterator.collect();
}

#[test]
fn test_ascii_class_xdigit_boundary() {
    use crate::ast::ClassAsciiKind;
    
    let kind = ClassAsciiKind::Xdigit;
    let iterator = ascii_class(&kind);
    let result: Vec<(u8, u8)> = iterator.collect();
}

