// Answer 0

#[test]
fn test_from_name_alpha() {
    let name = "alpha";
    let result = ClassAsciiKind::from_name(name);
}

#[test]
fn test_from_name_not_alnum() {
    let name = "alpha";
    let result = ClassAsciiKind::from_name(name);
}

