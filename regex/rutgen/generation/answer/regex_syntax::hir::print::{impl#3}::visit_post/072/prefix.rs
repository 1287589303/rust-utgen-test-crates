// Answer 0

#[test]
fn test_visit_post_literal() {
    let mut writer = String::new();
    let mut visitor = Writer { wtr: &mut writer };
    let hir = Hir {
        kind: HirKind::Literal(Literal::new("a")),
        props: Properties::default(),
    };
    visitor.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_empty() {
    let mut writer = String::new();
    let mut visitor = Writer { wtr: &mut writer };
    let hir = Hir {
        kind: HirKind::Empty,
        props: Properties::default(),
    };
    visitor.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_class() {
    let mut writer = String::new();
    let mut visitor = Writer { wtr: &mut writer };
    let hir = Hir {
        kind: HirKind::Class(Class::new(vec!['a', 'b', 'c'])),
        props: Properties::default(),
    };
    visitor.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_look() {
    let mut writer = String::new();
    let mut visitor = Writer { wtr: &mut writer };
    let hir = Hir {
        kind: HirKind::Look(Look::new()),
        props: Properties::default(),
    };
    visitor.visit_post(&hir).unwrap();
}

