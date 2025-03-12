// Answer 0

#[test]
fn test_visit_post_with_empty() {
    let writer = Writer { wtr: String::new() };
    let mut visitor = writer;
    let hir = Hir {
        kind: HirKind::Empty,
        props: Properties::default(),
    };
    visitor.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_with_literal() {
    let writer = Writer { wtr: String::new() };
    let mut visitor = writer;
    let literal = Literal::new("test");
    let hir = Hir {
        kind: HirKind::Literal(literal),
        props: Properties::default(),
    };
    visitor.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_with_class() {
    let writer = Writer { wtr: String::new() };
    let mut visitor = writer;
    let class = Class::new(vec!['a', 'b', 'c']);
    let hir = Hir {
        kind: HirKind::Class(class),
        props: Properties::default(),
    };
    visitor.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_with_look() {
    let writer = Writer { wtr: String::new() };
    let mut visitor = writer;
    let look = Look::new("(?=test)");
    let hir = Hir {
        kind: HirKind::Look(look),
        props: Properties::default(),
    };
    visitor.visit_post(&hir).unwrap();
}

