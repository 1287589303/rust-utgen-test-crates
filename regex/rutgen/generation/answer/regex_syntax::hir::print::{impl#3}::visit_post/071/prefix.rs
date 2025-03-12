// Answer 0

#[test]
fn test_visit_post_with_empty_kind() {
    let writer = Writer { wtr: String::new() };
    let hir = Hir { kind: HirKind::Empty, props: Properties {} };
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_with_literal_kind() {
    let writer = Writer { wtr: String::new() };
    let hir = Hir { kind: HirKind::Literal(Literal::new("test")), props: Properties {} };
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_with_class_kind() {
    let writer = Writer { wtr: String::new() };
    let hir = Hir { kind: HirKind::Class(Class::new()), props: Properties {} };
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_with_look_kind() {
    let writer = Writer { wtr: String::new() };
    let hir = Hir { kind: HirKind::Look(Look::new()), props: Properties {} };
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_with_repetition_min_0_max_1_greedy() {
    let writer = Writer { wtr: String::new() };
    let repetition = Repetition { min: 0, max: Some(1), greedy: true, sub: Box::new(Hir { kind: HirKind::Literal(Literal::new("a")), props: Properties {} }) };
    let hir = Hir { kind: HirKind::Repetition(repetition), props: Properties {} };
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_with_repetition_min_0_max_none_greedy() {
    let writer = Writer { wtr: String::new() };
    let repetition = Repetition { min: 0, max: None, greedy: true, sub: Box::new(Hir { kind: HirKind::Literal(Literal::new("a")), props: Properties {} }) };
    let hir = Hir { kind: HirKind::Repetition(repetition), props: Properties {} };
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_with_repetition_min_1_max_none_greedy() {
    let writer = Writer { wtr: String::new() };
    let repetition = Repetition { min: 1, max: None, greedy: true, sub: Box::new(Hir { kind: HirKind::Literal(Literal::new("a")), props: Properties {} }) };
    let hir = Hir { kind: HirKind::Repetition(repetition), props: Properties {} };
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_with_repetition_min_1_max_1_greedy() {
    let writer = Writer { wtr: String::new() };
    let repetition = Repetition { min: 1, max: Some(1), greedy: true, sub: Box::new(Hir { kind: HirKind::Literal(Literal::new("a")), props: Properties {} }) };
    let hir = Hir { kind: HirKind::Repetition(repetition), props: Properties {} };
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_with_repetition_min_0_max_1_non_greedy() {
    let writer = Writer { wtr: String::new() };
    let repetition = Repetition { min: 0, max: Some(1), greedy: false, sub: Box::new(Hir { kind: HirKind::Literal(Literal::new("a")), props: Properties {} }) };
    let hir = Hir { kind: HirKind::Repetition(repetition), props: Properties {} };
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_with_repetition_min_0_max_none_non_greedy() {
    let writer = Writer { wtr: String::new() };
    let repetition = Repetition { min: 0, max: None, greedy: false, sub: Box::new(Hir { kind: HirKind::Literal(Literal::new("a")), props: Properties {} }) };
    let hir = Hir { kind: HirKind::Repetition(repetition), props: Properties {} };
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_with_repetition_min_1_max_none_non_greedy() {
    let writer = Writer { wtr: String::new() };
    let repetition = Repetition { min: 1, max: None, greedy: false, sub: Box::new(Hir { kind: HirKind::Literal(Literal::new("a")), props: Properties {} }) };
    let hir = Hir { kind: HirKind::Repetition(repetition), props: Properties {} };
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_with_repetition_min_1_max_1_non_greedy() {
    let writer = Writer { wtr: String::new() };
    let repetition = Repetition { min: 1, max: Some(1), greedy: false, sub: Box::new(Hir { kind: HirKind::Literal(Literal::new("a")), props: Properties {} }) };
    let hir = Hir { kind: HirKind::Repetition(repetition), props: Properties {} };
    writer.visit_post(&hir).unwrap();
}

