// Answer 0

#[test]
fn test_ascii_class_word() {
    use crate::ast::ClassAsciiKind;
    let kind = ClassAsciiKind::Word;
    let _result: Vec<(u8, u8)> = ascii_class(&kind).collect();
}

#[test]
fn test_ascii_class_word_boundary() {
    use crate::ast::ClassAsciiKind;
    let kind = ClassAsciiKind::Word;
    let _result: Vec<(u8, u8)> = ascii_class(&kind).collect();
}

