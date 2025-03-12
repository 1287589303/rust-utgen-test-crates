// Answer 0

#[test]
fn test_empty_hir() {
    let result = Hir::empty();
}

#[test]
fn test_empty_hir_properties() {
    let result = Hir::empty();
    let expected_kind = HirKind::Empty;
    let expected_props = Properties::empty();
}

