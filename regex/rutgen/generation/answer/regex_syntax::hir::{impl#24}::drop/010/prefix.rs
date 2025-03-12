// Answer 0

#[test]
fn test_drop_with_empty_alternation() {
    let empty_hir = Hir {
        kind: HirKind::Alternation(vec![]),
        props: Properties(Box::new(PropertiesI {})),
    };
    // Calling the drop method
    let _ = empty_hir;
}

#[test]
fn test_drop_with_empty_concat() {
    let empty_hir = Hir {
        kind: HirKind::Concat(vec![]),
        props: Properties(Box::new(PropertiesI {})),
    };
    // Calling the drop method
    let _ = empty_hir;
}

