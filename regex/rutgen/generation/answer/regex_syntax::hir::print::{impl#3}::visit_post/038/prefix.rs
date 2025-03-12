// Answer 0

#[test]
fn test_visit_post_repetition_min_1_max_none_greedy_true() {
    let mut buffer = String::new();
    let writer = Writer { wtr: &mut buffer };
    let repetition = Repetition { min: 1, max: None, greedy: true, sub: Box::new(Hir { kind: HirKind::Empty, props: Properties::default() }) };
    let hir = Hir { kind: HirKind::Repetition(repetition), props: Properties::default() };
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_repetition_min_1_max_none_greedy_false() {
    let mut buffer = String::new();
    let writer = Writer { wtr: &mut buffer };
    let repetition = Repetition { min: 1, max: None, greedy: false, sub: Box::new(Hir { kind: HirKind::Empty, props: Properties::default() }) };
    let hir = Hir { kind: HirKind::Repetition(repetition), props: Properties::default() };
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_repetition_min_1_max_1_greedy_true() {
    let mut buffer = String::new();
    let writer = Writer { wtr: &mut buffer };
    let repetition = Repetition { min: 1, max: Some(1), greedy: true, sub: Box::new(Hir { kind: HirKind::Empty, props: Properties::default() }) };
    let hir = Hir { kind: HirKind::Repetition(repetition), props: Properties::default() };
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_repetition_min_1_max_1_greedy_false() {
    let mut buffer = String::new();
    let writer = Writer { wtr: &mut buffer };
    let repetition = Repetition { min: 1, max: Some(1), greedy: false, sub: Box::new(Hir { kind: HirKind::Empty, props: Properties::default() }) };
    let hir = Hir { kind: HirKind::Repetition(repetition), props: Properties::default() };
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_repetition_min_0_max_1_greedy_true() {
    let mut buffer = String::new();
    let writer = Writer { wtr: &mut buffer };
    let repetition = Repetition { min: 0, max: Some(1), greedy: true, sub: Box::new(Hir { kind: HirKind::Empty, props: Properties::default() }) };
    let hir = Hir { kind: HirKind::Repetition(repetition), props: Properties::default() };
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_repetition_min_0_max_1_greedy_false() {
    let mut buffer = String::new();
    let writer = Writer { wtr: &mut buffer };
    let repetition = Repetition { min: 0, max: Some(1), greedy: false, sub: Box::new(Hir { kind: HirKind::Empty, props: Properties::default() }) };
    let hir = Hir { kind: HirKind::Repetition(repetition), props: Properties::default() };
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_repetition_min_0_max_none_greedy_true() {
    let mut buffer = String::new();
    let writer = Writer { wtr: &mut buffer };
    let repetition = Repetition { min: 0, max: None, greedy: true, sub: Box::new(Hir { kind: HirKind::Empty, props: Properties::default() }) };
    let hir = Hir { kind: HirKind::Repetition(repetition), props: Properties::default() };
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_repetition_min_0_max_none_greedy_false() {
    let mut buffer = String::new();
    let writer = Writer { wtr: &mut buffer };
    let repetition = Repetition { min: 0, max: None, greedy: false, sub: Box::new(Hir { kind: HirKind::Empty, props: Properties::default() }) };
    let hir = Hir { kind: HirKind::Repetition(repetition), props: Properties::default() };
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_repetition_min_2_max_3_greedy_true() {
    let mut buffer = String::new();
    let writer = Writer { wtr: &mut buffer };
    let repetition = Repetition { min: 2, max: Some(3), greedy: true, sub: Box::new(Hir { kind: HirKind::Empty, props: Properties::default() }) };
    let hir = Hir { kind: HirKind::Repetition(repetition), props: Properties::default() };
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_repetition_min_2_max_3_greedy_false() {
    let mut buffer = String::new();
    let writer = Writer { wtr: &mut buffer };
    let repetition = Repetition { min: 2, max: Some(3), greedy: false, sub: Box::new(Hir { kind: HirKind::Empty, props: Properties::default() }) };
    let hir = Hir { kind: HirKind::Repetition(repetition), props: Properties::default() };
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_repetition_min_3_max_none_greedy_true() {
    let mut buffer = String::new();
    let writer = Writer { wtr: &mut buffer };
    let repetition = Repetition { min: 3, max: None, greedy: true, sub: Box::new(Hir { kind: HirKind::Empty, props: Properties::default() }) };
    let hir = Hir { kind: HirKind::Repetition(repetition), props: Properties::default() };
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_repetition_min_3_max_none_greedy_false() {
    let mut buffer = String::new();
    let writer = Writer { wtr: &mut buffer };
    let repetition = Repetition { min: 3, max: None, greedy: false, sub: Box::new(Hir { kind: HirKind::Empty, props: Properties::default() }) };
    let hir = Hir { kind: HirKind::Repetition(repetition), props: Properties::default() };
    writer.visit_post(&hir).unwrap();
}

