// Answer 0

#[test]
#[should_panic(expected = r#"Ident is not a valid Ident"#)]
fn test_validate_ident_invalid_start_character() {
    let string = "-test";
    validate_ident(string);
}

#[test]
#[should_panic(expected = r#"Ident is not a valid Ident"#)]
fn test_validate_ident_non_alphanumeric_characters() {
    let string = "!@#$";
    validate_ident(string);
}

#[test]
#[should_panic(expected = r#"Ident is not a valid Ident"#)]
fn test_validate_ident_multiple_invalid_characters() {
    let string = "123@#";
    validate_ident(string);
}

