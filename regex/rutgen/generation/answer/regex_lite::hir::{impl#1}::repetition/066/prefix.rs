// Answer 0

#[test]
fn test_repetition_case_empty() {
    let sub_hir = Hir::empty();
    let rep = Repetition {
        min: 0,
        max: Some(0),
        greedy: true,
        sub: Box::new(sub_hir),
    };
    let result = Hir::repetition(rep);
}

#[test]
fn test_repetition_case_single() {
    let sub_hir = Hir::char('a');
    let rep = Repetition {
        min: 1,
        max: Some(1),
        greedy: true,
        sub: Box::new(sub_hir),
    };
    let result = Hir::repetition(rep);
}

#[test]
fn test_repetition_case_no_captures() {
    let sub_hir = Hir::empty();
    let rep = Repetition {
        min: 0,
        max: Some(2),
        greedy: false,
        sub: Box::new(sub_hir),
    };
    let result = Hir::repetition(rep);
}

