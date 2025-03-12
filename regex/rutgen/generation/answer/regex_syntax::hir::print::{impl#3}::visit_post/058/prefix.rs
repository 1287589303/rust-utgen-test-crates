// Answer 0

#[test]
fn test_visit_post_repetition_min_zero_max_none_greedy_true() {
    let mut output = String::new();
    let writer = Writer { wtr: &mut output };
    let repetition = Repetition { min: 0, max: None, greedy: true, sub: Box::new(Hir { kind: HirKind::Empty, props: Properties::default() }) };
    let hir = Hir { kind: HirKind::Repetition(repetition), props: Properties::default() };
    let _ = writer.visit_post(&hir);
}

#[test]
fn test_visit_post_repetition_min_zero_max_none_greedy_false() {
    let mut output = String::new();
    let writer = Writer { wtr: &mut output };
    let repetition = Repetition { min: 0, max: None, greedy: false, sub: Box::new(Hir { kind: HirKind::Empty, props: Properties::default() }) };
    let hir = Hir { kind: HirKind::Repetition(repetition), props: Properties::default() };
    let _ = writer.visit_post(&hir);
}

#[test]
fn test_visit_post_repetition_min_zero_max_some_one() {
    let mut output = String::new();
    let writer = Writer { wtr: &mut output };
    let repetition = Repetition { min: 0, max: Some(1), greedy: true, sub: Box::new(Hir { kind: HirKind::Empty, props: Properties::default() }) };
    let hir = Hir { kind: HirKind::Repetition(repetition), props: Properties::default() };
    let _ = writer.visit_post(&hir);
}

