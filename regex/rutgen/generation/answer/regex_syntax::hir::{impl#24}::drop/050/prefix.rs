// Answer 0

#[test]
fn test_drop_look_kind() {
    let look_hir = Hir {
        kind: HirKind::Look(/* initialize Look here */),
        props: Properties(/* initialize properties here */),
    };
    let _ = look_hir; // Call drop implicitly by moving the variable out of scope.
}

#[test]
fn test_drop_class_kind() {
    let class_hir = Hir {
        kind: HirKind::Class(/* initialize Class here */),
        props: Properties(/* initialize properties here */),
    };
    let _ = class_hir; // Call drop implicitly by moving the variable out of scope.
}

#[test]
fn test_drop_empty_kind() {
    let empty_hir = Hir {
        kind: HirKind::Empty,
        props: Properties(/* initialize properties here */),
    };
    let _ = empty_hir; // Call drop implicitly by moving the variable out of scope.
}

#[test]
fn test_drop_literal_kind() {
    let literal_hir = Hir {
        kind: HirKind::Literal(/* initialize Literal here */),
        props: Properties(/* initialize properties here */),
    };
    let _ = literal_hir; // Call drop implicitly by moving the variable out of scope.
}

