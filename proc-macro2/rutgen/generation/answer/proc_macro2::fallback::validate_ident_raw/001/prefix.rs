// Answer 0

#[test]
#[should_panic]
fn test_validate_ident_raw_empty() {
    validate_ident_raw("");
}

#[test]
#[should_panic]
fn test_validate_ident_raw_number() {
    validate_ident_raw("123");
}

#[test]
#[should_panic]
fn test_validate_ident_raw_raw_identifier_super() {
    validate_ident_raw("super");
}

#[test]
#[should_panic]
fn test_validate_ident_raw_raw_identifier_self() {
    validate_ident_raw("self");
}

#[test]
#[should_panic]
fn test_validate_ident_raw_raw_identifier_Self() {
    validate_ident_raw("Self");
}

#[test]
#[should_panic]
fn test_validate_ident_raw_raw_identifier_crate() {
    validate_ident_raw("crate");
}

#[test]
fn test_validate_ident_raw_valid_ident() {
    validate_ident_raw("valid_ident");
}

