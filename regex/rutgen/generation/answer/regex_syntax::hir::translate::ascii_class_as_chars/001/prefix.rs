// Answer 0

#[test]
fn test_ascii_class_as_chars_alnum() {
    use crate::ast::ClassAsciiKind;
    let kind = ClassAsciiKind::Alnum;
    let _result: Vec<_> = ascii_class_as_chars(&kind).collect();
}

#[test]
fn test_ascii_class_as_chars_alpha() {
    use crate::ast::ClassAsciiKind;
    let kind = ClassAsciiKind::Alpha;
    let _result: Vec<_> = ascii_class_as_chars(&kind).collect();
}

#[test]
fn test_ascii_class_as_chars_ascii() {
    use crate::ast::ClassAsciiKind;
    let kind = ClassAsciiKind::Ascii;
    let _result: Vec<_> = ascii_class_as_chars(&kind).collect();
}

#[test]
fn test_ascii_class_as_chars_blank() {
    use crate::ast::ClassAsciiKind;
    let kind = ClassAsciiKind::Blank;
    let _result: Vec<_> = ascii_class_as_chars(&kind).collect();
}

#[test]
fn test_ascii_class_as_chars_cntrl() {
    use crate::ast::ClassAsciiKind;
    let kind = ClassAsciiKind::Cntrl;
    let _result: Vec<_> = ascii_class_as_chars(&kind).collect();
}

#[test]
fn test_ascii_class_as_chars_digit() {
    use crate::ast::ClassAsciiKind;
    let kind = ClassAsciiKind::Digit;
    let _result: Vec<_> = ascii_class_as_chars(&kind).collect();
}

#[test]
fn test_ascii_class_as_chars_graph() {
    use crate::ast::ClassAsciiKind;
    let kind = ClassAsciiKind::Graph;
    let _result: Vec<_> = ascii_class_as_chars(&kind).collect();
}

#[test]
fn test_ascii_class_as_chars_lower() {
    use crate::ast::ClassAsciiKind;
    let kind = ClassAsciiKind::Lower;
    let _result: Vec<_> = ascii_class_as_chars(&kind).collect();
}

#[test]
fn test_ascii_class_as_chars_print() {
    use crate::ast::ClassAsciiKind;
    let kind = ClassAsciiKind::Print;
    let _result: Vec<_> = ascii_class_as_chars(&kind).collect();
}

#[test]
fn test_ascii_class_as_chars_punct() {
    use crate::ast::ClassAsciiKind;
    let kind = ClassAsciiKind::Punct;
    let _result: Vec<_> = ascii_class_as_chars(&kind).collect();
}

#[test]
fn test_ascii_class_as_chars_space() {
    use crate::ast::ClassAsciiKind;
    let kind = ClassAsciiKind::Space;
    let _result: Vec<_> = ascii_class_as_chars(&kind).collect();
}

#[test]
fn test_ascii_class_as_chars_upper() {
    use crate::ast::ClassAsciiKind;
    let kind = ClassAsciiKind::Upper;
    let _result: Vec<_> = ascii_class_as_chars(&kind).collect();
}

#[test]
fn test_ascii_class_as_chars_word() {
    use crate::ast::ClassAsciiKind;
    let kind = ClassAsciiKind::Word;
    let _result: Vec<_> = ascii_class_as_chars(&kind).collect();
}

#[test]
fn test_ascii_class_as_chars_xdigit() {
    use crate::ast::ClassAsciiKind;
    let kind = ClassAsciiKind::Xdigit;
    let _result: Vec<_> = ascii_class_as_chars(&kind).collect();
}

