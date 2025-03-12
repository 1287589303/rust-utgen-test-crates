// Answer 0

#[test]
fn test_hirkind_subs_concat_multiple_elements() {
    let sub1 = Hir {
        kind: HirKind::Literal(Literal(vec![b'a'])),
        props: Properties::default(),
    };
    let sub2 = Hir {
        kind: HirKind::Literal(Literal(vec![b'b'])),
        props: Properties::default(),
    };
    let subs = vec![sub1, sub2];

    let kind = HirKind::Concat(subs.clone());
    let hir = Hir {
        kind,
        props: Properties::default(),
    };
    
    let _ = hir.subs();
}

#[test]
fn test_hirkind_subs_concat_three_elements() {
    let sub1 = Hir {
        kind: HirKind::Literal(Literal(vec![b'c'])),
        props: Properties::default(),
    };
    let sub2 = Hir {
        kind: HirKind::Literal(Literal(vec![b'd'])),
        props: Properties::default(),
    };
    let sub3 = Hir {
        kind: HirKind::Literal(Literal(vec![b'e'])),
        props: Properties::default(),
    };
    let subs = vec![sub1, sub2, sub3];

    let kind = HirKind::Concat(subs.clone());
    let hir = Hir {
        kind,
        props: Properties::default(),
    };

    let _ = hir.subs();
}

#[test]
fn test_hirkind_subs_concat_empty_elements() {
    let sub1 = Hir {
        kind: HirKind::Literal(Literal(vec![b'f'])),
        props: Properties::default(),
    };
    let sub2 = Hir {
        kind: HirKind::Literal(Literal(vec![])),
        props: Properties::default(),
    };
    let subs = vec![sub1, sub2];

    let kind = HirKind::Concat(subs.clone());
    let hir = Hir {
        kind,
        props: Properties::default(),
    };

    let _ = hir.subs();
}

