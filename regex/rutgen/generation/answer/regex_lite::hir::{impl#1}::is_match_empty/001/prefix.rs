// Answer 0

#[test]
fn test_is_match_empty_for_empty_hir() {
    let hir = Hir {
        kind: HirKind::Empty,
        is_start_anchored: false,
        is_match_empty: true,
        static_explicit_captures_len: None,
    };
    hir.is_match_empty();
}

#[test]
fn test_is_match_empty_for_char_hir() {
    let hir = Hir {
        kind: HirKind::Char('a'),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    hir.is_match_empty();
}

#[test]
fn test_is_match_empty_for_class_hir() {
    let class = Class; // Assume Class is properly defined elsewhere
    let hir = Hir {
        kind: HirKind::Class(class),
        is_start_anchored: false,
        is_match_empty: true,
        static_explicit_captures_len: None,
    };
    hir.is_match_empty();
}

#[test]
fn test_is_match_empty_for_look_hir() {
    let look = Look; // Assume Look is properly defined elsewhere
    let hir = Hir {
        kind: HirKind::Look(look),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    hir.is_match_empty();
}

#[test]
fn test_is_match_empty_for_repetition_hir() {
    let rep = Repetition; // Assume Repetition is properly defined elsewhere
    let hir = Hir {
        kind: HirKind::Repetition(rep),
        is_start_anchored: false,
        is_match_empty: true,
        static_explicit_captures_len: None,
    };
    hir.is_match_empty();
}

#[test]
fn test_is_match_empty_for_capture_hir() {
    let cap = Capture; // Assume Capture is properly defined elsewhere
    let hir = Hir {
        kind: HirKind::Capture(cap),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    hir.is_match_empty();
}

#[test]
fn test_is_match_empty_for_concat_hir() {
    let sub_hirs = vec![
        Hir {
            kind: HirKind::Char('a'),
            is_start_anchored: false,
            is_match_empty: false,
            static_explicit_captures_len: None,
        },
        Hir {
            kind: HirKind::Empty,
            is_start_anchored: false,
            is_match_empty: true,
            static_explicit_captures_len: None,
        },
    ];
    let hir = Hir {
        kind: HirKind::Concat(sub_hirs),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    hir.is_match_empty();
}

#[test]
fn test_is_match_empty_for_alternation_hir() {
    let sub_hirs = vec![
        Hir {
            kind: HirKind::Char('a'),
            is_start_anchored: false,
            is_match_empty: false,
            static_explicit_captures_len: None,
        },
        Hir {
            kind: HirKind::Empty,
            is_start_anchored: false,
            is_match_empty: true,
            static_explicit_captures_len: None,
        },
    ];
    let hir = Hir {
        kind: HirKind::Alternation(sub_hirs),
        is_start_anchored: false,
        is_match_empty: true,
        static_explicit_captures_len: None,
    };
    hir.is_match_empty();
}

