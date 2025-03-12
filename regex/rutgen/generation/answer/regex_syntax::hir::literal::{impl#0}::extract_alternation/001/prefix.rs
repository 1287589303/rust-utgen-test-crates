// Answer 0

#[test]
fn test_extract_alternation_with_finite_seq_and_hir_true() {
    let mut extractor = Extractor::new()
        .limit_class(10)
        .limit_repeat(5)
        .limit_literal_len(7)
        .limit_total(100);

    let hir_elements: Vec<Hir> = vec![
        Hir { kind: HirKind::Literal(hir::Literal(vec![1, 2, 3])) },
        Hir { kind: HirKind::Literal(hir::Literal(vec![4, 5, 6])) },
    ];

    let seq = extractor.extract_alternation(hir_elements.iter());
}

#[test]
fn test_extract_alternation_with_non_empty_finite_seq() {
    let mut extractor = Extractor::new()
        .limit_class(50)
        .limit_repeat(10)
        .limit_literal_len(5)
        .limit_total(300);

    let hir_elements: Vec<Hir> = vec![
        Hir { kind: HirKind::Literal(hir::Literal(vec![7, 8])) },
        Hir { kind: HirKind::Class(hir::Class::Unicode(vec![9, 10])) },
        Hir { kind: HirKind::Literal(hir::Literal(vec![11])) },
    ];

    let seq = extractor.extract_alternation(hir_elements.iter());
}

#[test]
fn test_extract_alternation_with_varied_hir_elements() {
    let mut extractor = Extractor::new()
        .limit_class(20)
        .limit_repeat(3)
        .limit_literal_len(6)
        .limit_total(200);

    let hir_elements: Vec<Hir> = vec![
        Hir { kind: HirKind::Literal(hir::Literal(vec![12, 13, 14, 15])) },
        Hir { kind: HirKind::Repetition(hir::Repetition::new(2, None, HirKind::Literal(hir::Literal(vec![16])))) },
        Hir { kind: HirKind::Class(hir::Class::Bytes(vec![17, 18])) },
    ];

    let seq = extractor.extract_alternation(hir_elements.iter());
}

#[test]
fn test_extract_alternation_with_minimum_liter_length() {
    let mut extractor = Extractor::new()
        .limit_class(5)
        .limit_repeat(1)
        .limit_literal_len(1)
        .limit_total(50);

    let hir_elements: Vec<Hir> = vec![
        Hir { kind: HirKind::Literal(hir::Literal(vec![])) },
        Hir { kind: HirKind::Capture { sub: Box::new(Hir { kind: HirKind::Literal(hir::Literal(vec![1])) }) } },
    ];

    let seq = extractor.extract_alternation(hir_elements.iter());
}

#[test]
fn test_extract_alternation_with_maximum_finite_elements() {
    let mut extractor = Extractor::new()
        .limit_class(100)
        .limit_repeat(100)
        .limit_literal_len(10)
        .limit_total(1000);

    let hir_elements: Vec<Hir> = (0..10).map(|i| {
        Hir { kind: HirKind::Literal(hir::Literal(vec![i])) }
    }).collect();

    let seq = extractor.extract_alternation(hir_elements.iter());
}

