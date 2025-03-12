// Answer 0

#[test]
fn test_visit_post_empty() {
    let writer = String::new();
    let mut visitor = Writer { wtr: writer };
    let hir = Hir { kind: HirKind::Empty, props: Properties::new() };
    let _ = visitor.visit_post(&hir);
}

#[test]
fn test_visit_post_literal() {
    let writer = String::new();
    let mut visitor = Writer { wtr: writer };
    let literal = Literal::new(); // Assuming Literal::new() initializes a valid Literal
    let hir = Hir { kind: HirKind::Literal(literal), props: Properties::new() };
    let _ = visitor.visit_post(&hir);
}

#[test]
fn test_visit_post_class() {
    let writer = String::new();
    let mut visitor = Writer { wtr: writer };
    let class = Class::new(); // Assuming Class::new() initializes a valid Class
    let hir = Hir { kind: HirKind::Class(class), props: Properties::new() };
    let _ = visitor.visit_post(&hir);
}

#[test]
fn test_visit_post_look() {
    let writer = String::new();
    let mut visitor = Writer { wtr: writer };
    let look = Look::new(); // Assuming Look::new() initializes a valid Look
    let hir = Hir { kind: HirKind::Look(look), props: Properties::new() };
    let _ = visitor.visit_post(&hir);
}

