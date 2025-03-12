// Answer 0

#[test]
fn test_hirkind_subs_alternation_with_literals() {
    let subs = vec![
        Hir {
            kind: HirKind::Literal(Literal {
                bytes: vec![b'a', b'b', b'c'],
                exact: true,
            }),
            props: Properties::default(),
        },
        Hir {
            kind: HirKind::Literal(Literal {
                bytes: vec![b'd', b'e', b'f'],
                exact: true,
            }),
            props: Properties::default(),
        },
    ];
    let alternation = HirKind::Alternation(subs);
    alternation.subs();
}

#[test]
fn test_hirkind_subs_alternation_with_repetitions() {
    let subs = vec![
        Hir {
            kind: HirKind::Repetition(Repetition {
                min: 1,
                max: Some(3),
                greedy: true,
                sub: Box::new(Hir {
                    kind: HirKind::Literal(Literal {
                        bytes: vec![b'x', b'y'],
                        exact: false,
                    }),
                    props: Properties::default(),
                }),
            }),
            props: Properties::default(),
        },
        Hir {
            kind: HirKind::Repetition(Repetition {
                min: 0,
                max: None,
                greedy: false,
                sub: Box::new(Hir {
                    kind: HirKind::Class(Class::Unicode(ClassUnicode::new())),
                    props: Properties::default(),
                }),
            }),
            props: Properties::default(),
        },
    ];
    let alternation = HirKind::Alternation(subs);
    alternation.subs();
}

#[test]
fn test_hirkind_subs_alternation_with_mixed_subs() {
    let subs = vec![
        Hir {
            kind: HirKind::Literal(Literal {
                bytes: vec![b'1', b'2', b'3'],
                exact: true,
            }),
            props: Properties::default(),
        },
        Hir {
            kind: HirKind::Capture(Capture {
                index: 0,
                name: None,
                sub: Box::new(Hir {
                    kind: HirKind::Concat(vec![
                        Hir {
                            kind: HirKind::Literal(Literal {
                                bytes: vec![b'a', b'b'],
                                exact: true,
                            }),
                            props: Properties::default(),
                        },
                        Hir {
                            kind: HirKind::Class(Class::Bytes(ClassBytes::new())),
                            props: Properties::default(),
                        },
                    ]),
                    props: Properties::default(),
                }),
            }),
            props: Properties::default(),
        },
    ];
    let alternation = HirKind::Alternation(subs);
    alternation.subs();
}

#[test]
fn test_hirkind_subs_alternation_with_empty_vec() {
    let subs: Vec<Hir> = vec![];
    let alternation = HirKind::Alternation(subs);
    alternation.subs();
}

