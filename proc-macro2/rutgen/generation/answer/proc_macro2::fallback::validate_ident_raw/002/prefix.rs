// Answer 0

#[test]
#[should_panic(expected = "r#\"super\" cannot be a raw identifier")]
fn test_validate_ident_raw_super() {
    validate_ident_raw("super");
}

#[test]
#[should_panic(expected = "r#\"Self\" cannot be a raw identifier")]
fn test_validate_ident_raw_self() {
    validate_ident_raw("Self");
}

#[test]
#[should_panic(expected = "r#\"crate\" cannot be a raw identifier")]
fn test_validate_ident_raw_crate() {
    validate_ident_raw("crate");
}

#[test]
#[should_panic(expected = "r#\"_\" cannot be a raw identifier")]
fn test_validate_ident_raw_underscore() {
    validate_ident_raw("_");
}

#[test]
#[should_panic(expected = "r#\"self\" cannot be a raw identifier")]
fn test_validate_ident_raw_self_lower() {
    validate_ident_raw("self");
}

