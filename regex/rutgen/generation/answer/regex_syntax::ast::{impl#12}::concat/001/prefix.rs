// Answer 0

#[test]
fn test_concat_single_empty_ast() {
    let span = Span {
        start: Position(0),
        end: Position(1),
    };
    let asts: Vec<Ast> = vec![];
    let concat = Concat { span, asts };
    let result = Ast::concat(concat);
}

#[test]
fn test_concat_single_ast() {
    let span = Span {
        start: Position(0),
        end: Position(2),
    };
    let asts: Vec<Ast> = vec![Ast::empty(span.clone())];
    let concat = Concat { span, asts };
    let result = Ast::concat(concat);
}

#[test]
fn test_concat_multiple_asts() {
    let span = Span {
        start: Position(0),
        end: Position(5),
    };
    let asts: Vec<Ast> = vec![Ast::empty(span.clone()), Ast::dot(span.clone())];
    let concat = Concat { span, asts };
    let result = Ast::concat(concat);
}

#[test]
fn test_concat_with_non_empty_span() {
    let span = Span {
        start: Position(1),
        end: Position(3),
    };
    let asts: Vec<Ast> = vec![Ast::literal(Literal(Box::new(b"a".to_vec())))];
    let concat = Concat { span, asts };
    let result = Ast::concat(concat);
}

#[test]
fn test_concat_empty_span() {
    let span = Span {
        start: Position(3),
        end: Position(3),
    };
    let asts: Vec<Ast> = vec![Ast::empty(span.clone())];
    let concat = Concat { span, asts };
    let result = Ast::concat(concat);
}

