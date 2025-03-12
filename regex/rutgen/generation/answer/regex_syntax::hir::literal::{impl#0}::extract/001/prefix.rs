// Answer 0

#[test]
fn test_extract_alternation_literals() {
    let extractor = Extractor::new()
        .kind(ExtractKind::Suffix)
        .limit_class(10)
        .limit_repeat(5)
        .limit_literal_len(3)
        .limit_total(20);
    
    let literals = vec![
        Literal::exact(vec![b'a']),
        Literal::exact(vec![b'b', b'c']),
    ];
    
    let hir = Hir {
        kind: HirKind::Alternation(vec![
            Hir { kind: HirKind::Literal(literals[0].clone()), props: Properties::default() },
            Hir { kind: HirKind::Literal(literals[1].clone()), props: Properties::default() },
        ]),
        props: Properties::default(),
    };

    extractor.extract(&hir);
}

#[test]
fn test_extract_alternation_classes() {
    let extractor = Extractor::new()
        .kind(ExtractKind::Prefix)
        .limit_class(15)
        .limit_repeat(10)
        .limit_literal_len(5)
        .limit_total(30);
    
    let class_unicode = ClassUnicode {
        set: IntervalSet::new(vec![ClassUnicodeRange::new(0x61, 0x7A)]),
    };
    
    let class_bytes = ClassBytes {
        set: IntervalSet::new(vec![ClassBytesRange::new(0x61, 0x62)]),
    };
    
    let hir = Hir {
        kind: HirKind::Alternation(vec![
            Hir { kind: HirKind::Class(class_unicode.clone()), props: Properties::default() },
            Hir { kind: HirKind::Class(class_bytes.clone()), props: Properties::default() },
        ]),
        props: Properties::default(),
    };

    extractor.extract(&hir);
}

#[test]
fn test_extract_alternation_mixed() {
    let extractor = Extractor::new()
        .kind(ExtractKind::Prefix)
        .limit_class(10)
        .limit_repeat(5)
        .limit_literal_len(3)
        .limit_total(20);
    
    let literals = vec![
        Literal::exact(vec![b'x']),
        Literal::exact(vec![b'y', b'z']),
    ];
    
    let class_unicode = ClassUnicode {
        set: IntervalSet::new(vec![ClassUnicodeRange::new(0x41, 0x5A)]),
    };
    
    let hir = Hir {
        kind: HirKind::Alternation(vec![
            Hir { kind: HirKind::Literal(literals[0].clone()), props: Properties::default() },
            Hir { kind: HirKind::Class(class_unicode.clone()), props: Properties::default() },
            Hir { kind: HirKind::Literal(literals[1].clone()), props: Properties::default() },
        ]),
        props: Properties::default(),
    };

    extractor.extract(&hir);
}

