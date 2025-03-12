// Answer 0

#[test]
fn test_validate_ident_raw_non_empty_valid_ident() {
    let valid_ident = "example";
    validate_ident_raw(valid_ident);
}

#[test]
fn test_validate_ident_raw_non_empty_valid_ident_with_numbers() {
    let valid_ident = "example123";
    validate_ident_raw(valid_ident);
}

#[test]
fn test_validate_ident_raw_non_empty_mixed_case_valid_ident() {
    let valid_ident = "Example";
    validate_ident_raw(valid_ident);
}

#[test]
fn test_validate_ident_raw_valid_ident_with_underscore() {
    let valid_ident = "example_valid";
    validate_ident_raw(valid_ident);
}

#[test]
fn test_validate_ident_raw_non_empty_valid_ident_with_special_characters() {
    let valid_ident = "example-abc";
    validate_ident_raw(valid_ident);
}

