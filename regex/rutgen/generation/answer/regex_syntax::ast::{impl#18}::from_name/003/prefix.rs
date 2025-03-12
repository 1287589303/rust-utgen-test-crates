// Answer 0

#[test]
fn test_from_name_ascii() {
    let name = "ascii";
    let result = ClassAsciiKind::from_name(name);
}

#[test]
#[should_panic]
fn test_from_name_invalid_case_alnum() {
    let name = "ALNUM";
    let result = ClassAsciiKind::from_name(name);
}

#[test]
#[should_panic]
fn test_from_name_invalid_case_alpha() {
    let name = "ALPHA";
    let result = ClassAsciiKind::from_name(name);
}

