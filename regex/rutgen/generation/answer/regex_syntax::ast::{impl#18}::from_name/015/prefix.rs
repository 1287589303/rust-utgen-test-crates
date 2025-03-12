// Answer 0

#[test]
fn test_from_name_invalid_alnum() {
    let result = ClassAsciiKind::from_name("not_alnum");
}

#[test]
fn test_from_name_invalid_alpha() {
    let result = ClassAsciiKind::from_name("not_alpha");
}

#[test]
fn test_from_name_invalid_ascii() {
    let result = ClassAsciiKind::from_name("not_ascii");
}

#[test]
fn test_from_name_invalid_blank() {
    let result = ClassAsciiKind::from_name("not_blank");
}

#[test]
fn test_from_name_invalid_cntrl() {
    let result = ClassAsciiKind::from_name("not_cntrl");
}

#[test]
fn test_from_name_invalid_digit() {
    let result = ClassAsciiKind::from_name("not_digit");
}

#[test]
fn test_from_name_invalid_graph() {
    let result = ClassAsciiKind::from_name("not_graph");
}

#[test]
fn test_from_name_invalid_lower() {
    let result = ClassAsciiKind::from_name("not_lower");
}

#[test]
fn test_from_name_invalid_print() {
    let result = ClassAsciiKind::from_name("not_print");
}

#[test]
fn test_from_name_invalid_punct() {
    let result = ClassAsciiKind::from_name("not_punct");
}

#[test]
fn test_from_name_invalid_space() {
    let result = ClassAsciiKind::from_name("not_space");
}

#[test]
fn test_from_name_invalid_upper() {
    let result = ClassAsciiKind::from_name("not_upper");
}

#[test]
fn test_from_name_invalid_word() {
    let result = ClassAsciiKind::from_name("not_word");
}

#[test]
fn test_from_name_invalid_xdigit() {
    let result = ClassAsciiKind::from_name("not_xdigit");
}

