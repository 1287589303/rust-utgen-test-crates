// Answer 0

#[test]
fn test_lift_common_prefix_multiple_concat() {
    let first_concat = Hir::concat(vec![
        Hir::literal("abc".to_string()),
        Hir::literal("123".to_string()),
    ]);
    let second_concat = Hir::concat(vec![
        Hir::literal("abc".to_string()),
        Hir::literal("456".to_string()),
    ]);

    let hirs = vec![
        Hir { kind: HirKind::Concat(vec![first_concat]), props: Properties::default() },
        Hir { kind: HirKind::Concat(vec![second_concat]), props: Properties::default() },
    ];
    
    let _result = lift_common_prefix(hirs);
}

#[test]
fn test_lift_common_prefix_with_empty_suffix() {
    let first_concat = Hir::concat(vec![
        Hir::literal("xyz".to_string()),
    ]);
    let second_concat = Hir::concat(vec![
        Hir::literal("xyz".to_string()),
        Hir::literal("boom".to_string()),
    ]);

    let hirs = vec![
        Hir { kind: HirKind::Concat(vec![first_concat]), props: Properties::default() },
        Hir { kind: HirKind::Concat(vec![second_concat]), props: Properties::default() },
    ];
    
    let _result = lift_common_prefix(hirs);
}

#[test]
fn test_lift_common_prefix_multiple_concats_different_suffixes() {
    let first_concat = Hir::concat(vec![
        Hir::literal("foo".to_string()),
        Hir::literal("bar".to_string()),
    ]);
    let second_concat = Hir::concat(vec![
        Hir::literal("foo".to_string()),
        Hir::literal("baz".to_string()),
    ]);

    let hirs = vec![
        Hir { kind: HirKind::Concat(vec![first_concat]), props: Properties::default() },
        Hir { kind: HirKind::Concat(vec![second_concat]), props: Properties::default() },
    ];
    
    let _result = lift_common_prefix(hirs);
}

