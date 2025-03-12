// Answer 0

#[test]
fn test_ident_with_r_prefix() {
    let input = Cursor { rest: "r\"hello\"" };
    let result = ident(input);
}

#[test]
fn test_ident_with_rsharp_prefix() {
    let input = Cursor { rest: "r#\"world\"" };
    let result = ident(input);
}

#[test]
fn test_ident_with_rsharpsharp_prefix() {
    let input = Cursor { rest: "r##\"example\"" };
    let result = ident(input);
}

#[test]
fn test_ident_with_b_prefix() {
    let input = Cursor { rest: "b\"byte\"" };
    let result = ident(input);
}

#[test]
fn test_ident_with_bapostrophe_prefix() {
    let input = Cursor { rest: "b\'character\'" };
    let result = ident(input);
}

#[test]
fn test_ident_with_br_prefix() {
    let input = Cursor { rest: "br\"raw byte string\"" };
    let result = ident(input);
}

#[test]
fn test_ident_with_brsharp_prefix() {
    let input = Cursor { rest: "br#\"raw byte string with sharp\"" };
    let result = ident(input);
}

#[test]
fn test_ident_with_c_prefix() {
    let input = Cursor { rest: "c\"char\"" };
    let result = ident(input);
}

#[test]
fn test_ident_with_cr_prefix() {
    let input = Cursor { rest: "cr\"character raw\"" };
    let result = ident(input);
}

#[test]
fn test_ident_with_crsharp_prefix() {
    let input = Cursor { rest: "cr#\"char raw sharp\"" };
    let result = ident(input);
}

