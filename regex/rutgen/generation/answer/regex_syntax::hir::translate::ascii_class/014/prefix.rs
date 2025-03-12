// Answer 0

#[test]
fn test_ascii_class_alnum() {
    use crate::ast::ClassAsciiKind;
    let kind = ClassAsciiKind::Alnum;
    let result = ascii_class(&kind).collect::<Vec<_>>();
}

#[test]
fn test_ascii_class_alpha() {
    use crate::ast::ClassAsciiKind;
    let kind = ClassAsciiKind::Alpha;
    let result = ascii_class(&kind).collect::<Vec<_>>();
}

#[test]
fn test_ascii_class_ascii() {
    use crate::ast::ClassAsciiKind;
    let kind = ClassAsciiKind::Ascii;
    let result = ascii_class(&kind).collect::<Vec<_>>();
}

#[test]
fn test_ascii_class_blank() {
    use crate::ast::ClassAsciiKind;
    let kind = ClassAsciiKind::Blank;
    let result = ascii_class(&kind).collect::<Vec<_>>();
}

#[test]
fn test_ascii_class_cntrl() {
    use crate::ast::ClassAsciiKind;
    let kind = ClassAsciiKind::Cntrl;
    let result = ascii_class(&kind).collect::<Vec<_>>();
}

#[test]
fn test_ascii_class_digit() {
    use crate::ast::ClassAsciiKind;
    let kind = ClassAsciiKind::Digit;
    let result = ascii_class(&kind).collect::<Vec<_>>();
}

#[test]
fn test_ascii_class_graph() {
    use crate::ast::ClassAsciiKind;
    let kind = ClassAsciiKind::Graph;
    let result = ascii_class(&kind).collect::<Vec<_>>();
}

#[test]
fn test_ascii_class_lower() {
    use crate::ast::ClassAsciiKind;
    let kind = ClassAsciiKind::Lower;
    let result = ascii_class(&kind).collect::<Vec<_>>();
}

#[test]
fn test_ascii_class_print() {
    use crate::ast::ClassAsciiKind;
    let kind = ClassAsciiKind::Print;
    let result = ascii_class(&kind).collect::<Vec<_>>();
}

#[test]
fn test_ascii_class_punct() {
    use crate::ast::ClassAsciiKind;
    let kind = ClassAsciiKind::Punct;
    let result = ascii_class(&kind).collect::<Vec<_>>();
}

#[test]
fn test_ascii_class_space() {
    use crate::ast::ClassAsciiKind;
    let kind = ClassAsciiKind::Space;
    let result = ascii_class(&kind).collect::<Vec<_>>();
}

#[test]
fn test_ascii_class_upper() {
    use crate::ast::ClassAsciiKind;
    let kind = ClassAsciiKind::Upper;
    let result = ascii_class(&kind).collect::<Vec<_>>();
}

#[test]
fn test_ascii_class_word() {
    use crate::ast::ClassAsciiKind;
    let kind = ClassAsciiKind::Word;
    let result = ascii_class(&kind).collect::<Vec<_>>();
}

#[test]
fn test_ascii_class_xdigit() {
    use crate::ast::ClassAsciiKind;
    let kind = ClassAsciiKind::Xdigit;
    let result = ascii_class(&kind).collect::<Vec<_>>();
}

