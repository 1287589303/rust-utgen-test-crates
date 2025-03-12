// Answer 0

#[test]
fn test_class_chars_with_non_class_kind() {
    let hirs: Vec<Hir> = vec![
        Hir {
            kind: HirKind::Empty,
            props: Properties {},
        },
        Hir {
            kind: HirKind::Literal(Literal::new(b"test")),
            props: Properties {},
        },
        Hir {
            kind: HirKind::Look(Look::new()),
            props: Properties {},
        },
        Hir {
            kind: HirKind::Repetition(Repetition::new(Hir::empty(), 1, Some(2))),
            props: Properties {},
        },
    ];
    let result = class_chars(&hirs);
}

#[test]
fn test_class_chars_with_concatenation() {
    let hirs: Vec<Hir> = vec![
        Hir {
            kind: HirKind::Concat(vec![
                Hir {
                    kind: HirKind::Literal(Literal::new(b"abc")),
                    props: Properties {},
                },
                Hir {
                    kind: HirKind::Empty,
                    props: Properties {},
                },
            ]),
            props: Properties {},
        },
    ];
    let result = class_chars(&hirs);
}

#[test]
fn test_class_chars_with_capturing_group() {
    let hirs: Vec<Hir> = vec![
        Hir {
            kind: HirKind::Capture(Capture::new(Hir::empty())),
            props: Properties {},
        },
        Hir {
            kind: HirKind::Class(Class::Unicode(ClassUnicode::empty())),
            props: Properties {},
        },
    ];
    let result = class_chars(&hirs);
}

