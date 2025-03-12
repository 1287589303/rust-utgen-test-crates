// Answer 0

#[test]
#[should_panic(expected = "Ident is not allowed to be empty; use Option<Ident>")]
fn test_validate_ident_empty_string() {
    let input = "";
    validate_ident(input);
}

