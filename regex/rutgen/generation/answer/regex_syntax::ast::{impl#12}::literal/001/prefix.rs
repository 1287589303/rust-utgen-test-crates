// Answer 0

#[test]
fn test_literal_empty_bytes_exact() {
    let e = Literal {
        bytes: vec![],
        exact: true,
    };
    let _result = Ast::literal(e);
}

#[test]
fn test_literal_empty_bytes_non_exact() {
    let e = Literal {
        bytes: vec![],
        exact: false,
    };
    let _result = Ast::literal(e);
}

#[test]
fn test_literal_single_byte_exact() {
    let e = Literal {
        bytes: vec![b'a'],
        exact: true,
    };
    let _result = Ast::literal(e);
}

#[test]
fn test_literal_single_byte_non_exact() {
    let e = Literal {
        bytes: vec![b'a'],
        exact: false,
    };
    let _result = Ast::literal(e);
}

#[test]
fn test_literal_multiple_bytes_exact() {
    let e = Literal {
        bytes: vec![b'a', b'b', b'c'],
        exact: true,
    };
    let _result = Ast::literal(e);
}

#[test]
fn test_literal_multiple_bytes_non_exact() {
    let e = Literal {
        bytes: vec![b'a', b'b', b'c'],
        exact: false,
    };
    let _result = Ast::literal(e);
}

#[test]
fn test_literal_max_size_bytes_exact() {
    let e = Literal {
        bytes: (0..=255).map(|i| i as u8).collect(),
        exact: true,
    };
    let _result = Ast::literal(e);
}

#[test]
fn test_literal_max_size_bytes_non_exact() {
    let e = Literal {
        bytes: (0..=255).map(|i| i as u8).collect(),
        exact: false,
    };
    let _result = Ast::literal(e);
}

#[test]
fn test_literal_char_boundary() {
    let e = Literal {
        bytes: vec![b' '], 
        exact: true,
    };
    let _result = Ast::literal(e);
}

