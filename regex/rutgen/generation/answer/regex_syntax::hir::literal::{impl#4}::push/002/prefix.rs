// Answer 0

#[test]
fn test_push_with_non_empty_literals_and_different_last_literal() {
    let mut seq = Seq {
        literals: Some(vec![Literal {
            span: Span::new(0, 1),
            kind: LiteralKind::Exact,
            c: 'a',
        }]),
    };
    let new_literal = Literal {
        span: Span::new(1, 2),
        kind: LiteralKind::Exact,
        c: 'b',
    };
    seq.push(new_literal);
}

#[test]
fn test_push_with_non_empty_literals_and_different_last_literal_multiple() {
    let mut seq = Seq {
        literals: Some(vec![
            Literal {
                span: Span::new(0, 1),
                kind: LiteralKind::Exact,
                c: 'a',
            },
            Literal {
                span: Span::new(1, 2),
                kind: LiteralKind::Exact,
                c: 'b',
            },
        ]),
    };
    let new_literal = Literal {
        span: Span::new(2, 3),
        kind: LiteralKind::Exact,
        c: 'c',
    };
    seq.push(new_literal);
}

#[test]
fn test_push_with_non_empty_literals_and_different_last_literal_empty_lit() {
    let mut seq = Seq {
        literals: Some(vec![Literal {
            span: Span::new(0, 1),
            kind: LiteralKind::Exact,
            c: 'a',
        }]),
    };
    let new_literal = Literal {
        span: Span::new(1, 2),
        kind: LiteralKind::Exact,
        c: 'b',
    };
    seq.push(new_literal);
}

#[test]
fn test_push_with_non_empty_literals_varied_literals() {
    let mut seq = Seq {
        literals: Some(vec![
            Literal {
                span: Span::new(0, 1),
                kind: LiteralKind::Exact,
                c: '1',
            },
            Literal {
                span: Span::new(1, 2),
                kind: LiteralKind::Exact,
                c: '2',
            },
        ]),
    };
    let new_literal = Literal {
        span: Span::new(2, 3),
        kind: LiteralKind::Exact,
        c: '3',
    };
    seq.push(new_literal);
}

