// Answer 0

#[test]
fn test_into_class_item_ranges_with_single_character_range() {
    let class_range = hir::ClassRange { start: 'a', end: 'b' };
    let hir = Hir {
        kind: HirKind::Class(hir::Class { ranges: vec![class_range] }),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let _ = into_class_item_ranges(hir);
}

#[test]
fn test_into_class_item_ranges_with_multiple_character_ranges() {
    let class_range1 = hir::ClassRange { start: 'a', end: 'c' };
    let class_range2 = hir::ClassRange { start: 'e', end: 'g' };
    let hir = Hir {
        kind: HirKind::Class(hir::Class { ranges: vec![class_range1, class_range2] }),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let _ = into_class_item_ranges(hir);
}

#[test]
fn test_into_class_item_ranges_with_overlapping_ranges() {
    let class_range1 = hir::ClassRange { start: 'a', end: 'd' };
    let class_range2 = hir::ClassRange { start: 'c', end: 'f' };
    let hir = Hir {
        kind: HirKind::Class(hir::Class { ranges: vec![class_range1, class_range2] }),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let _ = into_class_item_ranges(hir);
}

#[test]
fn test_into_class_item_ranges_with_single_character_range_equal() {
    let class_range = hir::ClassRange { start: 'x', end: 'x' };
    let hir = Hir {
        kind: HirKind::Class(hir::Class { ranges: vec![class_range] }),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let _ = into_class_item_ranges(hir);
}

#[test]
fn test_into_class_item_ranges_with_edge_case_range() {
    let class_range = hir::ClassRange { start: '1', end: '2' };
    let hir = Hir {
        kind: HirKind::Class(hir::Class { ranges: vec![class_range] }),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let _ = into_class_item_ranges(hir);
}

