// Answer 0

#[test]
fn test_into_class_item_range_with_invalid_hir_kind_1() {
    struct InvalidHirKind;
    let hir = Hir {
        kind: HirKind::Group(Box::new(InvalidHirKind)),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let _ = into_class_item_range(hir);
}

#[test]
fn test_into_class_item_range_with_invalid_hir_kind_2() {
    struct InvalidHirKind;
    let hir = Hir {
        kind: HirKind::Concat(vec![]),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let _ = into_class_item_range(hir);
}

#[test]
fn test_into_class_item_range_with_invalid_hir_kind_3() {
    struct InvalidHirKind;
    let hir = Hir {
        kind: HirKind::Alt(vec![]),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let _ = into_class_item_range(hir);
}

#[test]
fn test_into_class_item_range_with_invalid_hir_kind_4() {
    struct InvalidHirKind;
    let hir = Hir {
        kind: HirKind::Repetition(Box::new(InvalidHirKind), None, None),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let _ = into_class_item_range(hir);
}

