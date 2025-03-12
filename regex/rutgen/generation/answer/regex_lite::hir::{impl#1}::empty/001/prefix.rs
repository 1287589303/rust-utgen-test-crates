// Answer 0

#[test]
fn test_empty_hir() {
    let result = Hir::empty();
}

#[test]
fn test_empty_hir_kind() {
    let result = Hir::empty();
    let kind = result.kind();
}

#[test]
fn test_empty_hir_is_start_anchored() {
    let result = Hir::empty();
    let anchored = result.is_start_anchored();
}

#[test]
fn test_empty_hir_is_match_empty() {
    let result = Hir::empty();
    let match_empty = result.is_match_empty();
}

#[test]
fn test_empty_hir_static_explicit_captures_len() {
    let result = Hir::empty();
    let captures_len = result.static_explicit_captures_len();
}

