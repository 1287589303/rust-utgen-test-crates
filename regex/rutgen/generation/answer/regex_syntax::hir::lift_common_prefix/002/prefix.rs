// Answer 0

#[test]
fn test_lift_common_prefix_empty() {
    let hirs: Vec<Hir> = vec![];
    let _ = lift_common_prefix(hirs);
}

#[test]
fn test_lift_common_prefix_single_element() {
    let hirs = vec![Hir { kind: HirKind::Empty, props: Properties {} }];
    let _ = lift_common_prefix(hirs);
}

#[test]
fn test_lift_common_prefix_literal() {
    let hirs = vec![Hir { kind: HirKind::Literal(Literal::from("a")), props: Properties {} }];
    let _ = lift_common_prefix(hirs);
}

#[test]
fn test_lift_common_prefix_single_concats() {
    let h1 = Hir { kind: HirKind::Concat(vec![]), props: Properties {} };
    let h2 = Hir { kind: HirKind::Concat(vec![h1.clone()]), props: Properties {} };
    let hirs = vec![h1, h2];
    let _ = lift_common_prefix(hirs);
}

#[test]
fn test_lift_common_prefix_alternation_with_non_concat() {
    let h1 = Hir { kind: HirKind::Literal(Literal::from("abc")), props: Properties {} };
    let h2 = Hir { kind: HirKind::Concat(vec![Hir { kind: HirKind::Literal(Literal::from("xyz")), props: Properties {} }]), props: Properties {} };
    let hirs = vec![h1, h2];
    let _ = lift_common_prefix(hirs);
}

