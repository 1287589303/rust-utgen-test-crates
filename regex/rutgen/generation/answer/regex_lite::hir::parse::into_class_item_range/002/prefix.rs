// Answer 0

#[test]
fn test_into_class_item_range_lowercase_a() {
    let hir = Hir {
        kind: HirKind::Char('a'),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let _ = into_class_item_range(hir);
}

#[test]
fn test_into_class_item_range_uppercase_z() {
    let hir = Hir {
        kind: HirKind::Char('Z'),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let _ = into_class_item_range(hir);
}

#[test]
fn test_into_class_item_range_digit_0() {
    let hir = Hir {
        kind: HirKind::Char('0'),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let _ = into_class_item_range(hir);
}

#[test]
fn test_into_class_item_range_special_char() {
    let hir = Hir {
        kind: HirKind::Char('@'),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let _ = into_class_item_range(hir);
}

#[test]
fn test_into_class_item_range_special_char_hash() {
    let hir = Hir {
        kind: HirKind::Char('#'),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let _ = into_class_item_range(hir);
}

