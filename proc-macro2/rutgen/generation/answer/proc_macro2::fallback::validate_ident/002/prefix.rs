// Answer 0

#[test]
#[should_panic]
fn test_validate_ident_empty_string() {
    validate_ident("");
}

#[test]
#[should_panic]
fn test_validate_ident_numeric_string_1() {
    validate_ident("1");
}

#[test]
#[should_panic]
fn test_validate_ident_numeric_string_123() {
    validate_ident("123");
}

#[test]
#[should_panic]
fn test_validate_ident_numeric_string_999() {
    validate_ident("999");
}

