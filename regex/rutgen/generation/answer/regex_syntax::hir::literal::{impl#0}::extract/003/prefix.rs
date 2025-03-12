// Answer 0

#[test]
fn test_extract_concat_with_prefix_kind() {
    let mut extractor = Extractor::new()
        .kind(ExtractKind::Prefix)
        .limit_class(10)
        .limit_repeat(5)
        .limit_literal_len(15)
        .limit_total(100);

    let hir1 = Hir {
        kind: HirKind::Literal(Literal {
            bytes: b"abc".to_vec(),
            exact: true,
        }),
        props: Properties::default(),
    };
    
    let hir2 = Hir {
        kind: HirKind::Literal(Literal {
            bytes: b"def".to_vec(),
            exact: true,
        }),
        props: Properties::default(),
    };

    let hir_concat = Hir {
        kind: HirKind::Concat(vec![hir1.clone(), hir2.clone()]),
        props: Properties::default(),
    };

    let _seq = extractor.extract(&hir_concat);
}

#[test]
fn test_extract_concat_with_prefix_kind_empty_literal() {
    let mut extractor = Extractor::new()
        .kind(ExtractKind::Prefix)
        .limit_class(10)
        .limit_repeat(3)
        .limit_literal_len(5)
        .limit_total(50);

    let hir1 = Hir {
        kind: HirKind::Literal(Literal {
            bytes: b"".to_vec(),
            exact: true,
        }),
        props: Properties::default(),
    };

    let hir2 = Hir {
        kind: HirKind::Literal(Literal {
            bytes: b"ghi".to_vec(),
            exact: true,
        }),
        props: Properties::default(),
    };

    let hir_concat = Hir {
        kind: HirKind::Concat(vec![hir1.clone(), hir2.clone()]),
        props: Properties::default(),
    };

    let _seq = extractor.extract(&hir_concat);
}

#[test]
fn test_extract_concat_with_prefix_kind_multiple_literals() {
    let mut extractor = Extractor::new()
        .kind(ExtractKind::Prefix)
        .limit_class(20)
        .limit_repeat(8)
        .limit_literal_len(12)
        .limit_total(200);

    let hir1 = Hir {
        kind: HirKind::Literal(Literal {
            bytes: b"xyz".to_vec(),
            exact: true,
        }),
        props: Properties::default(),
    };

    let hir2 = Hir {
        kind: HirKind::Literal(Literal {
            bytes: b"uvw".to_vec(),
            exact: true,
        }),
        props: Properties::default(),
    };

    let hir3 = Hir {
        kind: HirKind::Literal(Literal {
            bytes: b"rst".to_vec(),
            exact: true,
        }),
        props: Properties::default(),
    };

    let hir_concat = Hir {
        kind: HirKind::Concat(vec![hir1.clone(), hir2.clone(), hir3.clone()]),
        props: Properties::default(),
    };

    let _seq = extractor.extract(&hir_concat);
}

