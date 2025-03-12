// Answer 0

#[test]
fn test_hirkind_empty() {
    let hir_kind = HirKind::Empty;
    let _ = hir_kind.subs();
}

#[test]
fn test_hirkind_literal() {
    let literal = Literal(vec![b'a', b'b', b'c'].into_boxed_slice());
    let hir_kind = HirKind::Literal(literal);
    let _ = hir_kind.subs();
}

#[test]
fn test_hirkind_class() {
    let class = Class::Unicode(ClassUnicode::new(vec!['a', 'b', 'c'].into_iter().collect()));
    let hir_kind = HirKind::Class(class);
    let _ = hir_kind.subs();
}

#[test]
fn test_hirkind_look() {
    let look = Look::Start;
    let hir_kind = HirKind::Look(look);
    let _ = hir_kind.subs();
}

