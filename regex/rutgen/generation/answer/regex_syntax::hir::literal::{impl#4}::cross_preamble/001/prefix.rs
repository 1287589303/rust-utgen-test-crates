// Answer 0

#[test]
fn test_cross_preamble_with_non_empty_literals() {
    let mut seq1 = Seq {
        literals: Some(vec![
            Literal {
                span: Span::new(0, 3),
                kind: LiteralKind::Exact,
                c: 'a'
            },
            Literal {
                span: Span::new(1, 4),
                kind: LiteralKind::Exact,
                c: 'b'
            },
        ]),
    };

    let mut seq2 = Seq {
        literals: Some(vec![
            Literal {
                span: Span::new(0, 3),
                kind: LiteralKind::Exact,
                c: 'c'
            },
            Literal {
                span: Span::new(1, 4),
                kind: LiteralKind::Exact,
                c: 'd'
            },
        ]),
    };

    seq1.cross_preamble(&mut seq2);
}

#[test]
fn test_cross_preamble_with_non_empty_literals_different_scenarios() {
    let mut seq1 = Seq {
        literals: Some(vec![
            Literal {
                span: Span::new(0, 2),
                kind: LiteralKind::Exact,
                c: 'x'
            },
        ]),
    };

    let mut seq2 = Seq {
        literals: Some(vec![
            Literal {
                span: Span::new(1, 3),
                kind: LiteralKind::Exact,
                c: 'y'
            },
            Literal {
                span: Span::new(0, 1),
                kind: LiteralKind::Exact,
                c: 'z'
            },
        ]),
    };

    seq1.cross_preamble(&mut seq2);
}

#[test]
fn test_cross_preamble_with_identical_literal() {
    let mut seq1 = Seq {
        literals: Some(vec![
            Literal {
                span: Span::new(0, 5),
                kind: LiteralKind::Exact,
                c: 'e'
            },
        ]),
    };

    let mut seq2 = Seq {
        literals: Some(vec![
            Literal {
                span: Span::new(0, 5),
                kind: LiteralKind::Exact,
                c: 'e'
            },
        ]),
    };

    seq1.cross_preamble(&mut seq2);
}

#[test]
fn test_cross_preamble_with_multiple_literals() {
    let mut seq1 = Seq {
        literals: Some(vec![
            Literal {
                span: Span::new(0, 1),
                kind: LiteralKind::Exact,
                c: 'a'
            },
            Literal {
                span: Span::new(2, 3),
                kind: LiteralKind::Exact,
                c: 'b'
            },
        ]),
    };

    let mut seq2 = Seq {
        literals: Some(vec![
            Literal {
                span: Span::new(0, 2),
                kind: LiteralKind::Exact,
                c: 'c'
            },
            Literal {
                span: Span::new(3, 4),
                kind: LiteralKind::Exact,
                c: 'd'
            },
        ]),
    };

    seq1.cross_preamble(&mut seq2);
}

