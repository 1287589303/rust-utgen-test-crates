// Answer 0

#[test]
#[should_panic]
fn test_validate_ident_raw_self() {
    validate_ident_raw("self");
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
#[should_panic]
fn test_validate_ident_raw_invalid() {
    validate_ident_raw("invalid_ident");
}

