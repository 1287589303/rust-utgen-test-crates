// Answer 0

#[test]
fn test_singleton_bytes_case_1() {
    struct Literal {
        bytes: Vec<u8>,
    }

    let hirs = vec![
        Hir {
            kind: HirKind::Literal(Literal { bytes: vec![1] }),
            props: Properties {},
        },
        Hir {
            kind: HirKind::Literal(Literal { bytes: vec![2] }),
            props: Properties {},
        },
        Hir {
            kind: HirKind::Literal(Literal { bytes: vec![3] }),
            props: Properties {},
        },
    ];

    let result = singleton_bytes(&hirs);
}

#[test]
fn test_singleton_bytes_case_2() {
    struct Literal {
        bytes: Vec<u8>,
    }

    let hirs = vec![
        Hir {
            kind: HirKind::Literal(Literal { bytes: vec![100] }),
            props: Properties {},
        },
        Hir {
            kind: HirKind::Literal(Literal { bytes: vec![101] }),
            props: Properties {},
        },
        Hir {
            kind: HirKind::Literal(Literal { bytes: vec![102] }),
            props: Properties {},
        },
    ];

    let result = singleton_bytes(&hirs);
}

#[test]
fn test_singleton_bytes_case_3() {
    struct Literal {
        bytes: Vec<u8>,
    }

    let hirs = vec![
        Hir {
            kind: HirKind::Literal(Literal { bytes: vec![255] }),
            props: Properties {},
        },
        Hir {
            kind: HirKind::Literal(Literal { bytes: vec![0] }),
            props: Properties {},
        },
    ];

    let result = singleton_bytes(&hirs);
}

#[test]
fn test_singleton_bytes_case_empty() {
    struct Literal {
        bytes: Vec<u8>,
    }

    let hirs: Vec<Hir> = vec![];

    let result = singleton_bytes(&hirs);
}

