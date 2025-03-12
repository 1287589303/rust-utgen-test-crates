// Answer 0

#[test]
fn test_drop_concat_empty() {
    use crate::Hir;
    use crate::HirKind;

    let hir_concat_empty = Hir {
        kind: HirKind::Concat(vec![]),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };

    let mut hir_instance = hir_concat_empty;
    std::mem::drop(hir_instance);
}

#[test]
fn test_drop_concat_empty_with_anchored() {
    use crate::Hir;
    use crate::HirKind;

    let hir_concat_empty = Hir {
        kind: HirKind::Concat(vec![]),
        is_start_anchored: true,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };

    let mut hir_instance = hir_concat_empty;
    std::mem::drop(hir_instance);
}

#[test]
fn test_drop_concat_empty_with_captures_len() {
    use crate::Hir;
    use crate::HirKind;

    let hir_concat_empty = Hir {
        kind: HirKind::Concat(vec![]),
        is_start_anchored: false,
        is_match_empty: true,
        static_explicit_captures_len: Some(0),
    };

    let mut hir_instance = hir_concat_empty;
    std::mem::drop(hir_instance);
}

