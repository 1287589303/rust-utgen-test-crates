// Answer 0

#[test]
fn test_subs_empty() {
    let empty_hir = Hir {
        kind: HirKind::Empty,
        props: Properties::default(),
    };
    let _ = empty_hir.subs();
}

#[test]
fn test_subs_literal() {
    let literal_hir = Hir {
        kind: HirKind::Literal(Literal(vec![b'a', b'b', b'c'].into_boxed_slice())),
        props: Properties::default(),
    };
    let _ = literal_hir.subs();
}

#[test]
fn test_subs_class() {
    let class_hir = Hir {
        kind: HirKind::Class(Class::Unicode(ClassUnicode::new(vec!['a', 'b', 'c']))),
        props: Properties::default(),
    };
    let _ = class_hir.subs();
}

#[test]
fn test_subs_look() {
    let look_hir = Hir {
        kind: HirKind::Look(Look::Start),
        props: Properties::default(),
    };
    let _ = look_hir.subs();
}

