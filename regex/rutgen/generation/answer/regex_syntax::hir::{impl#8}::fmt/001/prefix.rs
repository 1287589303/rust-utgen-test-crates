// Answer 0

#[test]
fn test_fmt_empty() {
    let hir = Hir {
        kind: HirKind::Empty,
        props: Properties(Box::new(PropertiesI {})),
    };
    let mut buffer = String::new();
    let _ = write!(buffer, "{}", hir);
}

#[test]
fn test_fmt_literal() {
    let hir = Hir {
        kind: HirKind::Literal(Literal::new("test")),
        props: Properties(Box::new(PropertiesI {})),
    };
    let mut buffer = String::new();
    let _ = write!(buffer, "{}", hir);
}

#[test]
fn test_fmt_class() {
    let hir = Hir {
        kind: HirKind::Class(Class::new(vec!['a', 'b', 'c'])),
        props: Properties(Box::new(PropertiesI {})),
    };
    let mut buffer = String::new();
    let _ = write!(buffer, "{}", hir);
}

#[test]
fn test_fmt_look() {
    let hir = Hir {
        kind: HirKind::Look(Look::new()),
        props: Properties(Box::new(PropertiesI {})),
    };
    let mut buffer = String::new();
    let _ = write!(buffer, "{}", hir);
}

#[test]
fn test_fmt_repetition() {
    let hir = Hir {
        kind: HirKind::Repetition(Repetition::new(Box::new(Hir {
            kind: HirKind::Literal(Literal::new("inner")),
            props: Properties(Box::new(PropertiesI {})),
        }))),
        props: Properties(Box::new(PropertiesI {})),
    };
    let mut buffer = String::new();
    let _ = write!(buffer, "{}", hir);
}

#[test]
fn test_fmt_capture() {
    let hir = Hir {
        kind: HirKind::Capture(Capture::new(Box::new(Hir {
            kind: HirKind::Literal(Literal::new("group")),
            props: Properties(Box::new(PropertiesI {})),
        }))),
        props: Properties(Box::new(PropertiesI {})),
    };
    let mut buffer = String::new();
    let _ = write!(buffer, "{}", hir);
}

#[test]
fn test_fmt_concat() {
    let hir = Hir {
        kind: HirKind::Concat(vec![
            Hir {
                kind: HirKind::Literal(Literal::new("part1")),
                props: Properties(Box::new(PropertiesI {})),
            },
            Hir {
                kind: HirKind::Literal(Literal::new("part2")),
                props: Properties(Box::new(PropertiesI {})),
            },
        ]),
        props: Properties(Box::new(PropertiesI {})),
    };
    let mut buffer = String::new();
    let _ = write!(buffer, "{}", hir);
}

#[test]
fn test_fmt_alternation() {
    let hir = Hir {
        kind: HirKind::Alternation(vec![
            Hir {
                kind: HirKind::Literal(Literal::new("option1")),
                props: Properties(Box::new(PropertiesI {})),
            },
            Hir {
                kind: HirKind::Literal(Literal::new("option2")),
                props: Properties(Box::new(PropertiesI {})),
            },
        ]),
        props: Properties(Box::new(PropertiesI {})),
    };
    let mut buffer = String::new();
    let _ = write!(buffer, "{}", hir);
}

