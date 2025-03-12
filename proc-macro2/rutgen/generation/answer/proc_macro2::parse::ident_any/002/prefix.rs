// Answer 0

#[test]
fn test_ident_any_with_raw_ident_super() {
    let input = Cursor { rest: "r##super" };
    let result = ident_any(input);
}

#[test]
fn test_ident_any_with_raw_ident_self() {
    let input = Cursor { rest: "r##self" };
    let result = ident_any(input);
}

#[test]
fn test_ident_any_with_raw_ident_Self() {
    let input = Cursor { rest: "r##Self" };
    let result = ident_any(input);
}

#[test]
fn test_ident_any_with_raw_ident_crate() {
    let input = Cursor { rest: "r##crate" };
    let result = ident_any(input);
}

#[test]
fn test_ident_any_with_raw_ident_underscore() {
    let input = Cursor { rest: "r##_" };
    let result = ident_any(input);
}

