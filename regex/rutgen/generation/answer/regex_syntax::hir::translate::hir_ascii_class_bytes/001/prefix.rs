// Answer 0

#[test]
fn test_hir_ascii_class_bytes_alnum() {
    let kind = ast::ClassAsciiKind::Alnum;
    let result = hir_ascii_class_bytes(&kind);
}

#[test]
fn test_hir_ascii_class_bytes_alpha() {
    let kind = ast::ClassAsciiKind::Alpha;
    let result = hir_ascii_class_bytes(&kind);
}

#[test]
fn test_hir_ascii_class_bytes_ascii() {
    let kind = ast::ClassAsciiKind::Ascii;
    let result = hir_ascii_class_bytes(&kind);
}

#[test]
fn test_hir_ascii_class_bytes_blank() {
    let kind = ast::ClassAsciiKind::Blank;
    let result = hir_ascii_class_bytes(&kind);
}

#[test]
fn test_hir_ascii_class_bytes_cntrl() {
    let kind = ast::ClassAsciiKind::Cntrl;
    let result = hir_ascii_class_bytes(&kind);
}

#[test]
fn test_hir_ascii_class_bytes_digit() {
    let kind = ast::ClassAsciiKind::Digit;
    let result = hir_ascii_class_bytes(&kind);
}

#[test]
fn test_hir_ascii_class_bytes_graph() {
    let kind = ast::ClassAsciiKind::Graph;
    let result = hir_ascii_class_bytes(&kind);
}

#[test]
fn test_hir_ascii_class_bytes_lower() {
    let kind = ast::ClassAsciiKind::Lower;
    let result = hir_ascii_class_bytes(&kind);
}

#[test]
fn test_hir_ascii_class_bytes_print() {
    let kind = ast::ClassAsciiKind::Print;
    let result = hir_ascii_class_bytes(&kind);
}

#[test]
fn test_hir_ascii_class_bytes_punct() {
    let kind = ast::ClassAsciiKind::Punct;
    let result = hir_ascii_class_bytes(&kind);
}

#[test]
fn test_hir_ascii_class_bytes_space() {
    let kind = ast::ClassAsciiKind::Space;
    let result = hir_ascii_class_bytes(&kind);
}

#[test]
fn test_hir_ascii_class_bytes_upper() {
    let kind = ast::ClassAsciiKind::Upper;
    let result = hir_ascii_class_bytes(&kind);
}

#[test]
fn test_hir_ascii_class_bytes_word() {
    let kind = ast::ClassAsciiKind::Word;
    let result = hir_ascii_class_bytes(&kind);
}

#[test]
fn test_hir_ascii_class_bytes_xdigit() {
    let kind = ast::ClassAsciiKind::Xdigit;
    let result = hir_ascii_class_bytes(&kind);
}

