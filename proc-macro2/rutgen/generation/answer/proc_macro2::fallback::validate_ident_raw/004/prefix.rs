// Answer 0

#[test]
#[should_panic]
fn test_validate_ident_raw_self() {
    validate_ident_raw("Self");
}

#[test]
#[should_panic]
fn test_validate_ident_raw_crate() {
    validate_ident_raw("crate");
}

#[test]
#[should_panic]
fn test_validate_ident_raw_super() {
    validate_ident_raw("super");
}

#[test]
#[should_panic]
fn test_validate_ident_raw_underscore() {
    validate_ident_raw("_");
}

#[test]
fn test_validate_ident_raw_valid_ident() {
    validate_ident_raw("valid_ident");
}

#[test]
#[should_panic]
fn test_validate_ident_raw_empty() {
    validate_ident_raw("");
}

#[test]
#[should_panic]
fn test_validate_ident_raw_all_digits() {
    validate_ident_raw("12345");
}

#[test]
#[should_panic]
fn test_validate_ident_raw_invalid_start() {
    validate_ident_raw("123abc");
}

#[test]
fn test_validate_ident_raw_valid_case() {
    validate_ident_raw("valid_case");
}

