// Answer 0

#[test]
fn test_subs_empty() {
    let hir_kind = HirKind::Empty;
    let subs: &[Hir] = hir_kind.subs();
}

#[test]
fn test_subs_literal() {
    let literal = Literal(Box::new([b'a', b'b', b'c']));
    let hir_kind = HirKind::Literal(literal);
    let subs: &[Hir] = hir_kind.subs();
}

#[test]
fn test_subs_class() {
    let class = Class::Unicode(ClassUnicode::new(vec!['a', 'b', 'c']));
    let hir_kind = HirKind::Class(class);
    let subs: &[Hir] = hir_kind.subs();
}

#[test]
fn test_subs_look() {
    let look = Look::Start;
    let hir_kind = HirKind::Look(look);
    let subs: &[Hir] = hir_kind.subs();
}

