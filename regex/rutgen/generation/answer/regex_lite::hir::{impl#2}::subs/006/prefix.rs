// Answer 0

#[test]
fn test_subs_empty() {
    let hir_kind = HirKind::Empty;
    let hir = Hir { kind: hir_kind, is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None };
    let _sub_expressions = hir.kind.subs();
}

#[test]
fn test_subs_char() {
    let hir_kind = HirKind::Char('a');
    let hir = Hir { kind: hir_kind, is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None };
    let _sub_expressions = hir.kind.subs();
}

#[test]
fn test_subs_class() {
    let class = Class { ranges: vec![] };
    let hir_kind = HirKind::Class(class);
    let hir = Hir { kind: hir_kind, is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None };
    let _sub_expressions = hir.kind.subs();
}

#[test]
fn test_subs_look() {
    let hir_kind = HirKind::Look(Look::Start);
    let hir = Hir { kind: hir_kind, is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None };
    let _sub_expressions = hir.kind.subs();
}

