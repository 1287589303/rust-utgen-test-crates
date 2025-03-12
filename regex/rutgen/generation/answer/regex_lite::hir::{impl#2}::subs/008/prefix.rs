// Answer 0

#[test]
fn test_subs_empty() {
    let hir_kind = HirKind::Empty;
    let result = hir_kind.subs();
}

#[test]
fn test_subs_char() {
    let hir_kind = HirKind::Char('a');
    let result = hir_kind.subs();
}

#[test]
fn test_subs_class() {
    let class = Class { ranges: vec![] };
    let hir_kind = HirKind::Class(class);
    let result = hir_kind.subs();
}

#[test]
fn test_subs_look() {
    let hir_kind = HirKind::Look(Look::Start);
    let result = hir_kind.subs();
}

