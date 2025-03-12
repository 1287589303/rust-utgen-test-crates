// Answer 0

#[test]
fn test_into_kind_empty() {
    let hir = Hir { kind: HirKind::Empty, props: Properties(Box::new(PropertiesI {})) };
    let _kind = hir.into_kind();
}

#[test]
fn test_into_kind_literal() {
    let literal = Literal::new("test");
    let hir = Hir { kind: HirKind::Literal(literal), props: Properties(Box::new(PropertiesI {})) };
    let _kind = hir.into_kind();
}

#[test]
fn test_into_kind_class() {
    let class = Class::new(vec!['a', 'b', 'c']);
    let hir = Hir { kind: HirKind::Class(class), props: Properties(Box::new(PropertiesI {})) };
    let _kind = hir.into_kind();
}

#[test]
fn test_into_kind_look() {
    let look = Look::new("(?=test)");
    let hir = Hir { kind: HirKind::Look(look), props: Properties(Box::new(PropertiesI {})) };
    let _kind = hir.into_kind();
}

#[test]
fn test_into_kind_repetition() {
    let repetition = Repetition::new(Box::new(Hir { kind: HirKind::Literal(Literal::new("abc")), props: Properties(Box::new(PropertiesI {})) }), 1..=3);
    let hir = Hir { kind: HirKind::Repetition(repetition), props: Properties(Box::new(PropertiesI {})) };
    let _kind = hir.into_kind();
}

#[test]
fn test_into_kind_capture() {
    let capture = Capture::new(Box::new(Hir { kind: HirKind::Literal(Literal::new("abc")), props: Properties(Box::new(PropertiesI {})) }));
    let hir = Hir { kind: HirKind::Capture(capture), props: Properties(Box::new(PropertiesI {})) };
    let _kind = hir.into_kind();
}

#[test]
fn test_into_kind_concat() {
    let concat = vec![
        Hir { kind: HirKind::Literal(Literal::new("a")), props: Properties(Box::new(PropertiesI {})) },
        Hir { kind: HirKind::Literal(Literal::new("b")), props: Properties(Box::new(PropertiesI {})) }
    ];
    let hir = Hir { kind: HirKind::Concat(concat), props: Properties(Box::new(PropertiesI {})) };
    let _kind = hir.into_kind();
}

#[test]
fn test_into_kind_alternation() {
    let alternation = vec![
        Hir { kind: HirKind::Literal(Literal::new("a")), props: Properties(Box::new(PropertiesI {})) },
        Hir { kind: HirKind::Literal(Literal::new("b")), props: Properties(Box::new(PropertiesI {})) }
    ];
    let hir = Hir { kind: HirKind::Alternation(alternation), props: Properties(Box::new(PropertiesI {})) };
    let _kind = hir.into_kind();
}

