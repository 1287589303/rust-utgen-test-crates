// Answer 0

#[test]
fn test_union_non_empty_literals() {
    struct DummyLiteral {
        span: hir::Span,
        kind: hir::LiteralKind,
        c: char,
    }

    let lit1 = Literal(Box::new(b"hello".to_vec()));
    let lit2 = Literal(Box::new(b"world".to_vec()));
    let lit3 = Literal(Box::new(b"hello".to_vec())); // duplicate
    let lit4 = Literal(Box::new(b"rust".to_vec()));

    let mut seq1 = Seq::new(vec![lit1.clone(), lit2.clone()]);
    let mut seq2 = Seq::new(vec![lit3.clone(), lit4.clone()]);

    seq1.union(&mut seq2);
}

#[test]
fn test_union_with_exact_non_exact_literals() {
    struct DummyLiteral {
        span: hir::Span,
        kind: hir::LiteralKind,
        c: char,
    }

    let lit1 = Literal(Box::new(b"abc".to_vec())); // exact
    let lit2 = Literal(Box::new(b"def".to_vec())); // exact
    let lit3 = Literal(Box::new(b"abc".to_vec())); // duplicate
    let lit4 = Literal(Box::new(b"xyz".to_vec())); // non-exact

    let mut seq1 = Seq::new(vec![lit1.clone(), lit2.clone()]);
    let mut seq2 = Seq::new(vec![lit3.clone(), lit4.clone()]);

    seq1.union(&mut seq2);
}

#[test]
fn test_union_deduplication() {
    struct DummyLiteral {
        span: hir::Span,
        kind: hir::LiteralKind,
        c: char,
    }

    let lit1 = Literal(Box::new(b"foo".to_vec())); // exact
    let lit2 = Literal(Box::new(b"bar".to_vec())); // exact
    let lit3 = Literal(Box::new(b"foo".to_vec())); // duplicate, non-exact

    let mut seq1 = Seq::new(vec![lit1.clone(), lit2.clone()]);
    let mut seq2 = Seq::new(vec![lit3.clone()]);

    seq1.union(&mut seq2);
}

