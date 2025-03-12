// Answer 0

#[test]
fn test_ascii_class_alnum() {
    let kind = ast::ClassAsciiKind::Alnum;
    let _ = ascii_class(&kind).collect::<Vec<(u8, u8)>>();
}

#[test]
fn test_ascii_class_alpha() {
    let kind = ast::ClassAsciiKind::Alpha;
    let _ = ascii_class(&kind).collect::<Vec<(u8, u8)>>();
}

#[test]
fn test_ascii_class_ascii() {
    let kind = ast::ClassAsciiKind::Ascii;
    let _ = ascii_class(&kind).collect::<Vec<(u8, u8)>>();
}

#[test]
fn test_ascii_class_blank() {
    let kind = ast::ClassAsciiKind::Blank;
    let _ = ascii_class(&kind).collect::<Vec<(u8, u8)>>();
}

#[test]
fn test_ascii_class_cntrl() {
    let kind = ast::ClassAsciiKind::Cntrl;
    let _ = ascii_class(&kind).collect::<Vec<(u8, u8)>>();
}

#[test]
fn test_ascii_class_digit() {
    let kind = ast::ClassAsciiKind::Digit;
    let _ = ascii_class(&kind).collect::<Vec<(u8, u8)>>();
}

#[test]
fn test_ascii_class_graph() {
    let kind = ast::ClassAsciiKind::Graph;
    let _ = ascii_class(&kind).collect::<Vec<(u8, u8)>>();
}

#[test]
fn test_ascii_class_lower() {
    let kind = ast::ClassAsciiKind::Lower;
    let _ = ascii_class(&kind).collect::<Vec<(u8, u8)>>();
}

#[test]
fn test_ascii_class_print() {
    let kind = ast::ClassAsciiKind::Print;
    let _ = ascii_class(&kind).collect::<Vec<(u8, u8)>>();
}

#[test]
fn test_ascii_class_punct() {
    let kind = ast::ClassAsciiKind::Punct;
    let _ = ascii_class(&kind).collect::<Vec<(u8, u8)>>();
}

#[test]
fn test_ascii_class_space() {
    let kind = ast::ClassAsciiKind::Space;
    let _ = ascii_class(&kind).collect::<Vec<(u8, u8)>>();
}

#[test]
fn test_ascii_class_upper() {
    let kind = ast::ClassAsciiKind::Upper;
    let _ = ascii_class(&kind).collect::<Vec<(u8, u8)>>();
}

#[test]
fn test_ascii_class_word() {
    let kind = ast::ClassAsciiKind::Word;
    let _ = ascii_class(&kind).collect::<Vec<(u8, u8)>>();
}

#[test]
fn test_ascii_class_xdigit() {
    let kind = ast::ClassAsciiKind::Xdigit;
    let _ = ascii_class(&kind).collect::<Vec<(u8, u8)>>();
}

