// Answer 0

#[test]
fn test_singleton_bytes_with_non_literal_hir_kind() {
    struct Literal(Vec<u8>);
    let hirs = vec![
        Hir {
            kind: HirKind::Class(Class), // Non-literal kind
            props: Properties,
        },
        Hir {
            kind: HirKind::Repetition(Repetition), // Non-literal kind
            props: Properties,
        },
        Hir {
            kind: HirKind::Concat(vec![
                Hir {
                    kind: HirKind::Literal(Literal(vec![0x61])), // Literal kind
                    props: Properties,
                },
                Hir {
                    kind: HirKind::Look(Look), // Non-literal kind
                    props: Properties,
                },
            ]),
            props: Properties,
        },
    ];
    let result = singleton_bytes(&hirs);
}

#[test]
fn test_singleton_bytes_with_only_non_literal_kinds() {
    struct Literal(Vec<u8>);
    let hirs = vec![
        Hir {
            kind: HirKind::Look(Look), // Non-literal kind
            props: Properties,
        },
        Hir {
            kind: HirKind::Class(Class), // Non-literal kind
            props: Properties,
        },
        Hir {
            kind: HirKind::Capture(Capture), // Non-literal kind
            props: Properties,
        },
    ];
    let result = singleton_bytes(&hirs);
}

