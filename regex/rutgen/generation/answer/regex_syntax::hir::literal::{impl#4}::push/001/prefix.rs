// Answer 0

#[test]
fn test_push_with_equivalent_literal() {
    let mut seq = Seq {
        literals: Some(vec![Literal {
            span: Span::default(),
            kind: LiteralKind::default(),
            c: 'a',
        }]),
    };

    let equivalent_literal = Literal {
        span: Span::default(),
        kind: LiteralKind::default(),
        c: 'a',
    };

    seq.push(equivalent_literal);
}

#[test]
fn test_push_with_multiple_literls_same_as_last() {
    let mut seq = Seq {
        literals: Some(vec![Literal {
            span: Span::default(),
            kind: LiteralKind::default(),
            c: 'b',
        }]),
    };

    let second_equivalent_literal = Literal {
        span: Span::default(),
        kind: LiteralKind::default(),
        c: 'b',
    };

    seq.push(second_equivalent_literal);
}

#[test]
fn test_push_with_empti_seq() {
    let mut seq = Seq {
        literals: Some(vec![]),
    };

    let new_literal = Literal {
        span: Span::default(),
        kind: LiteralKind::default(),
        c: 'c',
    };

    seq.push(new_literal);
}

