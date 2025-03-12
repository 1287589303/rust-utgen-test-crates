// Answer 0

#[test]
fn test_into_class_item_ranges_with_valid_char() {
    let hir = Hir {
        kind: HirKind::Char('a'),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let _result = into_class_item_ranges(hir);
}

#[test]
fn test_into_class_item_ranges_with_digit_char() {
    let hir = Hir {
        kind: HirKind::Char('1'),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let _result = into_class_item_ranges(hir);
}

#[test]
fn test_into_class_item_ranges_with_special_char() {
    let hir = Hir {
        kind: HirKind::Char('#'),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let _result = into_class_item_ranges(hir);
}

