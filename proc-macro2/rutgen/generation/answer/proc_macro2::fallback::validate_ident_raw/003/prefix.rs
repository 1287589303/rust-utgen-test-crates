// Answer 0

#[test]
#[should_panic]
fn test_validate_ident_raw_self() {
    validate_ident_raw("self");
}

#[test]
#[should_panic]
fn test_validate_ident_raw_empty() {
    validate_ident_raw("");
}

#[test]
#[should_panic]
fn test_validate_ident_raw_numeric() {
    validate_ident_raw("123");
}

#[test]
#[should_panic]
fn test_validate_ident_raw_invalid_ident() {
    validate_ident_raw("123abc");
}

